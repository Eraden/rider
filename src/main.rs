#![allow(unused_imports)]

extern crate plex;
extern crate rand;
extern crate sdl2;
extern crate dirs;
#[macro_use]
extern crate serde;
#[macro_use]
extern crate serde_json;
#[macro_use]
extern crate serde_derive;

pub mod app;
pub mod ui;
pub mod file;
pub mod lexer;
pub mod renderer;
pub mod themes;

use crate::app::Application;

fn main() {
    let mut app = Application::new();
    app.init();
    app.open_file("./tests/example.txt".to_string());
    app.run();
}
