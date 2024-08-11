mod display;
mod get_info;
mod util;
use console::Style;
use get_info::Info;
use rand::Rng;
fn main() {
    let list = [
        Info::Header,
        Info::Open,
        Info::Os('', "OS"),
        Info::Host('󰪫', "Host"),
        Info::Kernel('', "Kernel"),
        Info::Uptime('󰔟', "Uptime"),
        Info::Package('', "Package"),
        Info::Shell('', "Shell"),
        Info::Display('󱣴', "Display"),
        Info::De('󰇄', "DE"),
        Info::Wm('󱂬', "WM"),
        Info::Theme('󰌁', "Theme"),
        Info::Icon('󰉦', "Icon"),
        Info::Terminal('', "Terminal"),
        Info::Temperature('󰔏', "Temperature"),
        Info::Cpu('', "CPU"),
        Info::Memory('󰍛', "Memory"),
        Info::Disk('󱛟', "Disk"),
        Info::Break,
        Info::Color('', "Color"),
        Info::Close,
    ];
    let mut random = rand::thread_rng();
    // random header color
    let rand_color_header = util::gen_color(random.gen_range(0..6));
    let style_header = Style::new().bold().fg(rand_color_header);
    // Random logo color
    let logo_color = util::gen_color(random.gen_range(0..6));
    let logo_style = Style::new().bold().fg(logo_color);
    // seed number color
    let seed_num_color_info = random.gen_range(0..6);

    display::display_fetch(&list, style_header, logo_style, seed_num_color_info);
}
