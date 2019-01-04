use dirs;
use std::path::PathBuf;

pub fn log_dir() -> PathBuf {
    let mut log_dir = config_dir();
    log_dir.push("log");
    log_dir
}

pub fn themes_dir() -> PathBuf {
    let mut themes_dir = config_dir();
    themes_dir.push("themes");
    themes_dir
}

pub fn fonts_dir() -> PathBuf {
    let mut fonts_dir = config_dir();
    fonts_dir.push("fonts");
    fonts_dir
}

pub fn config_dir() -> PathBuf {
    let home_dir = dirs::config_dir().unwrap();

    let mut config_dir = home_dir.clone();
    config_dir.push("rider");
    config_dir
}
