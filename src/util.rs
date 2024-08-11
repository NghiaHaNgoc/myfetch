use std::fmt;

use console::{Color, Style};

pub const LIMIT_LOGO_LENGTH: usize = 43;
// Random color
pub fn gen_color(mut rand_num: u8) -> Color {
    rand_num %= 6;
    match rand_num {
        0 => Color::Red,
        1 => Color::Yellow,
        2 => Color::Green,
        3 => Color::Cyan,
        4 => Color::Blue,
        _ => Color::Magenta,
    }
}

pub struct LogoLine<'a> {
    pub content: &'a str,
    pub style: Style,
    pub apply_style: bool,
}

impl<'a> fmt::Display for LogoLine<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.apply_style {
            write!(f, "{}", self.style.apply_to(self.content))
        } else {
            write!(f, "{}", self.content)
        }
    }
}
