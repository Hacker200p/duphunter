// src/args.rs
use clap::Parser;

#[derive(Parser)]
#[command(author, version, about)]
pub struct Args {
    /// Path to folder to scan
    pub folder: String,
}
