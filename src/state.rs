use sysinfo::System;

pub struct AppState {
    pub sys: System,
}

impl AppState {
    pub fn new() -> Self {
        let mut sys = System::new_all();
        sys.refresh_all();
        Self { sys }
    }

    pub fn on_tick(&mut self) {
        self.sys.refresh_all();
    }
}
