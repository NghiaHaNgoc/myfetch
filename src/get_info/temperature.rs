use systemstat::{platform::linux::PlatformImpl, Platform};

pub fn temperature(stat: &PlatformImpl) -> Option<String> {
    match stat.cpu_temp() {
        Ok(temp) => Some(format!("{:.0}°C", temp)),
        Err(_) => None,
    }
}
