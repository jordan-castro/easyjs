use core::str;

pub fn read_file(file_path: &str) -> String {
    // TODO: wasm file load.

    let file_contents = std::fs::read(file_path);
    if file_contents.is_err() {
        return "".to_string();
    }

    str::from_utf8(&file_contents.unwrap()).unwrap().to_string()
}

/// This is what is used in WASM and for URL files in general.
pub fn read_file_from_web(filr_uri: &str) -> String {
    String::new()
}