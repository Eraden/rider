extern crate dirs;
extern crate log;
extern crate rand;
extern crate rider_config;
extern crate rider_themes;
extern crate serde;
extern crate serde_derive;
extern crate serde_json;
extern crate simplelog;
extern crate uuid;

pub mod config;
pub mod images;
pub mod themes;
pub mod write_bytes_to;

fn main() -> std::io::Result<()> {
    config::create()?;
    themes::create()?;
    images::create()?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use rider_config::directories::fonts_dir;
    use std::env::set_var;
    use std::fs::create_dir_all;
    use std::path::Path;
    use uuid::Uuid;

    #[cfg(test)]
    fn exists(dir: &String, sub: &str) -> bool {
        let joined = join(dir.clone(), sub.to_owned());
        //        let _xc = joined.as_str();
        Path::new(joined.as_str()).exists()
    }

    #[cfg(test)]
    fn join(a: String, b: String) -> String {
        vec![a, b].join("/")
    }

    #[test]
    fn assert_main() {
        let uniq = Uuid::new_v4();
        let joined = join("/tmp".to_owned(), uniq.to_string());
        let test_path = joined.as_str();
        create_dir_all(test_path.to_owned()).unwrap();
        //        set_var("HOME", test_path.as_str());

        //        let _home = dirs::home_dir().unwrap().to_str().unwrap();
        //        let _config = dirs::config_dir().unwrap().to_str().unwrap();
        //        let _fonts = fonts_dir().as_os_str().to_str().unwrap();

        set_var("HOME", "/tmp");
        set_var("XDG_CONFIG_HOME", test_path);
        set_var("XDG_RUNTIME_DIR", test_path);

        //        let _home = dirs::home_dir().unwrap().to_str().unwrap();
        //        let _config = dirs::config_dir().unwrap().to_str().unwrap();
        //        let _fonts = fonts_dir().as_os_str().to_str().unwrap();

        let rider_dir = join(test_path.to_owned(), "rider".to_owned());
        assert_eq!(exists(&rider_dir, "themes"), false);
        assert_eq!(exists(&rider_dir, "log"), false);
        assert_eq!(exists(&test_path.to_owned(), ".rider"), false);
        assert_eq!(exists(&rider_dir, "themes/default.json"), false);
        assert_eq!(exists(&rider_dir, "themes/railscasts.json"), false);
        assert_eq!(main().is_ok(), true);
        assert_eq!(exists(&rider_dir, "fonts"), true);
        assert_eq!(exists(&rider_dir, "log"), true);
        assert_eq!(exists(&rider_dir, "themes"), true);
        assert_eq!(exists(&test_path.to_owned(), ".rider"), true);
        assert_eq!(exists(&rider_dir, "themes/default.json"), true);
        assert_eq!(exists(&rider_dir, "themes/railscasts.json"), true);
    }
}
