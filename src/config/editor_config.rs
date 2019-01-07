use crate::config::directories;

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
