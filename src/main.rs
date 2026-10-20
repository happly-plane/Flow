mod models;
mod state;
mod monitor;
mod ui;
mod big_text;

use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::CrosstermBackend,
    Terminal,
};
use std::path::PathBuf;
use std::{error::Error, io, time::Duration};
use tokio::sync::mpsc;
use tokio::time::interval;

use monitor::{Monitor, MonitorEvent};
use state::AppState;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // create app and run it
    let app = AppState::new();
    let res = run_app(&mut terminal, app).await;

    // restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        println!("{:?}", err)
    }

    Ok(())
}

async fn run_app(terminal: &mut Terminal<CrosstermBackend<io::Stdout>>, mut app: AppState) -> Result<(), Box<dyn Error>> {
    let home = std::env::var("HOME").unwrap_or_else(|_| String::from("~"));
    let mut watch_path = PathBuf::from(home);
    watch_path.push(".claude/projects");

    let (tx, mut rx) = mpsc::channel(100);
    let monitor = Monitor::new(watch_path, tx);

    tokio::spawn(async move {
        if let Err(e) = monitor.start().await {
            eprintln!("Monitor error: {:?}", e);
        }
    });

    let mut tick_rate = interval(Duration::from_millis(250));

    loop {
        terminal.draw(|f| ui::draw(f, &app))?;

        tokio::select! {
            _ = tick_rate.tick() => {
                app.tick();

                // Poll terminal events without blocking the async loop for too long
                if event::poll(Duration::from_millis(10))? {
                    if let Event::Key(key) = event::read()? {
                        if let KeyCode::Char('q') = key.code {
                            return Ok(());
                        }
                    }
                }
            }
            Some(event) = rx.recv() => {
                match event {
                    MonitorEvent::UsageUpdate { input_tokens, output_tokens } => {
                        app.add_usage(input_tokens, output_tokens);
                    }
                }
            }
        }
    }
}
