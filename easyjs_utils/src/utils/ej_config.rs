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
    EJConfig::from_json(&contents).map_err(|e| Box::new(e) as Box<dyn Error>)
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

impl EJConfig {
    /// from_json
    pub fn from_json(json_string: &str) -> Result<EJConfig, serde_json::Error> {
        serde_json::from_str(json_string)
    }
    /// to_string
    pub fn to_string(self) -> Result<String, serde_json::Error> {
        serde_json::to_string(&self)
    }
}