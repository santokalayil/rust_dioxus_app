use std::path::Path;
use std::module_path;


use dioxus::prelude::*;
use dioxus_logger::tracing::{info, error, debug};

use crate::utils::{yaml, text};
use crate::paths;




#[component]
pub fn render() -> Element {
    // let testing_text = main_dir!();
    // let testing_text = module_path!();
    // let current_dir = Path::new(".").canonicalize().unwrap();
    // let testing_text = format!("{testing_text:?}");
    let mut contents = String::new();
    let fetched_path = paths::get("resources").join("general_config.yaml");
    // debug!("{fetched_path:?}");
    debug!("Contents --> {contents:?}");
    match text::read(fetched_path.as_path()) {
        Ok(content) => {contents = content}
        Err(error) => {contents = format!("Error {error}")}
    };
    debug!("Contents --> {contents:?}");
    rsx! {
        p { "{contents}" }
        // p { "{testing_text:?}"}
        // p { "{current_dir:?}" }
    }
}
