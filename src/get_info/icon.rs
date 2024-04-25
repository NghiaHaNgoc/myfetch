use std::process::Command;

pub fn icons() -> Option<String> {
    let mut cmd = Command::new("gsettings");
    cmd.args(["get", "org.gnome.desktop.interface", "icon-theme"]);
    let out = match cmd.output() {
        Ok(out) => out.stdout,
        Err(_) => return None,
    };
    let icon_name = match String::from_utf8(out) {
        Ok(icon_name) => icon_name,
        Err(_) => return None,
    };
    Some(String::from(icon_name.replace('\'', "").trim()))
}
