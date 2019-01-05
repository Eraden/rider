use crate::config::directories::themes_dir;
use crate::themes::CaretColor;
use crate::themes::CodeHighlightingColor;
use crate::themes::DiffColor;
use crate::themes::SerdeColor;
use dirs;
use std::fs;

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct Theme {
    name: String,
    background: SerdeColor,
    caret: CaretColor,
    code_highlighting: CodeHighlightingColor,
    diff: DiffColor,
}

impl Default for Theme {
    fn default() -> Self {
        use crate::themes::config_creator;
        Self {
            name: "default".to_string(),
            background: SerdeColor::new(255, 255, 255, 0),
            caret: CaretColor::default(),
            code_highlighting: CodeHighlightingColor::default(),
            diff: DiffColor::default(),
        }
    }
}

impl Theme {
    pub fn new(
        name: String,
        background: SerdeColor,
        caret: CaretColor,
        code_highlighting: CodeHighlightingColor,
        diff: DiffColor,
    ) -> Self {
        Self {
            name,
            background,
            caret,
            code_highlighting,
            diff,
        }
    }

    pub fn name(&self) -> &String {
        &self.name
    }

    pub fn background(&self) -> &SerdeColor {
        &self.background
    }

    pub fn caret(&self) -> &CaretColor {
        &self.caret
    }

    pub fn diff(&self) -> &DiffColor {
        &self.diff
    }

    pub fn code_highlighting(&self) -> &CodeHighlightingColor {
        &self.code_highlighting
    }

    pub fn load(theme_name: String) -> Self {
        let home_dir = dirs::config_dir().unwrap();
        let mut config_dir = home_dir.clone();
        config_dir.push("rider");
        fs::create_dir_all(&config_dir)
            .unwrap_or_else(|_| panic!("Cannot create config directory"));
        Self::load_content(format!("{}.json", theme_name).as_str())
    }

    fn load_content(file_name: &str) -> Theme {
        let mut config_file = themes_dir();
        config_file.push(file_name);
        let contents = match fs::read_to_string(&config_file) {
            Ok(s) => s,
            Err(_) => {
                use crate::themes::config_creator;
                config_creator::create();
                fs::read_to_string(&config_file)
                    .unwrap_or_else(|_| panic!("Failed to load theme config file"))
            }
        };
        serde_json::from_str(&contents).unwrap_or_default()
    }
}
