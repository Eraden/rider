use crate::config::directories::*;
use crate::themes::predef::*;
use crate::themes::*;
use dirs;
use std::fs;
use std::path::PathBuf;

pub fn create() {
    fs::create_dir_all(themes_dir())
        .unwrap_or_else(|_| panic!("Cannot create theme config directory"));
    for theme in default_styles() {
        write_theme(&theme);
    }
}

fn write_theme(theme: &Theme) {
    let mut theme_path = themes_dir();
    theme_path.push(format!("{}.json", theme.name()));
    let contents = serde_json::to_string_pretty(&theme).unwrap();
    fs::write(&theme_path, contents.clone())
        .unwrap_or_else(|_| panic!("Failed to crate theme config file"));
}

fn default_styles() -> Vec<Theme> {
    vec![default::build_theme(), railscasts::build_theme()]
}

//fn default_theme() -> Theme {
//    Theme::default()
//}

//fn railscasts_theme() -> Theme {
//    Theme {
//        name: "railscasts".to_string(),
//        background: SerdeColor {
//            r: 60,
//            g: 60,
//            b: 60,
//            a: 0,
//        },
//        caret: CaretColor {
//            bright: ThemeConfig {
//                color: SerdeColor {
//                    r: 121,
//                    g: 121,
//                    b: 121,
//                    a: 0,
//                },
//                italic: false,
//                bold: false,
//            },
//            blur: ThemeConfig {
//                color: SerdeColor {
//                    r: 21,
//                    g: 21,
//                    b: 21,
//                    a: 0,
//                },
//                italic: false,
//                bold: false,
//            },
//        },
//        code_highlighting: CodeHighlightingColor {
//            whitespace: ThemeConfig {
//                color: SerdeColor {
//                    r: 220,
//                    g: 220,
//                    b: 220,
//                    a: 90,
//                },
//                italic: false,
//                bold: false,
//            },
//            keyword: ThemeConfig {
//                color: SerdeColor {
//                    r: 175,
//                    g: 95,
//                    b: 0,
//                    a: 0,
//                },
//                italic: false,
//                bold: true,
//            },
//            string: ThemeConfig {
//                color: SerdeColor {
//                    r: 135,
//                    g: 175,
//                    b: 95,
//                    a: 0,
//                },
//                italic: false,
//                bold: false,
//            },
//            number: ThemeConfig {
//                color: SerdeColor {
//                    r: 135,
//                    g: 175,
//                    b: 95,
//                    a: 0,
//                },
//                italic: false,
//                bold: false,
//            },
//            identifier: ThemeConfig {
//                color: SerdeColor {
//                    r: 175,
//                    g: 75,
//                    b: 75,
//                    a: 0,
//                },
//                italic: false,
//                bold: false,
//            },
//            literal: ThemeConfig {
//                color: SerdeColor {
//                    r: 121,
//                    g: 121,
//                    b: 121,
//                    a: 0,
//                },
//                italic: false,
//                bold: false,
//            },
//            comment: ThemeConfig {
//                color: SerdeColor {
//                    r: 175,
//                    g: 135,
//                    b: 95,
//                    a: 0,
//                },
//                italic: true,
//                bold: false,
//            },
//            operator: ThemeConfig {
//                color: SerdeColor {
//                    r: 200,
//                    g: 0,
//                    b: 0,
//                    a: 0,
//                },
//                italic: false,
//                bold: false,
//            },
//            separator: ThemeConfig {
//                color: SerdeColor {
//                    r: 221,
//                    g: 221,
//                    b: 221,
//                    a: 0,
//                },
//                italic: false,
//                bold: false,
//            },
//        },
//    }
//}
