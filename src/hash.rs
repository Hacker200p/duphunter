// src/hash.rs
use std::fs::File;
use std::io::{self, Read};
use std::path::Path;
use sha2::{Sha256, Digest};

pub fn hash_file(path: &Path) -> io::Result<String> {
    let mut file = File::open(path)?;
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)?;
    let mut hasher = Sha256::new();
    hasher.update(&buffer);
    Ok(format!("{:x}", hasher.finalize()))
}
