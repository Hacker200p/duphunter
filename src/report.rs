// src/report.rs
use serde::Serialize;

#[derive(Serialize)]
pub struct DuplicateGroup {
    pub hash: String,
    pub files: Vec<String>,
}
