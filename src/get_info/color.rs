use console::style;

pub fn colors() -> Option<String> {
    Some(format!(
        "{}  {}  {}  {}  {}  {}  {}  {} ",
        style('').black(),
        style('').red(),
        style('').yellow(),
        style('').green(),
        style('').cyan(),
        style('').blue(),
        style('').magenta(),
        style('').white()
    ))
}
