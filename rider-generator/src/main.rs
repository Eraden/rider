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
    use std::fs::{create_dir_all, remove_dir_all};
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
        remove_dir_all(joined.clone()).unwrap_or_else(|_| ());
        create_dir_all(test_path.to_owned()).unwrap();

        set_var("XDG_CONFIG_HOME", test_path);
        set_var("XDG_RUNTIME_DIR", test_path);

        debug_assert!(
            !exists(&test_path.to_owned(), ".rider"),
            "rider config dir should not exists before generator run"
        );
        debug_assert!(main().is_ok(), "generator should not failed");
        debug_assert!(
            exists(&test_path.to_owned(), ".rider"),
            "rider config dir should exists after generator run"
        );
    }

    #[test]
    fn assert_fonts_dir() {
        let uniq = Uuid::new_v4();
        let joined = join("/tmp/rider-tests".to_owned(), uniq.to_string());

        remove_dir_all(joined.clone()).unwrap_or_else(|_| ());
        create_dir_all(joined.clone()).unwrap();

        set_var("XDG_CONFIG_HOME", joined.as_str().clone());
        set_var("XDG_RUNTIME_HOME", joined.as_str().clone());

        debug_assert!(
            !exists(&joined, "rider/fonts"),
            "fonts director should not exists before run generator"
        );
        debug_assert!(main().is_ok(), "generator should not failed");
        debug_assert!(
            exists(&joined, "rider/fonts"),
            "fonts director should exists after run generator"
        );
    }

    #[test]
    fn assert_log_dir() {
        let uniq = Uuid::new_v4();
        let joined = join("/tmp/rider-tests".to_owned(), uniq.to_string());

        remove_dir_all(joined.clone()).unwrap_or_else(|_| ());
        create_dir_all(joined.clone()).unwrap();

        set_var("XDG_CONFIG_HOME", joined.as_str().clone());
        set_var("XDG_RUNTIME_HOME", joined.as_str().clone());

        debug_assert!(
            !exists(&joined, "rider/log"),
            "log should not exists before run generator"
        );
        debug_assert!(main().is_ok(), "generator should not failed");
        debug_assert!(
            exists(&joined, "rider/log"),
            "log should exists after run generator"
        );
    }

    #[test]
    fn assert_themes_dir() {
        let uniq = Uuid::new_v4();
        let joined = join("/tmp/rider-tests".to_owned(), uniq.to_string());

        remove_dir_all(joined.clone()).unwrap_or_else(|_| ());
        create_dir_all(joined.clone()).unwrap();

        set_var("XDG_CONFIG_HOME", joined.as_str().clone());
        set_var("XDG_RUNTIME_HOME", joined.as_str().clone());

        debug_assert!(
            !exists(&joined, "rider/themes"),
            "themes should not exists before run generator"
        );
        debug_assert!(main().is_ok(), "generator should not failed");
        debug_assert!(
            exists(&joined, "rider/themes"),
            "themes should exists after run generator"
        );
    }

    #[test]
    fn assert_default_json() {
        let uniq = Uuid::new_v4();
        let joined = join("/tmp/rider-tests".to_owned(), uniq.to_string());

        remove_dir_all(joined.clone()).unwrap_or_else(|_| ());
        create_dir_all(joined.clone()).unwrap();

        set_var("XDG_CONFIG_HOME", joined.as_str().clone());
        set_var("XDG_RUNTIME_HOME", joined.as_str().clone());

        debug_assert!(
            !exists(&joined, "rider/themes/default.json"),
            "default theme should not exists before run generator"
        );
        debug_assert!(main().is_ok(), "generator should not failed");
        debug_assert!(
            exists(&joined, "rider/themes/default.json"),
            "default theme should exists after run generator"
        );
    }

    #[test]
    fn assert_railscasts_json() {
        let uniq = Uuid::new_v4();
        let joined = join("/tmp/rider-tests".to_owned(), uniq.to_string());

        remove_dir_all(joined.clone()).unwrap_or_else(|_| ());
        create_dir_all(joined.clone()).unwrap();

        set_var("XDG_CONFIG_HOME", joined.as_str().clone());
        set_var("XDG_RUNTIME_HOME", joined.as_str().clone());

        debug_assert!(
            !exists(&joined, "rider/themes/railscasts.json"),
            "railscasts theme should not exists before run generator"
        );
        debug_assert!(main().is_ok(), "generator should not failed");
        debug_assert!(
            exists(&joined, "rider/themes/railscasts.json"),
            "railscasts theme should exists after run generator"
        );
    }
}
