use std::fs::File;
use std::io::{BufRead, BufReader, Seek, SeekFrom};
use std::path::{Path, PathBuf};
use std::collections::HashMap;
use notify::{Watcher, RecursiveMode, EventKind, event::ModifyKind};
use tokio::sync::mpsc;
use walkdir::WalkDir;

use crate::models::LogEntry;

pub enum MonitorEvent {
    UsageUpdate { input_tokens: u64, output_tokens: u64 },
}

pub struct Monitor {
    watch_path: PathBuf,
    sender: mpsc::Sender<MonitorEvent>,
    file_readers: HashMap<PathBuf, BufReader<File>>,
}

impl Monitor {
    pub fn new(watch_path: PathBuf, sender: mpsc::Sender<MonitorEvent>) -> Self {
        Self {
            watch_path,
            sender,
            file_readers: HashMap::new(),
        }
    }

    pub async fn start(mut self) -> anyhow::Result<()> {
        let (tx, mut rx) = mpsc::channel(100);

        let mut watcher = notify::recommended_watcher(move |res| {
            if let Ok(event) = res {
                let _ = tx.blocking_send(event);
            }
        })?;

        // Catch up on existing files
        if self.watch_path.exists() {
            for entry in WalkDir::new(&self.watch_path).into_iter().filter_map(|e| e.ok()) {
                let path = entry.path().to_path_buf();
                if path.is_file() && path.extension().map_or(false, |ext| ext == "jsonl") {
                    self.track_file(path.clone());
                    // Seek to end to avoid reading all past history on startup
                    if let Some(reader) = self.file_readers.get_mut(&path) {
                        let _ = reader.seek(SeekFrom::End(0));
                    }
                }
            }
            watcher.watch(&self.watch_path, RecursiveMode::Recursive)?;
        }

        while let Some(event) = rx.recv().await {
            match event.kind {
                EventKind::Modify(ModifyKind::Data(_)) => {
                    for path in event.paths {
                        if path.extension().map_or(false, |ext| ext == "jsonl") {
                            if !self.file_readers.contains_key(&path) {
                                self.track_file(path.clone());
                            }
                            self.read_new_lines(&path).await;
                        }
                    }
                }
                EventKind::Create(_) => {
                    for path in event.paths {
                        if path.extension().map_or(false, |ext| ext == "jsonl") {
                            self.track_file(path.clone());
                            self.read_new_lines(&path).await;
                        }
                    }
                }
                _ => {}
            }
        }

        Ok(())
    }

    fn track_file(&mut self, path: PathBuf) {
        if let Ok(file) = File::open(&path) {
            let reader = BufReader::new(file);
            self.file_readers.insert(path, reader);
        }
    }

    async fn read_new_lines(&mut self, path: &Path) {
        let mut lines = Vec::new();
        if let Some(reader) = self.file_readers.get_mut(path) {
            let mut line = String::new();
            loop {
                line.clear();
                match reader.read_line(&mut line) {
                    Ok(0) => break, // EOF
                    Ok(_) => {
                        lines.push(line.clone());
                    }
                    Err(_) => break, // Error reading
                }
            }
        }

        for line in lines {
            self.process_line(&line).await;
        }
    }

    async fn process_line(&self, line: &str) {
        if let Ok(entry) = serde_json::from_str::<LogEntry>(line) {
            if entry.entry_type.as_deref() == Some("message") || entry.entry_type.as_deref() == Some("assistant") {
                if let Some(message) = entry.message {
                    if let Some(usage) = message.usage {
                        let input = usage.input_tokens.unwrap_or(0);
                        let output = usage.output_tokens.unwrap_or(0);
                        if input > 0 || output > 0 {
                            let _ = self.sender.send(MonitorEvent::UsageUpdate {
                                input_tokens: input,
                                output_tokens: output,
                            }).await;
                        }
                    }
                }
            }
        }
    }
}
