use std::sync::{Arc, RwLock};

pub mod config;
pub(crate) mod creator;
pub mod directories;
pub mod editor_config;

pub use crate::config::config::*;
pub use crate::config::directories::*;
pub use crate::config::editor_config::*;

pub type ConfigAccess = Arc<RwLock<Config>>;

pub trait ConfigHolder {
    fn config(&self) -> &ConfigAccess;
}
