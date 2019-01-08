use crate::images;
use rider_config::directories::*;
use std::fs;

pub fn create() {
    if !themes_dir().exists() {
        let r = fs::create_dir_all(&themes_dir());
        #[cfg_attr(tarpaulin, skip)]
        r.unwrap_or_else(|_| panic!("Cannot create themes config directory"));
        images::create();
    }

    if !fonts_dir().exists() {
        let r = fs::create_dir_all(&fonts_dir());
        #[cfg_attr(tarpaulin, skip)]
        r.unwrap_or_else(|_| panic!("Cannot create fonts config directory"));
        write_default_fonts();
    }

    if !log_dir().exists() {
        let r = fs::create_dir_all(&log_dir());
        #[cfg_attr(tarpaulin, skip)]
        r.unwrap_or_else(|_| panic!("Cannot create log directory"));
    }

    if !project_dir().exists() {
        let r = fs::create_dir_all(&project_dir());
        #[cfg_attr(tarpaulin, skip)]
        r.unwrap_or_else(|_| panic!("Cannot create project directory"));
    }
}

fn write_default_fonts() {
    {
        let mut default_font_path = fonts_dir();
        default_font_path.push("DejaVuSansMono.ttf");
        let contents = include_bytes!("../assets/fonts/DejaVuSansMono.ttf");
        let r = fs::write(default_font_path, contents.to_vec());
        #[cfg_attr(tarpaulin, skip)]
        r.unwrap_or_else(|_| panic!("Cannot write default font file!"));
    }
    {
        let mut default_font_path = fonts_dir();
        default_font_path.push("ElaineSans-Medium.ttf");
        let contents = include_bytes!("../assets/fonts/ElaineSans-Medium.ttf");
        let r = fs::write(default_font_path, contents.to_vec());
        #[cfg_attr(tarpaulin, skip)]
        r.unwrap_or_else(|_| panic!("Cannot write default font file!"));
    }
}
