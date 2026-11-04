use std::path::PathBuf;

pub fn check_is_file(path: &PathBuf) -> bool {
    if path.is_dir() {
        return false;
    } else {
        return true;
    }
}
