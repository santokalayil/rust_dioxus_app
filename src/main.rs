#![allow(non_snake_case)]

use dioxus::prelude::*;
use dioxus_logger::tracing::{info, Level};

mod ui;
mod config;
pub mod utils;
pub mod paths;


use crate::utils::text;
// use crate::sections::view_header;
use crate::ui::sections::{header, body};

fn main() {
    // Init logger
    dioxus_logger::init(Level::DEBUG).expect("failed to init logger");
    info!("starting app");
    launch(App);
}

#[component]
fn App() -> Element {
    // Build cool things ✌️
    // let mut contents = String::new();
    // match text::read(paths::get("resources").join("sant.yaml").as_path()) {
    //     Ok(content) => {contents = content}
    //     Err(error) => {contents = format!("Error {error}")}
    // };

    rsx! {
        // link { rel: "stylesheet", href: "main.css" }
        header::render {}
        body::render {}
        // h1 {"{content/s}"}
    }
}





// fn main() {
//     println!("Hi");
//     let mut contents = String::new();
//     match text::read(paths::get("resources").join("santo.yaml").as_path()) {
//         Ok(content) => {contents = content}
//         Err(error) => {contents = format!("Error {error}")}
//     };
//     println!("Contents: {contents}");
// }
