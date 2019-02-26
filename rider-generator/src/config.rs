use crate::images;
use rider_config::directories::*;
use std::fs;

pub fn create() -> std::io::Result<()> {
    if !themes_dir().exists() {
        fs::create_dir_all(&themes_dir())?;
        images::create()?;
    }

    if !fonts_dir().exists() {
        write_default_fonts()?;
    }

    if !log_dir().exists() {
        fs::create_dir_all(&log_dir())?;
    }

    if !project_dir().exists() {
        fs::create_dir_all(&project_dir())?;
    }
    Ok(())
}

fn write_default_fonts() -> std::io::Result<()> {
    fs::create_dir_all(&fonts_dir())?;
    #[cfg_attr(tarpaulin, skip)]
    {
        let mut default_font_path = fonts_dir();
        let _x = default_font_path.as_os_str().to_str().unwrap();
        default_font_path.push("DejaVuSansMono.ttf");
        let _x = default_font_path.as_os_str().to_str().unwrap();
        let contents = include_bytes!("../assets/fonts/DejaVuSansMono.ttf");
        fs::write(default_font_path, contents.to_vec())?;
    }
    {
        let mut default_font_path = fonts_dir();
        let _x = default_font_path.as_os_str().to_str().unwrap();
        default_font_path.push("ElaineSans-Medium.ttf");
        let _x = default_font_path.as_os_str().to_str().unwrap();
        let contents = include_bytes!("../assets/fonts/ElaineSans-Medium.ttf");
        fs::write(default_font_path, contents.to_vec())?;
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env::set_var;
    use std::fs::create_dir_all;
    use std::path::Path;
    use uuid::Uuid;

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
        assert_eq!(
            Path::new(join(rider_dir.clone(), "themes".to_owned()).as_str()).exists(),
            false
        );
        assert_eq!(
            Path::new(join(rider_dir.clone(), "log".to_owned()).as_str()).exists(),
            false
        );
        assert_eq!(
            Path::new(join(test_path.clone(), ".rider".to_owned()).as_str()).exists(),
            false
        );
        assert_eq!(create().is_ok(), true);
        assert_eq!(
            Path::new(join(rider_dir.clone(), "fonts".to_owned()).as_str()).exists(),
            true
        );
        assert_eq!(
            Path::new(join(rider_dir.clone(), "log".to_owned()).as_str()).exists(),
            true
        );
        assert_eq!(
            Path::new(join(rider_dir.clone(), "themes".to_owned()).as_str()).exists(),
            true
        );
        assert_eq!(
            Path::new(join(test_path.clone(), ".rider".to_owned()).as_str()).exists(),
            true
        );
    }
}
