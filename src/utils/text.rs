use std::{fs, io, path::Path};



pub fn read(path: &Path) -> Result<String, io::Error> {
    if path.is_file() {
        let data: Vec<u8> = fs::read(path)?;
        Ok(String::from_utf8(data).expect("Failed to convert bytes to UTF-8"))
    } else {
        Err(io::Error::new(io::ErrorKind::NotFound, "File not found"))
    }
}
