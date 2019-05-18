pub mod app_state;
pub mod application;
pub mod caret_manager;
pub mod file_content_manager;

pub use crate::app::app_state::*;
pub use crate::app::application::*;
pub use crate::app::caret_manager::*;
pub use crate::app::file_content_manager::*;

pub trait Resize {
    fn resize_element(&mut self);
}
