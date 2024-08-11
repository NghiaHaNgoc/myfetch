use std::env;
use std::process::Command;

pub fn shell() -> Option<String> {
    let shell_path = match env::var("SHELL") {
        Ok(path) => path,
        Err(_) => return None,
    };
    let (name, version) = match shell_path {
        shell if shell.contains("bash") => ("bash", bash_version()),
        shell if shell.contains("zsh") => ("zsh", zsh_version()),
        shell if shell.contains("fish") => ("fish", fish_version()),
        _ => return None,
    };
    Some(format!("{} {}", name, version.unwrap_or(String::new())))
}

fn bash_version() -> Option<String> {
    let mut bash = Command::new("bash");
    bash.arg("--version");
    let bash_output = match bash.output() {
        Ok(it) => it,
        Err(_) => return None,
    }
    .stdout;
    let bash_output = match String::from_utf8(bash_output) {
        Ok(it) => it,
        Err(_) => return None,
    };
    let bash_version_line = bash_output.lines().next()?;
    let version = bash_version_line.split_whitespace().nth(3)?.to_string();
    Some(version)
}

fn zsh_version() -> Option<String> {
    let mut zsh = Command::new("zsh");
    zsh.arg("--version");
    let zsh_output = match zsh.output() {
        Ok(it) => it,
        Err(_) => return None,
    }
    .stdout;
    let zsh_output = match String::from_utf8(zsh_output) {
        Ok(it) => it,
        Err(_) => return None,
    };
    let zsh_version_line = zsh_output.lines().next()?;
    let version = zsh_version_line.split_whitespace().nth(1)?.to_string();
    Some(version)
}

fn fish_version() -> Option<String> {
    let mut fish = Command::new("fish");
    fish.arg("--version");
    let fish_output = match fish.output() {
        Ok(it) => it,
        Err(_) => return None,
    }
    .stdout;
    let fish_output = match String::from_utf8(fish_output) {
        Ok(it) => it,
        Err(_) => return None,
    };
    let fish_version_line = fish_output.lines().next()?;
    let version = fish_version_line.split_whitespace().nth(2)?.to_string();
    Some(version)
}
