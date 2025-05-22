pub mod version;
pub mod type_of;
pub mod h;
pub mod reader;
pub mod js_helpers;
pub mod input;
pub mod ej_config;

use std::env;
use std::path::PathBuf;

/// Get the path of our executable.
pub fn get_exe_dir() -> Option<PathBuf> {
    env::current_exe()
        .ok()
        .and_then(|path| path.parent().map(|p| p.to_path_buf()))
}
