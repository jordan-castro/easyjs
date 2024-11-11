use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

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

    format!("{}{}", input[0..1].to_string(), hash[..4].to_string())
}