use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

use rand::distr::SampleString;

pub fn generate_hash(input: &str) -> String {
    let mut hasher = DefaultHasher::new();
    input.hash(&mut hasher);
    let hash = hasher.finish();
    format!("{:x}", hash)  // Convert hash to a hexadecimal string
}

pub fn hash_string(input: &str) -> String {
    let mut hasher = DefaultHasher::new();
    let _ = input.hash(&mut hasher);
    let hash = hasher.finish().to_string();

    format!("hashsed_{}{}", input[0..1].to_string(), hash[..4].to_string())
}

/// Generate a random hash from a length
pub fn random_hash(length: usize) -> String {
    let r = rand::distr::Alphanumeric.sample_string(&mut rand::rng(), length);
    format!("_{r}")
}

/// Get JS name if is_pub
pub fn get_js_name(ej_name: String, is_pub: bool) -> String {
    if is_pub {
        ej_name
    } else {
        random_hash(4)
    }
}