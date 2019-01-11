use crate::*;
use rider_config::directories::*;
use rider_themes::predef::*;
use rider_themes::Theme;
use std::fs;

pub fn create() {
    let r = fs::create_dir_all(themes_dir());
    #[cfg_attr(tarpaulin, skip)]
    r.unwrap_or_else(|_| panic!("Cannot create theme config directory"));
    for theme in default_styles() {
        write_theme(&theme);
    }
}

fn write_theme(theme: &Theme) {
    let mut theme_path = themes_dir();
    theme_path.push(format!("{}.json", theme.name()));
    let contents = serde_json::to_string_pretty(&theme).unwrap();
    let r = fs::write(&theme_path, contents.clone());
    #[cfg_attr(tarpaulin, skip)]
    r.unwrap_or_else(|_| panic!("Failed to crate theme config file"));
}

fn default_styles() -> Vec<Theme> {
    vec![default::build_theme(), railscasts::build_theme()]
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env::set_var;
    use std::fs::create_dir_all;
    use std::path::Path;
    use uuid::Uuid;

    #[test]
    fn assert_default_styles() {
        assert_eq!(default_styles().len(), 2);
    }

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
            Path::new(join(rider_dir.clone(), "themes/default.json".to_owned()).as_str()).exists(),
            false
        );
        assert_eq!(
            Path::new(join(rider_dir.clone(), "themes/railscasts.json".to_owned()).as_str())
                .exists(),
            false
        );
        create();
        assert_eq!(
            Path::new(join(rider_dir.clone(), "themes/default.json".to_owned()).as_str()).exists(),
            true
        );
        assert_eq!(
            Path::new(join(rider_dir.clone(), "themes/railscasts.json".to_owned()).as_str())
                .exists(),
            true
        );
    }
}
