#![feature(clamp)]

extern crate dirs;
#[macro_use]
extern crate log;
extern crate lazy_static;
extern crate rand;
extern crate rider_config;
extern crate rider_lexers;
extern crate rider_themes;
extern crate sdl2;
extern crate serde;
extern crate serde_derive;
extern crate serde_json;
extern crate simplelog;

use crate::app::Application;
use rider_config::directories::Directories;
use simplelog::*;
use std::fs::File;

pub mod app;
pub mod renderer;
#[cfg(test)]
pub mod tests;
pub mod ui;

#[cfg_attr(tarpaulin, skip)]
fn init_logger(directories: &Directories) {
    //    use simplelog::SharedLogger;

    let mut log_file_path = directories.log_dir.clone();
    log_file_path.push("rider.log");

    let mut outputs: Vec<Box<dyn SharedLogger>> = vec![WriteLogger::new(
        LevelFilter::Info,
        Config::default(),
        File::create(log_file_path).unwrap(),
    )];
    let terminal_level = if cfg!(release) {
        LevelFilter::Trace
    } else {
        LevelFilter::Debug
    };
    if let Some(term) = TermLogger::new(terminal_level, Config::default()) {
        outputs.push(term);
    }

    CombinedLogger::init(outputs).unwrap();
}

#[cfg_attr(tarpaulin, skip)]
fn main() -> Result<(), String> {
    let directories = Directories::new(None, None);
    let mut app = Application::new();
    app.init();
    init_logger(&directories);
    app.open_file("./test_files/test.rs".to_string());
    app.run()
}
