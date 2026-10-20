use std::collections::VecDeque;
use std::time::{Duration, Instant};

const HISTORY_SIZE: usize = 60; // 60 seconds

#[derive(Debug, Default)]
pub struct AppState {
    pub input_tokens_total: u64,
    pub output_tokens_total: u64,

    pub input_history: VecDeque<u64>,
    pub output_history: VecDeque<u64>,

    pub current_input_tokens: u64,
    pub current_output_tokens: u64,

    pub last_tick: Option<Instant>,
}

impl AppState {
    pub fn new() -> Self {
        let mut input_history = VecDeque::with_capacity(HISTORY_SIZE);
        let mut output_history = VecDeque::with_capacity(HISTORY_SIZE);
        for _ in 0..HISTORY_SIZE {
            input_history.push_back(0);
            output_history.push_back(0);
        }

        Self {
            input_tokens_total: 0,
            output_tokens_total: 0,
            input_history,
            output_history,
            current_input_tokens: 0,
            current_output_tokens: 0,
            last_tick: None,
        }
    }

    pub fn add_usage(&mut self, input: u64, output: u64) {
        self.input_tokens_total += input;
        self.output_tokens_total += output;

        self.current_input_tokens += input;
        self.current_output_tokens += output;
    }

    pub fn tick(&mut self) {
        let now = Instant::now();
        if let Some(last) = self.last_tick {
            if now.duration_since(last) >= Duration::from_secs(1) {
                // Shift history
                self.input_history.pop_front();
                self.input_history.push_back(self.current_input_tokens);

                self.output_history.pop_front();
                self.output_history.push_back(self.current_output_tokens);

                // Reset accumulators
                self.current_input_tokens = 0;
                self.current_output_tokens = 0;

                self.last_tick = Some(now);
            }
        } else {
            self.last_tick = Some(now);
        }
    }
}
