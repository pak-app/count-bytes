
use std::path::PathBuf;
use std::fs;

pub fn check_is_file(path: &PathBuf) -> bool {
    if path.is_dir() {
        return false;
    } else {
        return true;
    }
}

pub fn get_file_size(path: PathBuf) -> std::io::Result<u64> {
    Ok(fs::metadata(path)?.len())
}
