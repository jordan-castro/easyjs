use core::str;
use std::path::Path;
use std::io;
use std::fs;

pub fn read_file(file_path: &str) -> String {
    // TODO: wasm file load.

    let file_contents = std::fs::read(file_path);
    if file_contents.is_err() {
        return "".to_string();
    }

    str::from_utf8(&file_contents.unwrap()).unwrap().to_string()
}

/// Write a file
pub fn write_file(file_path: &str, contents: &str) {
    write_file_with_dirs(file_path, contents).expect(format!("Unable to write file: {}", file_path).as_str());
}

/// Write a file and any sub directories.
fn write_file_with_dirs<P: AsRef<Path>>(file_path: P, content: &str) -> io::Result<()> {
    let file_path = file_path.as_ref();

    // Extract the parent directory of the file
    if let Some(parent) = file_path.parent() {
        // Create all directories leading to the file if they don't exist
        fs::create_dir_all(parent)?;
    }

    // Write content to the file
    fs::write(file_path, content)?;

    Ok(())
}
