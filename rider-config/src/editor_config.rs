use crate::directories;

#[derive(Debug, Clone)]
pub struct EditorConfig {
    character_size: u16,
    font_path: String,
    current_theme: String,
    margin_left: u16,
    margin_top: u16,
}

impl EditorConfig {
    pub fn new() -> Self {
        let mut default_font_path = directories::fonts_dir();
        default_font_path.push("DejaVuSansMono.ttf");
        Self {
            character_size: 14,
            font_path: default_font_path.to_str().unwrap().to_string(),
            current_theme: "railscasts".to_string(),
            margin_left: 10,
            margin_top: 10,
        }
    }

    pub fn character_size(&self) -> u16 {
        self.character_size
    }

    pub fn font_path(&self) -> &String {
        &self.font_path
    }

    pub fn current_theme(&self) -> &String {
        &self.current_theme
    }

    pub fn margin_left(&self) -> u16 {
        self.margin_left
    }

    pub fn margin_top(&self) -> u16 {
        self.margin_top
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::env::{set_var, temp_dir};

    #[test]
    fn assert_font_path() {
        let config = EditorConfig::new();
        set_var("XDG_CONFIG_HOME", temp_dir());
        let path = config.font_path().to_owned();
        let expected: String = "/tmp/rider/fonts/DejaVuSansMono.ttf".to_owned();
        assert_eq!(path, expected);
    }

    #[test]
    fn assert_character_size() {
        let config = EditorConfig::new();
        let result = config.character_size();
        let expected: u16 = 14;
        assert_eq!(result, expected);
    }

    #[test]
    fn assert_current_theme() {
        let config = EditorConfig::new();
        let result = config.current_theme().to_owned();
        let expected = "railscasts".to_owned();
        assert_eq!(result, expected);
    }

    #[test]
    fn assert_margin_left() {
        let config = EditorConfig::new();
        let result = config.margin_left();
        let expected: u16 = 10;
        assert_eq!(result, expected);
    }

    #[test]
    fn assert_margin_top() {
        let config = EditorConfig::new();
        let result = config.margin_top();
        let expected: u16 = 10;
        assert_eq!(result, expected);
    }
}
