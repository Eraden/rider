use crate::directories::*;
use crate::EditorConfig;
use crate::ScrollConfig;
use rider_lexers::Language;
use rider_themes::Theme;
use std::collections::HashMap;
use std::fs;

pub type LanguageMapping = HashMap<String, Language>;

#[derive(Debug, Clone)]
pub struct Config {
    width: u32,
    height: u32,
    menu_height: u16,
    editor_config: EditorConfig,
    theme: Theme,
    extensions_mapping: LanguageMapping,
    scroll: ScrollConfig,
}

impl Config {
    pub fn new() -> Self {
        let editor_config = EditorConfig::new();
        let mut extensions_mapping = HashMap::new();
        extensions_mapping.insert(".".to_string(), Language::PlainText);
        extensions_mapping.insert("txt".to_string(), Language::PlainText);
        extensions_mapping.insert("rs".to_string(), Language::Rust);

        Self {
            width: 1024,
            height: 860,
            menu_height: 60,
            theme: Config::load_theme(editor_config.current_theme().clone()),
            editor_config,
            extensions_mapping,
            scroll: ScrollConfig::new(),
        }
    }

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn set_width(&mut self, w: u32) {
        self.width = w;
    }

    pub fn height(&self) -> u32 {
        self.height
    }

    pub fn set_height(&mut self, h: u32) {
        self.height = h;
    }

    pub fn editor_config(&self) -> &EditorConfig {
        &self.editor_config
    }

    pub fn theme(&self) -> &Theme {
        &self.theme
    }

    pub fn menu_height(&self) -> u16 {
        self.menu_height
    }

    pub fn editor_top_margin(&self) -> i32 {
        (self.menu_height() as i32) + (self.editor_config().margin_top() as i32)
    }

    pub fn editor_left_margin(&self) -> i32 {
        self.editor_config().margin_left() as i32
    }

    pub fn extensions_mapping(&self) -> &LanguageMapping {
        &self.extensions_mapping
    }

    pub fn scroll(&self) -> &ScrollConfig {
        &self.scroll
    }

    pub fn scroll_mut(&mut self) -> &mut ScrollConfig {
        &mut self.scroll
    }
}

impl Config {
    pub fn load_theme(theme_name: String) -> Theme {
        let home_dir = dirs::config_dir().unwrap();
        let mut config_dir = home_dir.clone();
        config_dir.push("rider");
        fs::create_dir_all(&config_dir)
            .unwrap_or_else(|_| panic!("Cannot create config directory"));
        Self::load_theme_content(format!("{}.json", theme_name).as_str())
    }

    fn load_theme_content(file_name: &str) -> Theme {
        let mut config_file = themes_dir();
        config_file.push(file_name);
        let contents = match fs::read_to_string(&config_file) {
            Ok(s) => s,
            Err(_) => fs::read_to_string(&config_file)
                .unwrap_or_else(|_| panic!("Failed to load theme config file")),
        };
        serde_json::from_str(&contents).unwrap_or_default()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn must_return_language_mapping() {
        let config = Config::new();

        let mapping = config.extensions_mapping();
        {
            let mut keys: Vec<String> = mapping.keys().map(|s| s.to_string()).collect();
            let mut expected: Vec<String> =
                vec![".".to_string(), "txt".to_string(), "rs".to_string()];
            keys.sort();
            expected.sort();
            assert_eq!(keys, expected);
        }
        {
            let mut keys: Vec<Language> = mapping.values().map(|s| s.clone()).collect();
            let mut expected: Vec<Language> =
                vec![Language::PlainText, Language::PlainText, Language::Rust];
            keys.sort();
            expected.sort();
            assert_eq!(keys, expected);
        }
    }

    #[test]
    fn assert_scroll() {
        let config = Config::new();
        let result = config.scroll();
        let expected = ScrollConfig::new();
        assert_eq!(result.clone(), expected);
    }

    #[test]
    fn assert_scroll_mut() {
        let mut config = Config::new();
        let result = config.scroll_mut();
        result.set_margin_right(1236);
        let mut expected = ScrollConfig::new();
        expected.set_margin_right(1236);
        assert_eq!(result.clone(), expected);
    }
}

#[cfg(test)]
mod test_getters {
    use super::*;

    #[test]
    fn assert_width() {
        let config = Config::new();
        let result = config.width();
        let expected = 1024;
        assert_eq!(result, expected);
    }

    #[test]
    fn assert_height() {
        let config = Config::new();
        let result = config.height();
        let expected = 860;
        assert_eq!(result, expected);
    }

    //    #[test]
    //    fn assert_editor_config() {
    //        let config = Config::new();
    //        let result = config.editor_config();
    //        let expected = 1;
    //        assert_eq!(result, expected);
    //    }

    //    #[test]
    //    fn assert_theme() {
    //        let config = Config::new();
    //        let result = config.theme();
    //        let expected = 1;
    //        assert_eq!(result, expected);
    //    }

    #[test]
    fn assert_menu_height() {
        let config = Config::new();
        let result = config.menu_height();
        let expected = 60;
        assert_eq!(result, expected);
    }

    #[test]
    fn assert_editor_top_margin() {
        let config = Config::new();
        let result = config.editor_top_margin();
        let expected = config.menu_height() as i32 + config.editor_config().margin_top() as i32;
        assert_eq!(result, expected);
    }

    #[test]
    fn assert_editor_left_margin() {
        let config = Config::new();
        let result = config.editor_left_margin();
        let expected = 10;
        assert_eq!(result, expected);
    }

    #[test]
    fn assert_extensions_mapping() {
        let config = Config::new();
        let mut result: Vec<String> = config
            .extensions_mapping()
            .keys()
            .map(|s| s.to_owned())
            .collect();
        result.sort();
        let mut expected: Vec<String> = vec!["rs".to_string(), "txt".to_string(), ".".to_string()];
        expected.sort();
        assert_eq!(result, expected);
    }

}