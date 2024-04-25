use std::{env, process::Command};
const ENV_DE: [&str; 3] = [
    "DESKTOP_SESSION",
    "XDG_SESSION_DESKTOP",
    "XDG_CURRENT_DESKTOP",
];
pub fn de() -> Option<String> {
    let name = de_name()?;
    let version = match name.as_str() {
        "gnome" => gnome_version()?,
        _ => String::new(),
    };
    Some(format!("{} {}", name, version))
}
fn de_name() -> Option<String> {
    let mut de = None;

    for i in ENV_DE {
        if let Ok(x) = env::var(i) {
            de = Some(x.to_lowercase());
            break;
        }
    }
    de
}

fn gnome_version() -> Option<String> {
    let mut cmd = Command::new("gnome-shell");
    cmd.arg("--version");
    let stdout = match cmd.output() {
        Ok(it) => it,
        Err(_) => return None,
    }
    .stdout;
    let stdout = match String::from_utf8(stdout) {
        Ok(it) => it,
        Err(_) => return None,
    };
    let version = stdout.split_whitespace().nth(2)?.to_string();
    Some(version)
}
