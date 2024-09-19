use std::path::Path;
use std::io::Error;
use dioxus_logger::tracing::{info, error};

use crate::utils::text;


pub fn read(path: &Path) {
    let result: Result<String, Error> = text::read(path);
    if let Ok(content) = result {
        info!("Content \n{content}")
    } else {
        error!("Unable to get file content")
    }
}

