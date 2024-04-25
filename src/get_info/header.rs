use std::env;
use sysinfo::System;

pub fn header() -> Option<String> {
    let unknown = String::new();
    let user = match env::var("USER") {
        Ok(user) => user,
        Err(_) => unknown.clone(),
    };
    let hostname = match System::host_name() {
        Some(host) => host,
        None => unknown,
    };
    Some(format!("{}{}{}", hostname, " 󰁥 ", user))
}
