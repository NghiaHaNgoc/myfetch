use std::process::Command;
use sysinfo::System;

pub fn packages() -> Option<String> {
    let name = match System::name() {
        Some(name) => name,
        None => return None,
    };
    let (name, num) = match name.as_str() {
        "Arch Linux" | "Manjaro" | "Garuda Linux" | "Artix Linux" => {
            ("pacman", pacman_num_packages())
        }
        "Debian" | "Ubuntu" => ("apt", apt_num_packages()),
         "NixOS" => ("nix", nix_num_packages()),
        _ => return None,
    };
    Some(format!("{} ({})", num.unwrap_or(0), name))
}

pub(super) fn pacman_num_packages() -> Option<usize> {
    let mut pacman = Command::new("pacman");
    pacman.arg("-Q");
    let pacman_output = match pacman.output() {
        Ok(it) => it,
        Err(_) => return None,
    }.stdout;
    let pacman_output = match String::from_utf8(pacman_output) {
        Ok(it) => it,
        Err(_) => return None,
    };
    let num = pacman_output.lines().count();
    Some(num)
}

fn apt_num_packages() -> Option<usize> {
    let mut apt = Command::new("apt");
    apt.args(["list", "--installed"]);
    let apt_output = match apt.output() {
        Ok(it) => it,
        Err(_) => return None,
    }.stdout;
    let apt_output = match String::from_utf8(apt_output) {
        Ok(it) => it,
        Err(_) => return None,
    };
    let num = apt_output.lines().count();
    Some(num)
}

fn nix_num_packages() -> Option<usize> {
    let mut apt = Command::new("nix-store");
    apt.args(["--query", "--requisites", "/run/current-system"]);
    let apt_output = match apt.output() {
        Ok(it) => it,
        Err(_) => return None,
    }.stdout;
    let apt_output = match String::from_utf8(apt_output) {
        Ok(it) => it,
        Err(_) => return None,
    };
    let num = apt_output.lines().count();
    Some(num)
}
