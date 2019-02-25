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
    images::create();
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
        Path::new(join(dir.clone(), sub.to_owned()).as_str()).exists()
    }

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
        assert_eq!(exists(&rider_dir, "themes"), false);
        assert_eq!(exists(&rider_dir, "log"), false);
        assert_eq!(exists(&test_path, ".rider"), false);
        assert_eq!(exists(&rider_dir, "themes/default.json"), false);
        assert_eq!(exists(&rider_dir, "themes/railscasts.json"), false);
        main();
        assert_eq!(exists(&rider_dir, "fonts"), true);
        assert_eq!(exists(&rider_dir, "log"), true);
        assert_eq!(exists(&rider_dir, "themes"), true);
        assert_eq!(exists(&test_path, ".rider"), true);
        assert_eq!(exists(&rider_dir, "themes/default.json"), true);
        assert_eq!(exists(&rider_dir, "themes/railscasts.json"), true);
    }
}
