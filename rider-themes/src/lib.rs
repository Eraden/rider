extern crate serde;
extern crate serde_json;
#[macro_use]
extern crate serde_derive;

pub mod caret_color;
pub mod code_highlighting_color;
pub mod diff_color;
pub mod images;
pub mod predef;
pub mod serde_color;
pub mod theme;
pub mod theme_config;

pub use crate::caret_color::CaretColor;
pub use crate::code_highlighting_color::CodeHighlightingColor;
pub use crate::diff_color::DiffColor;
pub use crate::images::ThemeImages;
pub use crate::serde_color::SerdeColor;
pub use crate::theme::Theme;
pub use crate::theme_config::ThemeConfig;
