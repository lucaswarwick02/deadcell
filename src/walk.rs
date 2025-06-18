use std::fs;
use std::path::PathBuf;

pub fn visit_paths(path: &PathBuf) {
    if path.is_file() {
        if let Some(ext) = path.extension().and_then(|e| e.to_str()) {
            if ext == "py" || ext == "ipynb" {
                println!("File: {}", path.display());
            }
        }
    } else if path.is_dir() {
        if let Ok(entries) = fs::read_dir(path) {
            for entry in entries.flatten() {
                visit_paths(&entry.path());
            }
        }
    }
}