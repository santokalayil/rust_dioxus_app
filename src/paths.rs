use std::path::{Path, PathBuf};


fn get_main_dir() -> PathBuf {
    Path::new(".").canonicalize().unwrap()
}

pub fn get(path_str: &str) -> PathBuf {
    let main_dir = get_main_dir();
    if path_str.eq("main") {
        Path::new(".").canonicalize().unwrap()
    } else if path_str.eq("resources") {
        main_dir.join("resources")
    } else {
        main_dir
    }
}
