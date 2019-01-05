use crate::config::directories::*;
use crate::themes::config_creator;
use dirs;
use std::fs;
use std::path;

pub fn create() {
    if !themes_dir().exists() {
        fs::create_dir_all(&themes_dir())
            .unwrap_or_else(|_| panic!("Cannot create themes config directory"));
    }

    if !fonts_dir().exists() {
        fs::create_dir_all(&fonts_dir())
            .unwrap_or_else(|_| panic!("Cannot create fonts config directory"));
        write_default_fonts();
    }

    if !log_dir().exists() {
        fs::create_dir_all(&log_dir()).unwrap_or_else(|_| panic!("Cannot create log directory"));
    }

    if !project_dir().exists() {
        fs::create_dir_all(&project_dir())
            .unwrap_or_else(|_| panic!("Cannot create project directory"));
    }
}

fn write_default_fonts() {
    {
        let mut default_font_path = fonts_dir();
        default_font_path.push("DejaVuSansMono.ttf");
        let contents = include_bytes!("../../assets/fonts/DejaVuSansMono.ttf");
        fs::write(default_font_path, contents.to_vec())
            .unwrap_or_else(|_| panic!("Cannot write default font file!"));
    }
    {
        let mut default_font_path = fonts_dir();
        default_font_path.push("ElaineSans-Medium.ttf");
        let contents = include_bytes!("../../assets/fonts/ElaineSans-Medium.ttf");
        fs::write(default_font_path, contents.to_vec())
            .unwrap_or_else(|_| panic!("Cannot write default font file!"));
    }
}
