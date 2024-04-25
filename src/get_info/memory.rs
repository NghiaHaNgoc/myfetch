use sysinfo::System;

pub fn memory(system: &mut System) -> Option<String> {
    system.refresh_memory();
    let used = system.used_memory() as f32 / 1048576.0;
    let total = system.total_memory() as f32 / 1048576.0;
    let percent_used = (used / total) * 100.0;
    let used = if used < 1024.0 {
        format!("{:.2} MiB", used)
    } else {
        format!("{:.2} GiB", used / 1024.0)
    };
    let total = if total < 1024.0 {
        format!("{:.2} MiB", total)
    } else {
        format!("{:.2} GiB", total / 1024.0)
    };
    Some(format!("{} / {} ({:.0}%)", used, total, percent_used))
}
