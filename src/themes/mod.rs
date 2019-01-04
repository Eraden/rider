use crate::config::directories::*;
use sdl2::pixels::Color;
use serde::ser::{Serialize, SerializeMap, SerializeSeq, Serializer};
use serde_json;
use std::env;
use std::fs;
use std::path::PathBuf;

pub mod config_creator;

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct SerdeColor {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl SerdeColor {
    pub fn new(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self { r, g, b, a }
    }
}

impl Into<Color> for &SerdeColor {
    fn into(self) -> Color {
        Color {
            r: self.r,
            g: self.g,
            b: self.b,
            a: self.a,
        }
    }
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct ThemeConfig {
    color: SerdeColor,
    italic: bool,
    bold: bool,
}

impl ThemeConfig {
    pub fn color(&self) -> &SerdeColor {
        &self.color
    }

    pub fn italic(&self) -> bool {
        self.italic
    }

    pub fn bold(&self) -> bool {
        self.bold
    }
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct CaretColor {
    bright: ThemeConfig,
    blur: ThemeConfig,
}

impl Default for CaretColor {
    fn default() -> Self {
        Self {
            bright: ThemeConfig {
                color: SerdeColor::new(0, 0, 0, 0),
                italic: false,
                bold: false,
            },
            blur: ThemeConfig {
                color: SerdeColor::new(0, 0, 0, 0),
                italic: false,
                bold: false,
            },
        }
    }
}

impl CaretColor {
    pub fn bright(&self) -> &ThemeConfig {
        &self.bright
    }

    pub fn blur(&self) -> &ThemeConfig {
        &self.blur
    }
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct CodeHighlightingColor {
    whitespace: ThemeConfig,
    keyword: ThemeConfig,
    string: ThemeConfig,
    number: ThemeConfig,
    identifier: ThemeConfig,
    literal: ThemeConfig,
    comment: ThemeConfig,
    operator: ThemeConfig,
    separator: ThemeConfig,
}

impl Default for CodeHighlightingColor {
    fn default() -> Self {
        Self {
            whitespace: ThemeConfig {
                color: SerdeColor::new(0, 0, 0, 0),
                bold: false,
                italic: false,
            },
            keyword: ThemeConfig {
                color: SerdeColor::new(0, 0, 0, 0),
                bold: false,
                italic: false,
            },
            string: ThemeConfig {
                color: SerdeColor::new(0, 0, 0, 0),
                bold: false,
                italic: false,
            },
            number: ThemeConfig {
                color: SerdeColor::new(0, 0, 0, 0),
                bold: false,
                italic: false,
            },
            identifier: ThemeConfig {
                color: SerdeColor::new(0, 0, 0, 0),
                bold: false,
                italic: false,
            },
            literal: ThemeConfig {
                color: SerdeColor::new(0, 0, 0, 0),
                bold: false,
                italic: false,
            },
            comment: ThemeConfig {
                color: SerdeColor::new(0, 0, 0, 0),
                bold: false,
                italic: false,
            },
            operator: ThemeConfig {
                color: SerdeColor::new(0, 0, 0, 0),
                bold: false,
                italic: false,
            },
            separator: ThemeConfig {
                color: SerdeColor::new(0, 0, 0, 0),
                bold: false,
                italic: false,
            },
        }
    }
}

impl CodeHighlightingColor {
    pub fn whitespace(&self) -> &ThemeConfig {
        &self.whitespace
    }

    pub fn keyword(&self) -> &ThemeConfig {
        &self.keyword
    }

    pub fn string(&self) -> &ThemeConfig {
        &self.string
    }

    pub fn number(&self) -> &ThemeConfig {
        &self.number
    }

    pub fn identifier(&self) -> &ThemeConfig {
        &self.identifier
    }

    pub fn literal(&self) -> &ThemeConfig {
        &self.literal
    }

    pub fn comment(&self) -> &ThemeConfig {
        &self.comment
    }

    pub fn operator(&self) -> &ThemeConfig {
        &self.operator
    }

    pub fn separator(&self) -> &ThemeConfig {
        &self.separator
    }
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct Theme {
    name: String,
    background: SerdeColor,
    caret: CaretColor,
    code_highlighting: CodeHighlightingColor,
}

impl Default for Theme {
    fn default() -> Self {
        use crate::themes::config_creator;
        Self {
            name: "default".to_string(),
            background: SerdeColor::new(255, 255, 255, 0),
            caret: CaretColor::default(),
            code_highlighting: CodeHighlightingColor::default(),
        }
    }
}

impl Theme {
    pub fn name(&self) -> &String {
        &self.name
    }

    pub fn background(&self) -> &SerdeColor {
        &self.background
    }

    pub fn caret(&self) -> &CaretColor {
        &self.caret
    }

    pub fn code_highlighting(&self) -> &CodeHighlightingColor {
        &self.code_highlighting
    }

    pub fn load(theme_name: String) -> Self {
        use dirs;
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
