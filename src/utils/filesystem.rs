use std::fs::metadata;

pub fn validate_file(file_path: &str) -> Result<(), String> {
    let meta = metadata(file_path).unwrap();

    if meta.is_file() {
        Ok(())
    } else if meta.is_dir() {
        Err(format!("{} is a directory", file_path))
    } else {
        Err(format!("{} is not a file or directory", file_path))
    }
}