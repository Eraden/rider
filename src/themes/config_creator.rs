use crate::config::directories::*;
use crate::themes::predef::*;
use crate::themes::*;
use dirs;
use std::fs;
use std::path::PathBuf;

pub fn create() {
    fs::create_dir_all(themes_dir())
        .unwrap_or_else(|_| panic!("Cannot create theme config directory"));
    for theme in default_styles() {
        write_theme(&theme);
    }
}

fn write_theme(theme: &Theme) {
    let mut theme_path = themes_dir();
    theme_path.push(format!("{}.json", theme.name()));
    let contents = serde_json::to_string_pretty(&theme).unwrap();
    fs::write(&theme_path, contents.clone())
        .unwrap_or_else(|_| panic!("Failed to crate theme config file"));
}

fn default_styles() -> Vec<Theme> {
    vec![default::build_theme(), railscasts::build_theme()]
}
