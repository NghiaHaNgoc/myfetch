use std::process::Command;

pub fn display() -> Option<String> {
    let mut cmd = Command::new("xrandr");
    let stdout = match cmd.output() {
        Ok(it) => it,
        Err(_) => return None,
    }
    .stdout;
    let stdout = match String::from_utf8(stdout) {
        Ok(it) => it,
        Err(_) => return None,
    };
    let mut info_line = None;
    for line in stdout.lines() {
        if line.contains("*+") {
            info_line = Some(line);
        }
    }
    let info_line = info_line?.trim().replace("*+", "");
    let mut info_display = info_line.split_whitespace();
    Some(format!(
        "{} ({}Hz)",
        info_display.next()?,
        info_display.next()?
    ))
}
