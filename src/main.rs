#![allow(unused_imports)]

extern crate dirs;
extern crate plex;
extern crate rand;
extern crate sdl2;
#[macro_use]
extern crate serde;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate serde_json;

use crate::app::Application;

pub mod app;
pub mod config;
pub mod file;
pub mod lexer;
pub mod renderer;
pub mod themes;
pub mod ui;

fn main() {
    let mut app = Application::new();
    app.init();
    app.open_file("./tests/example.txt".to_string());
    app.run();
}
