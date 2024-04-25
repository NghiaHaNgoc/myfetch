use std::fs;

pub fn laptop() -> Option<String> {
    let content_file = match fs::read_to_string("/sys/devices/virtual/dmi/id/product_name") {
        Ok(content) => content,
        Err(_) => return None,
    };
    Some(content_file.trim().to_string())
}
