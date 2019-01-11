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

#[cfg(test)]
mod tests {
    use super::*;
    use uuid::Uuid;
    use std::fs::create_dir_all;
    use std::env::set_var;
    use std::path::{Path};

    #[cfg(test)]
    fn join(a: String, b: String) -> String {
        vec![a, b].join("/")
    }

    #[test]
    fn assert_create() {
        let uniq = Uuid::new_v4();
        let test_path = join("/tmp".to_owned(), uniq.to_string());
        create_dir_all(test_path.clone()).unwrap();
        set_var("XDG_CONFIG_HOME", test_path.as_str());
        set_var("XDG_RUNTIME_DIR", test_path.as_str());
        let rider_dir = join(test_path.clone(), "rider".to_owned());
        assert_eq!(Path::new(join(rider_dir.clone(), "themes".to_owned()).as_str()).exists(), false);
        assert_eq!(Path::new(join(rider_dir.clone(), "log".to_owned()).as_str()).exists(), false);
        assert_eq!(Path::new(join(test_path.clone(), ".rider".to_owned()).as_str()).exists(), false);
        create();
        assert_eq!(Path::new(join(rider_dir.clone(), "fonts".to_owned()).as_str()).exists(), true);
        assert_eq!(Path::new(join(rider_dir.clone(), "log".to_owned()).as_str()).exists(), true);
        assert_eq!(Path::new(join(rider_dir.clone(), "themes".to_owned()).as_str()).exists(), true);
        assert_eq!(Path::new(join(test_path.clone(), ".rider".to_owned()).as_str()).exists(), true);
    }
}
