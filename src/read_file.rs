use std::io;
use std::fs;

pub fn read_file(file_path: &str) -> Result<String, io::Error> {
    let file_content = fs::read_to_string(file_path)?;
    Ok(file_content)
}
