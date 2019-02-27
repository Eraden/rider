extern crate rider_lexers;
extern crate rider_themes;

use std::sync::{Arc, RwLock};

pub mod config;
pub mod directories;
pub mod editor_config;
pub mod scroll_config;

pub use crate::config::*;
pub use crate::directories::*;
pub use crate::editor_config::*;
pub use crate::scroll_config::*;

pub type ConfigAccess = Arc<RwLock<Config>>;

pub trait ConfigHolder {
    fn config(&self) -> &ConfigAccess;
}
