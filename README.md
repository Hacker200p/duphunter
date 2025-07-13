# File Deduplicator

A simple Rust tool that finds duplicate files in a directory and moves them to a quarantine folder.

## What it does

1. *Scans a directory* recursively for all files
2. *Hashes each file* using SHA-256 to identify duplicates
3. *Moves duplicates* to a quarantine folder (keeps the first file as original)
4. *Generates a JSON report* in the scanned directory

## Usage

bash
cargo run <directory_path>


### Example

bash
# Scan your Documents folder
cargo run "C:\Users\Username\Documents"

# Scan current directory
cargo run "."


## Output

### Console Output

Scanning folder: C:\Users\Username\Documents
 Duplicate found:
 - C:\Users\Username\Documents\file1.txt
 - C:\Users\Username\Documents\backup\file1.txt (moved)
Report saved to duplicates_report.json
Done!


### Generated Files

- **quarantine/** - Folder containing all duplicate files
- **duplicates_report.json** - JSON report of all duplicate groups

### Report Format
json
[
  {
    "hash": "a1b2c3d4e5f6...",
    "files": [
      "C:\\Users\\Username\\Documents\\file1.txt",
      "C:\\Users\\Username\\Documents\\backup\\file1.txt"
    ]
  }
]


## How it works

1. *Scan*: Walks through all files in the specified directory
2. *Hash*: Creates SHA-256 hash of each file's content
3. *Group*: Groups files with identical hashes
4. *Move*: Moves duplicate files (keeping the first as original) to quarantine/
5. *Report*: Saves a JSON report in the scanned directory

## Requirements

- Rust and Cargo installed
- Read access to the target directory
- Write access to create quarantine folder and report

## Safety

- Original files are never deleted or moved
- Only duplicate files are moved to quarantine
- The first file in each duplicate group stays in place
