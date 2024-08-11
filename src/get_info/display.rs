use display_info::DisplayInfo;

pub fn display() -> Option<String> {
    let display_infos = DisplayInfo::all().ok()?;
    let display: Vec<String> = display_infos
        .into_iter()
        .map(
            |DisplayInfo {
                 width,
                 height,
                 frequency,
                 ..
             }| format!("{}x{} ({:.0}Hz)", width, height, frequency),
        )
        .collect();
    Some(display.join(", "))
}
