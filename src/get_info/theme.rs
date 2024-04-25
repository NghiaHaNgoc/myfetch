use std::process::Command;

pub fn theme() -> Option<String> {
    let mut cmd = Command::new("gsettings");
    cmd.args(["get", "org.gnome.desktop.interface", "gtk-theme"]);
    let out = match cmd.output() {
        Ok(out) => out.stdout,
        Err(_) => return None,
    };
    let theme_name = match String::from_utf8(out) {
        Ok(theme_name) => theme_name,
        Err(_) => return None,
    };
    Some(String::from(theme_name.replace('\'', "").trim()))
}
