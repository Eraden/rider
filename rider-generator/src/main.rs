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

fn main() {
    config::create();
    themes::create();
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
    fn assert_main() {
        let uniq = Uuid::new_v4();
        let test_path = join("/tmp".to_owned(), uniq.to_string());
        create_dir_all(test_path.clone()).unwrap();
        set_var("XDG_CONFIG_HOME", test_path.as_str());
        set_var("XDG_RUNTIME_DIR", test_path.as_str());
        let rider_dir = join(test_path.clone(), "rider".to_owned());
        assert_eq!(Path::new(join(rider_dir.clone(), "themes".to_owned()).as_str()).exists(), false);
        assert_eq!(Path::new(join(rider_dir.clone(), "log".to_owned()).as_str()).exists(), false);
        assert_eq!(Path::new(join(test_path.clone(), ".rider".to_owned()).as_str()).exists(), false);
        assert_eq!(Path::new(join(rider_dir.clone(), "themes/default.json".to_owned()).as_str()).exists(), false);
        assert_eq!(Path::new(join(rider_dir.clone(), "themes/railscasts.json".to_owned()).as_str()).exists(), false);
        main();
        assert_eq!(Path::new(join(rider_dir.clone(), "fonts".to_owned()).as_str()).exists(), true);
        assert_eq!(Path::new(join(rider_dir.clone(), "log".to_owned()).as_str()).exists(), true);
        assert_eq!(Path::new(join(rider_dir.clone(), "themes".to_owned()).as_str()).exists(), true);
        assert_eq!(Path::new(join(test_path.clone(), ".rider".to_owned()).as_str()).exists(), true);
        assert_eq!(Path::new(join(rider_dir.clone(), "themes/default.json".to_owned()).as_str()).exists(), true);
        assert_eq!(Path::new(join(rider_dir.clone(), "themes/railscasts.json".to_owned()).as_str()).exists(), true);
    }
}
