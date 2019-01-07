use crate::config::creator;
use crate::config::EditorConfig;
use crate::lexer::Language;
use crate::themes::Theme;
use dirs;
use std::collections::HashMap;
use std::fs;

pub type LanguageMapping = HashMap<String, Language>;

#[derive(Debug, Clone)]
pub struct Config {
    width: u32,
    height: u32,
    scroll_speed: i32,
    menu_height: u16,
    editor_config: EditorConfig,
    theme: Theme,
    extensions_mapping: LanguageMapping,
}

impl Config {
    pub fn new() -> Self {
        creator::create();
        let editor_config = EditorConfig::new();
        let mut extensions_mapping = HashMap::new();
        extensions_mapping.insert(".".to_string(), Language::PlainText);
        extensions_mapping.insert("txt".to_string(), Language::PlainText);
        extensions_mapping.insert("rs".to_string(), Language::Rust);

        Self {
            width: 1024,
            height: 860,
            scroll_speed: 10,
            menu_height: 60,
            theme: Theme::load(editor_config.current_theme().clone()),
            editor_config,
            extensions_mapping,
        }
    }

    pub fn scroll_speed(&self) -> i32 {
        self.scroll_speed
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
}

#[cfg(test)]
mod tests {
    use crate::config::*;
    use crate::lexer::*;

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
}
