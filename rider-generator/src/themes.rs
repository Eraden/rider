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

    #[test]
    fn assert_default_styles() {
        assert_eq!(default_styles().len(), 2);
    }
}
