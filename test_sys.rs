use sysinfo::System;
fn main() {
    let mut sys = System::new_all();
    sys.refresh_all();
    println!("{}", sys.global_cpu_info().cpu_usage());
}
