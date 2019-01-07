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
#[macro_use]
extern crate log;
extern crate simplelog;
#[macro_use]
extern crate lazy_static;

use crate::app::Application;
use crate::config::directories::log_dir;
use log::Level;
use simplelog::*;
use std::fs::create_dir_all;
use std::fs::File;

pub mod app;
pub mod config;
pub mod lexer;
pub mod renderer;
#[cfg(test)]
pub mod tests;
pub mod themes;
pub mod ui;

fn init_logger() {
    let mut log_file_path = log_dir();
    log_file_path.push("rider.log");

    CombinedLogger::init(vec![
//        TermLogger::new(LevelFilter::Warn, Config::default()).unwrap(),
        WriteLogger::new(
            LevelFilter::Info,
            Config::default(),
            File::create(log_file_path).unwrap(),
        ),
    ])
    .unwrap();
}

fn main() {
    let mut app = Application::new();
    app.init();
    init_logger();
    app.open_file("./assets/examples/test.rs".to_string());
    app.run();
}
