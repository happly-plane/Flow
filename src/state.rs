use sysinfo::System;
use crate::arm::{ArmData, read_sensors};

pub struct AppState {
    pub sys: System,
    pub arm_data: ArmData,
}

impl AppState {
    pub fn new() -> Self {
        let mut sys = System::new_all();
        sys.refresh_all();
        Self { sys, arm_data: read_sensors() }
    }

    pub fn on_tick(&mut self) {
        self.sys.refresh_all();
        self.arm_data = read_sensors();
    }
}
