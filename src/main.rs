
mod args;
mod hash;
mod report;

use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};

use clap::Parser;
use walkdir::WalkDir;
use fs_extra::file::move_file;

use args::Args;
use hash::hash_file;
use report::DuplicateGroup;

fn main() {
    let args = Args::parse();
    let folder_path = &args.folder;

    println!("Scanning folder: {}", folder_path);

    let mut map: HashMap<String, Vec<PathBuf>> = HashMap::new();

    // Step 1: Walk through files and group by hash
    for entry in WalkDir::new(folder_path) {
        let entry = match entry {
            Ok(e) => e,
            Err(_) => continue,
        };

        let path = entry.path();
        if path.is_file() {
            match hash_file(path) {
                Ok(hash) => {
                    map.entry(hash).or_default().push(path.to_path_buf());
                }
                Err(e) => {
                    eprintln!("Failed to hash file {}: {}", path.display(), e);
                }
            }
        }
    }

    // Step 2: Create quarantine folder
    let quarantine_dir = Path::new(folder_path).join("quarantine");
    let _ = fs::create_dir_all(&quarantine_dir);

    // Step 3: Process duplicates
    let mut report: Vec<DuplicateGroup> = Vec::new();

    for (hash, files) in map {
        if files.len() > 1 {
            println!(" Duplicate found:");
            for (i, file) in files.iter().enumerate() {
                print!(" - {}", file.display());
                if i > 0 {
                    println!(" (moved)");
                    if let Some(name) = file.file_name() {
                        let target = quarantine_dir.join(name);
                        let _ = move_file(file, &target, &fs_extra::file::CopyOptions::new());
                    }
                } else {
                    println!();
                }
            }

            report.push(DuplicateGroup {
                hash,
                files: files.iter().map(|f| f.display().to_string()).collect(),
            });
        }
    }

    // Step 4: Write JSON report
    let report_path = Path::new(folder_path).join("duplicates_report.json");
    match serde_json::to_string_pretty(&report) {
        Ok(json) => {
            if let Err(e) = fs::write(&report_path, json) {
                eprintln!("Failed to write report: {}", e);
            } else {
                println!("Report saved to {}", report_path.display());
            }
        }
        Err(e) => {
            eprintln!("Failed to serialize report: {}", e);
        }
    }

    println!("Done!");
}
