use std::env;
use sysinfo::System;

const SUPPORTED_WM: [&str; 3] = ["mutter", "kwin", "sway"];

pub fn wm() -> Option<String> {
    let mut system = System::new();
    system.refresh_processes();
    let processes = system.processes();
    let mut wm_name = None;
    let mut process_name: &str;
    for process in processes.values() {
        process_name = process.name().split(':').next()?;
        if SUPPORTED_WM.contains(&process_name) {
            wm_name = Some(process_name);
            break;
        }
    }
    let wm_name = wm_name?;
    match env::var("XDG_SESSION_TYPE") {
        Ok(wayland) => Some(format!("{} ({})", wm_name, wayland)),
        Err(_) => Some(wm_name.to_string()),
    }
}
