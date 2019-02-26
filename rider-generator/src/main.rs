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

use rider_config::directories::Directories;

pub mod config;
pub mod images;
pub mod themes;
pub mod write_bytes_to;

fn main() -> std::io::Result<()> {
    let directories = Directories::new(None, None);
    config::create(&directories)?;
    themes::create(&directories)?;
    images::create(&directories)?;
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
    fn exists(dir: &String, sub: &str) -> bool {
        let joined = join(dir.clone(), sub.to_owned());
        Path::new(joined.as_str()).exists()
    }

    #[cfg(test)]
    fn join(a: String, b: String) -> String {
        vec![a, b].join("/")
    }

    #[test]
    fn assert_main() {
        let uniq = Uuid::new_v4();
        let joined = join("/tmp/rider-tests".to_owned(), uniq.to_string());
        let test_path = joined.as_str();
        create_dir_all(test_path.to_owned()).unwrap();

        set_var("XDG_CONFIG_HOME", test_path);
        set_var("XDG_RUNTIME_DIR", test_path);

        assert_eq!(exists(&test_path.to_owned(), ".rider"), false);
        assert_eq!(main().is_ok(), true);
        assert_eq!(exists(&test_path.to_owned(), ".rider"), true);
    }

    #[test]
    fn assert_fonts_dir() {
        let uniq = Uuid::new_v4();
        let joined = join("/tmp/rider-tests".to_owned(), uniq.to_string());
        create_dir_all(joined.clone()).unwrap();
        set_var("XDG_CONFIG_HOME", joined.as_str().clone());
        set_var("XDG_RUNTIME_HOME", joined.as_str().clone());
        assert_eq!(exists(&joined, "rider/fonts"), false);
        assert_eq!(main().is_ok(), true);
        assert_eq!(exists(&joined, "rider/fonts"), true);
    }

    #[test]
    fn assert_log_dir() {
        let uniq = Uuid::new_v4();
        let joined = join("/tmp/rider-tests".to_owned(), uniq.to_string());
        create_dir_all(joined.clone()).unwrap();
        set_var("XDG_CONFIG_HOME", joined.as_str().clone());
        set_var("XDG_RUNTIME_HOME", joined.as_str().clone());
        assert_eq!(exists(&joined, "rider/log"), false);
        assert_eq!(main().is_ok(), true);
        assert_eq!(exists(&joined, "rider/log"), true);
    }

    #[test]
    fn assert_themes_dir() {
        let uniq = Uuid::new_v4();
        let joined = join("/tmp/rider-tests".to_owned(), uniq.to_string());
        create_dir_all(joined.clone()).unwrap();
        set_var("XDG_CONFIG_HOME", joined.as_str().clone());
        set_var("XDG_RUNTIME_HOME", joined.as_str().clone());
        assert_eq!(exists(&joined, "rider/themes"), false);
        assert_eq!(main().is_ok(), true);
        assert_eq!(exists(&joined, "rider/themes"), true);
    }

    #[test]
    fn assert_default_json() {
        let uniq = Uuid::new_v4();
        let joined = join("/tmp/rider-tests".to_owned(), uniq.to_string());
        create_dir_all(joined.clone()).unwrap();
        set_var("XDG_CONFIG_HOME", joined.as_str().clone());
        set_var("XDG_RUNTIME_HOME", joined.as_str().clone());
        assert_eq!(exists(&joined, "rider/themes/default.json"), false);
        assert_eq!(main().is_ok(), true);
        assert_eq!(exists(&joined, "rider/themes/default.json"), true);
    }

    #[test]
    fn assert_railscasts_json() {
        let uniq = Uuid::new_v4();
        let joined = join("/tmp/rider-tests".to_owned(), uniq.to_string());
        create_dir_all(joined.clone()).unwrap();
        set_var("XDG_CONFIG_HOME", joined.as_str().clone());
        set_var("XDG_RUNTIME_HOME", joined.as_str().clone());
        assert_eq!(exists(&joined, "rider/themes/railscasts.json"), false);
        assert_eq!(main().is_ok(), true);
        assert_eq!(exists(&joined, "rider/themes/railscasts.json"), true);
    }
}
