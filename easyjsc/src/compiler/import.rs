use std::collections::HashMap;

use crate::std::load_std;
use easyjs_utils::utils::reader::read_file;

/// Import a `easyjs` file.
pub fn import_file(file_path: &str, custom_libs: HashMap<String, String>) -> String {
    // Check if this is a custom lib
    if custom_libs.len() > 0 {
        let contents = custom_libs.get(file_path);
        if let Some(contents) = contents {
            return contents.to_owned();
        }
    }

    // check if this is a STD
    let std = load_std(file_path);
    if std != "" {
        std
    } else {
        // open file path and Read. 
        // based on current base path.
        read_file(file_path)
    } 
}