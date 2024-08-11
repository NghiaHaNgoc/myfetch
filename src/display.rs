use console::{strip_ansi_codes, Style};
use sysinfo::System;
use systemstat::platform::linux::PlatformImpl;
use systemstat::Platform;
const UNKNOWN: &str = "unknown";
use crate::util::{self, LogoLine};

use super::get_info;
use super::get_info::Info;

pub fn display_fetch(
    list_info: &[Info],
    header_style: Style,
    logo_style: Style,
    mut seed_num_color_info: u8,
) {
    sysinfo::set_open_files_limit(0);
    let mut system = System::new();
    let logo = get_info::logo().unwrap_or_default();
    let mut logo_iter = logo.lines();
    let logo_line_empty = format!("{:38}", ' ');
    let mut logo_line = LogoLine {
        content: "",
        style: logo_style,
        apply_style: false,
    };
    if let Some(name) = System::name() {
        if name != "NixOS" {
            logo_line.apply_style = true;
        }
    }
    let stat = PlatformImpl::new();
    let mut info_style = Style::new();

    for info in list_info {
        logo_line.content = match logo_iter.next() {
            Some(content) => content,

            None => logo_line_empty.as_str(),
        };
        info_style = info_style.fg(util::gen_color(seed_num_color_info));
        seed_num_color_info += 1;
        match info {
            Info::Break => display_break(&logo_line),
            Info::Close => display_close(&logo_line),
            Info::Color(icon, name) => {
                display_color(&logo_line, *icon, name, get_info::colors(), &info_style)
            }
            Info::Cpu(icon, name) => display_info(
                &logo_line,
                *icon,
                name,
                get_info::cpu(&mut system),
                &info_style,
            ),
            Info::De(icon, name) => {
                display_info(&logo_line, *icon, name, get_info::de(), &info_style)
            }
            Info::Disk(icon, name) => {
                display_info(&logo_line, *icon, name, get_info::disk(), &info_style)
            }
            Info::Display(icon, name) => {
                display_info(&logo_line, *icon, name, get_info::display(), &info_style)
            }
            Info::Header => display_header(&logo_line, get_info::header(), &header_style),
            Info::Host(icon, name) => {
                display_info(&logo_line, *icon, name, get_info::laptop(), &info_style)
            }
            Info::Icon(icon, name) => {
                display_info(&logo_line, *icon, name, get_info::icons(), &info_style)
            }
            Info::Kernel(icon, name) => {
                display_info(&logo_line, *icon, name, get_info::kernel(), &info_style)
            }
            Info::Memory(icon, name) => display_info(
                &logo_line,
                *icon,
                name,
                get_info::memory(&mut system),
                &info_style,
            ),
            Info::Open => display_open(&logo_line),
            Info::Os(icon, name) => {
                display_info(&logo_line, *icon, name, get_info::os(), &info_style)
            }
            Info::Package(icon, name) => {
                display_info(&logo_line, *icon, name, get_info::packages(), &info_style)
            }
            Info::Shell(icon, name) => {
                display_info(&logo_line, *icon, name, get_info::shell(), &info_style)
            }
            Info::Temperature(icon, name) => display_info(
                &logo_line,
                *icon,
                name,
                get_info::temperature(&stat),
                &info_style,
            ),
            Info::Terminal(icon, name) => {
                display_info(&logo_line, *icon, name, get_info::terminal(), &info_style)
            }
            Info::Theme(icon, name) => {
                display_info(&logo_line, *icon, name, get_info::theme(), &info_style)
            }
            Info::Uptime(icon, name) => {
                display_info(&logo_line, *icon, name, get_info::uptime(), &info_style)
            }
            Info::Wm(icon, name) => {
                display_info(&logo_line, *icon, name, get_info::wm(), &info_style)
            }
        }
    }
}

// Display info

fn display_header(logo_line: &LogoLine, header: Option<String>, header_style: &Style) {
    let header = match header {
        Some(ref head) => head,
        None => UNKNOWN,
    };
    let padding_len =
        util::LIMIT_LOGO_LENGTH.saturating_sub(strip_ansi_codes(logo_line.content).len());
    println!(
        "{}{:<padding_len$} {}",
        logo_line,
        "",
        header_style.apply_to(header)
    );
}

fn display_open(logo_line: &LogoLine) {
    let padding_len =
        util::LIMIT_LOGO_LENGTH.saturating_sub(strip_ansi_codes(logo_line.content).len());
    println!("{}{:<padding_len$}╭─────────────╮", logo_line, "");
}

fn display_close(logo_line: &LogoLine) {
    let padding_len =
        util::LIMIT_LOGO_LENGTH.saturating_sub(strip_ansi_codes(logo_line.content).len());
    println!("{}{:<padding_len$}╰─────────────╯", logo_line, "");
}

fn display_break(logo_line: &LogoLine) {
    let padding_len =
        util::LIMIT_LOGO_LENGTH.saturating_sub(strip_ansi_codes(logo_line.content).len());
    println!("{}{:<padding_len$}├─────────────┤", logo_line, "");
}

fn display_color(
    logo_line: &LogoLine,
    icon: char,
    name: &str,
    info: Option<String>,
    info_style: &Style,
) {
    let info = match info {
        Some(ref info) => info,
        None => UNKNOWN,
    };

    let padding_len =
        util::LIMIT_LOGO_LENGTH.saturating_sub(strip_ansi_codes(logo_line.content).len());
    println!(
        "{}{:<padding_len$}│{} {:11}│ {}",
        logo_line,
        "",
        info_style.apply_to(icon),
        console::style(name).bold(),
        info
    );
}
fn display_info(
    logo_line: &LogoLine,
    icon: char,
    name: &str,
    info: Option<String>,
    info_style: &Style,
) {
    let info = match info {
        Some(ref info) => info,
        None => UNKNOWN,
    };

    let padding_len =
        util::LIMIT_LOGO_LENGTH.saturating_sub(strip_ansi_codes(logo_line.content).len());
    println!(
        "{}{:<padding_len$}│{} {:11}│ {}",
        logo_line,
        "",
        info_style.apply_to(icon),
        console::style(name).bold(),
        info_style.apply_to(info)
    );
}
