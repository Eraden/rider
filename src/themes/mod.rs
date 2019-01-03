use std::fs;
use std::path::PathBuf;
use std::env;
use sdl2::pixels::Color;
use serde_json;
use serde::ser::{Serialize, Serializer, SerializeSeq, SerializeMap};

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct SerdeColor {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8
}

impl SerdeColor {
    pub fn new(r: u8,g: u8,b: u8,a: u8) -> Self {
        Self { r,g,b,a }
    }
}

impl Into<Color> for SerdeColor {
    fn into(self) -> Color {
        Color {
            r: self.r,
            g: self.g,
            b: self.b,
            a: self.a
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
        Self {
            name: "Default".to_string(),
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

    fn railscasts() -> Self {
        Self {
            name: "railscasts".to_string(),
            background: SerdeColor {
                r: 60,
                g: 60,
                b: 60,
                a: 0
            },
            caret: CaretColor { bright: ThemeConfig {
                color: SerdeColor {
                    r: 0,
                    g: 0,
                    b: 0,
                    a: 0
                },
                italic: false,
                bold: false
            }, blur: ThemeConfig {
                color: SerdeColor {
                    r: 0,
                    g: 0,
                    b: 0,
                    a: 0
                },
                italic: false,
                bold: false
            } },
            code_highlighting: CodeHighlightingColor {
                whitespace: ThemeConfig {
                    color: SerdeColor {
                        r: 0,
                        g: 0,
                        b: 0,
                        a: 0
                    },
                    italic: false,
                    bold: false
                },
                keyword: ThemeConfig {
                    color: SerdeColor {
                        r: 203,
                        g: 120,
                        b: 50,
                        a: 0
                    },
                    italic: false,
                    bold: true
                },
                string: ThemeConfig {
                    color: SerdeColor {
                        r: 0,
                        g: 0,
                        b: 0,
                        a: 0
                    },
                    italic: false,
                    bold: false
                },
                number: ThemeConfig {
                    color: SerdeColor {
                        r: 0,
                        g: 0,
                        b: 0,
                        a: 0
                    },
                    italic: false,
                    bold: false
                },
                identifier: ThemeConfig {
                    color: SerdeColor {
                        r: 0,
                        g: 0,
                        b: 0,
                        a: 0
                    },
                    italic: false,
                    bold: false
                },
                literal: ThemeConfig {
                    color: SerdeColor {
                        r: 0,
                        g: 0,
                        b: 0,
                        a: 0
                    },
                    italic: false,
                    bold: false
                },
                comment: ThemeConfig {
                    color: SerdeColor {
                        r: 188,
                        g: 147,
                        b: 88,
                        a: 0
                    },
                    italic: true,
                    bold: false
                },
                operator: ThemeConfig {
                    color: SerdeColor {
                        r: 0,
                        g: 0,
                        b: 0,
                        a: 0
                    },
                    italic: false,
                    bold: false
                },
                separator: ThemeConfig {
                    color: SerdeColor {
                        r: 0,
                        g: 0,
                        b: 0,
                        a: 0
                    },
                    italic: false,
                    bold: false
                }
            }
        }
    }

    pub fn load(_theme_name: String) -> Self {
        use dirs;
        let home_dir = dirs::config_dir().unwrap();
        let mut config_dir = home_dir.clone();
        config_dir.push("rider/themes");
        fs::create_dir_all(&config_dir)
            .unwrap_or_else(|_| panic!("Cannot create config directory"));
        let theme = Self::load_content(&config_dir, "default.json");
        println!("theme config:\n{:?}", theme);
        theme
    }

    fn load_content(config_dir: &PathBuf, file_name: &str) -> Theme {
        let mut config_file = config_dir.clone();
        config_file.push(file_name);
        let contents = match fs::read_to_string(&config_file) {
            Ok(s) => s,
            Err(_) => {
                let contents = serde_json::to_string_pretty(&Theme::default())
                    .unwrap();
                fs::write(&config_file, contents.clone())
                    .unwrap_or_else(|_| panic!("Failed to crate theme config file"));
                contents.to_string()
            }
        };
        serde_json::from_str(&contents).unwrap_or_default()
    }
}
