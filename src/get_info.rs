mod color;
mod cpu;
mod de;
mod disk;
mod display;
mod header;
mod host;
mod icon;
mod kernel;
mod logo;
mod memory;
mod os;
mod package;
mod shell;
mod temperature;
mod terminal;
mod theme;
mod uptime;
mod wm;

pub use color::colors;
pub use cpu::cpu;
pub use de::de;
pub use disk::disk;
pub use display::display;
pub use header::header;
pub use host::laptop;
pub use icon::icons;
pub use kernel::kernel;
pub use logo::logo;
pub use memory::memory;
pub use os::os;
pub use package::packages;
pub use shell::shell;
pub use temperature::temperature;
pub use terminal::terminal;
pub use theme::theme;
pub use uptime::uptime;
pub use wm::wm;

pub enum Info<'a> {
    Open,
    Close,
    Header,
    Break,
    Color(char, &'a str),
    Cpu(char, &'a str),
    De(char, &'a str),
    Disk(char, &'a str),
    Display(char, &'a str),
    Host(char, &'a str),
    Icon(char, &'a str),
    Kernel(char, &'a str),
    Memory(char, &'a str),
    Os(char, &'a str),
    Package(char, &'a str),
    Shell(char, &'a str),
    Terminal(char, &'a str),
    Temperature(char, &'a str),
    Theme(char, &'a str),
    Uptime(char, &'a str),
    Wm(char, &'a str),
}
