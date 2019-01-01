#![allow(unused_imports)]
#![feature(use_extern_macros)]

extern crate plex;
extern crate rand;
extern crate sdl2;

pub mod app;
pub mod file;
pub mod lexer;
pub mod renderer;

use crate::app::Application;

fn main() {
    let mut app = Application::new();
    app.init();
    app.open_file("./tests/example.txt".to_string());
    app.run();
}
