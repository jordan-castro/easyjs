use crate::std::load_std;
use easy_utils::utils::reader::read_file;

/// Import a `easyjs` file.
pub fn import_file(file_path: &str) -> String {
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