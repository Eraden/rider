#[derive(Debug, Clone)]
pub struct EditorConfig {
    pub character_size: u16,
    pub font_path: String,
}

#[derive(Debug, Clone)]
pub struct Config {
    pub width: u32,
    pub height: u32,
    pub editor_config: EditorConfig,
}

impl Config {
    pub fn new() -> Self {
        Self {
            width: 1024,
            height: 860,
            editor_config: EditorConfig {
                character_size: 24,
                font_path: "./assets/fonts/hinted-ElaineSans-Medium.ttf".to_string(),
            },
        }
    }
}
