use std::fs;
use std::path::Path;

pub fn read_file(file: &str) -> String {
    let file = format!("src/input/{}", file);
    let file_path = Path::new(file.as_str());
    fs::read_to_string(file_path).expect("Failed to read file")
}
