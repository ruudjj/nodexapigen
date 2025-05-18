use std::fs::{self};
use std::path::Path;

/// Create a folder (and any parent folders)
pub fn create_folder<P: AsRef<Path>>(path: P) {
    fs::create_dir_all(&path).expect(&format!("Failed to create folder: {:?}", path.as_ref()));
}