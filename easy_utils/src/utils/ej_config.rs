use std::error::Error;
use std::fs;
use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};

/// Corresponds to a .ejconfig file
#[derive(Serialize, Deserialize, Debug)]
pub struct EJConfig {
    pub name: String,
    pub description: String,
    pub author: Author,
    /// Where is the source .ej file located?
    pub source: String,
    /// Which runtime should we use to run this?
    pub runtime: String,
    /// What is the output name.
    pub output: String,
    /// Is this a package installed globally?
    pub global: bool
}

/// The Author details of a .ejconfig file.
#[derive(Serialize, Deserialize, Debug)]
pub struct Author {
    pub name: String,
    pub email: String,
}

/// Parse a .ejconfig file.
pub fn parse_ej_config(file: PathBuf) -> Result<EJConfig, Box<dyn Error>> {
    // Example implementation (replace with your actual logic)
    let contents = fs::read_to_string(file)?;
    let config: EJConfig = serde_json::from_str(&contents)?;
    Ok(config)
}

/// Find all the `.ejconfig` files in a directory.
pub fn get_ej_config(dir: &str) -> Vec<PathBuf> {
    let mut configs = Vec::new();

    if let Ok(entries) = fs::read_dir(dir) {
        for entry in entries.flatten() {
            let path = entry.path();

            // Check only regular files (not directories), with ".ejconfig" extension
            if path.is_file() && path.extension().and_then(|s| s.to_str()) == Some("ejconfig") {
                configs.push(path);
            }
        }
    }

    configs
}
