/// Sanatize a file path.
pub fn sanitize_path(path: &str) -> String {
    // Step 1: Replace backslashes and forward slashes with underscores
    let path = path.replace(['/', '\\'], "_");

    // Step 2: Remove the extension (everything after the last '.')
    match path.rfind('.') {
        Some(idx) => path[..idx].to_string(),
        None => path,
    }
}

/// Returns the last segment of the path (after the last / or \) and removes the extension.
pub fn get_filename_without_extension(path: &str) -> String {
    // Step 1: Get the last segment after / or \
    let last_segment = path
        .rsplit(['/', '\\'])  // supports both / and \ separators
        .next()
        .unwrap_or(path);

    // Step 2: Remove the extension, if any
    match last_segment.rfind('.') {
        Some(idx) => last_segment[..idx].to_string(),
        None => last_segment.to_string(),
    }
}

/// Read a enviroment variable (works with wasm by returning "")
#[cfg(not(target_arch = "wasm32"))]
pub fn read_env(var: &str) -> String {
    let res = std::env::var(var);
    if res.is_err() {
        String::from("")
    } else {
        res.unwrap()
    }
}

/// Read a enviroment variable (works with wasm by returning "")
#[cfg(target_arch = "wasm32")]
pub fn read_env(var: &str) -> String {
    String::from("")
}