use std::fs;


#[derive(Debug, Default)]
pub struct ArmData {
    pub gpu_freq_mhz: Option<u64>,
    pub gpu_load_percent: Option<u8>,
    pub temps: Vec<(String, f32)>,
}

pub fn read_sensors() -> ArmData {
    let mut data = ArmData::default();

    // Read Temperatures
    let mut temps = Vec::new();
    if let Ok(entries) = fs::read_dir("/sys/class/thermal") {
        for entry in entries.filter_map(Result::ok) {
            let path = entry.path();
            if let Some(name) = path.file_name().and_then(|n| n.to_str()) {
                if name.starts_with("thermal_zone") {
                    let type_path = path.join("type");
                    let temp_path = path.join("temp");

                    if let (Ok(t_str), Ok(temp_str)) = (fs::read_to_string(&type_path), fs::read_to_string(&temp_path)) {
                        let t_name = t_str.trim().to_string();
                        if let Ok(temp_milli) = temp_str.trim().parse::<f32>() {
                            temps.push((t_name, temp_milli / 1000.0));
                        }
                    }
                }
            }
        }
    }
    data.temps = temps;

    // Read GPU Freq and Load
    if let Ok(entries) = fs::read_dir("/sys/class/devfreq") {
        for entry in entries.filter_map(Result::ok) {
            let path = entry.path();
            let name = path.file_name().unwrap_or_default().to_string_lossy().to_lowercase();
            // Look for common GPU indicators
            if name.contains("gpu") || name.contains("mali") || name.contains("panfrost") || name.contains("lima") || name.contains("13040000.mali") {
                let freq_path = path.join("cur_freq");
                if let Ok(freq_str) = fs::read_to_string(&freq_path) {
                    if let Ok(freq_hz) = freq_str.trim().parse::<u64>() {
                        data.gpu_freq_mhz = Some(freq_hz / 1_000_000);
                    }
                }

                let load_path = path.join("load");
                if let Ok(load_str) = fs::read_to_string(&load_path) {
                    // Usually in format: "load@freq" or just usage
                    let load_part = load_str.split('@').next().unwrap_or("").trim();
                    if let Ok(load) = load_part.parse::<u8>() {
                        data.gpu_load_percent = Some(load);
                    }
                }
                break;
            }
        }
    }

    data
}
