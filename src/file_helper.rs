use std::fs::File;
use std::io::{Read, Write};
use std::path::Path;

pub fn get_file_name_without_extension(path: &str) -> Option<String> {
    let path = Path::new(path);
    path.file_stem().map(|s| s.to_string_lossy().into_owned())
}

pub fn read_file_to_string(path: &str) -> Result<String, std::io::Error> {
    let mut file = File::open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

pub fn write_string_to_file(path: &str, content: &str) -> Result<(), std::io::Error> {
    let mut file = File::create(path)?;
    file.write_all(content.as_bytes())?;
    Ok(())
}
