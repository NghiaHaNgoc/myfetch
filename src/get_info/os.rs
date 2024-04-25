use sysinfo::System;

pub fn os() -> Option<String> {
    let name = System::name();
    let version = System::os_version();
    let os_info = format!("{} {}", name?, version.unwrap_or_default());
    Some(os_info)
}
