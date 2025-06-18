use std::fs;
use std::path::{Path, PathBuf};

pub fn collect_files(root: &Path) -> Vec<PathBuf> {
    let mut files = Vec::new();

    if root.is_file() {
        if let Some(ext) = root.extension().and_then(|e| e.to_str()) {
            if ext == "py" || ext == "ipynb" {
                files.push(root.to_path_buf());
            }
        }
    } else if root.is_dir() {
        if let Ok(entries) = fs::read_dir(root) {
            for entry in entries.flatten() {
                files.extend(collect_files(&entry.path()));
            }
        }
    }

    files
}
