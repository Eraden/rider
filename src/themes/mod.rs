use crate::config::directories::*;
use sdl2::pixels::Color;
use serde::ser::{Serialize, SerializeMap, SerializeSeq, Serializer};
use serde_json;
use std::env;
use std::fs;
use std::path::PathBuf;

pub mod caret_color;
pub mod code_highlighting_color;
pub mod config_creator;
pub mod diff_color;
pub mod predef;
pub mod serde_color;
pub mod theme;
pub mod theme_config;

pub use crate::themes::caret_color::CaretColor;
pub use crate::themes::code_highlighting_color::CodeHighlightingColor;
pub use crate::themes::diff_color::DiffColor;
pub use crate::themes::serde_color::SerdeColor;
pub use crate::themes::theme::Theme;
pub use crate::themes::theme_config::ThemeConfig;
