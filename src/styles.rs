use std::fs;
use std::fs::File;
use std::io::{self, BufRead};
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

pub fn load_styles(path: &PathBuf) -> Vec<String> {
    let mut styles = vec![];
    let mut buffer = String::new();
    let separator: &str = "/=/";

    let reader = io::BufReader::new(File::open(path).unwrap());
    for line in reader.lines() {
        let unwrapped = line.unwrap();

        // Check if sep push buff
        if unwrapped.trim_end() == separator {
            styles.push(buffer);
            buffer = String::from("");
        } else {
            buffer.push_str(&unwrapped);
            buffer.push('\n');
        }
    }
    styles.push(buffer);

    if styles.len() != 11 {
        panic!(
            "{}",
            format!(
                "Expected 11 styles, found {} in {}",
                styles.len(),
                path.display()
            )
        );
    }

    styles
}
