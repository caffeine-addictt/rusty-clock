use std::fs;
use std::path::{Path, PathBuf};

pub fn walk_style_dir() -> Vec<PathBuf> {
    let styles_dir = Path::new("styles");

    if !styles_dir.exists() || !styles_dir.is_dir() {
        return Vec::new();
    }

    match fs::read_dir(styles_dir) {
        Ok(entries) => {
            let mut file_paths = Vec::new();
            for entry in entries {
                let entry = entry.unwrap().path();
                if entry.is_file() {
                    file_paths.push(entry);
                }
            }
            file_paths
        }
        Err(_) => Vec::new(),
    }
}
