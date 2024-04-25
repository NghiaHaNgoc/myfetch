use sysinfo::System;

pub fn uptime() -> Option<String> {
    let time = System::uptime();
    let minutes = (time / 60) % 60;
    let hours = time / 3600;
    let str_minute = if minutes == 0 || minutes == 1 {
        format!("{} minute", minutes)
    } else {
        format!("{} minutes", minutes)
    };
    let str_hour = if hours == 0 {
        String::new()
    } else if hours == 1 {
        format!("{} hour, ", hours)
    } else {
        format!("{} hours, ", hours)
    };
    Some(format!("{}{}", str_hour, str_minute))
}
