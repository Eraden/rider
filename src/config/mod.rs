use crate::lexer::Language;
use crate::themes::Theme;
use dirs;
use std::collections::HashMap;
use std::fs;

mod creator;
pub mod directories;

type LanguageMapping = HashMap<String, Language>;

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

#[derive(Debug, Clone)]
pub struct Config {
    width: u32,
    height: u32,
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
            menu_height: 60,
            theme: Theme::load(editor_config.current_theme().clone()),
            editor_config,
            extensions_mapping,
        }
    }
    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
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
