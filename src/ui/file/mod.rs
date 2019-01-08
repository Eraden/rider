use sdl2::rect::Rect;

pub mod editor_file;
pub mod editor_file_section;
pub mod editor_file_token;

pub use crate::ui::file::editor_file::*;
pub use crate::ui::file::editor_file_section::*;
pub use crate::ui::file::editor_file_token::*;
use crate::ui::TextCharacter;

pub trait TextCollection {
    fn get_character_at(&self, index: usize) -> Option<TextCharacter>;

    fn get_line(&self, line: &usize) -> Option<Vec<&TextCharacter>>;

    fn get_last_at_line(&self, line: usize) -> Option<TextCharacter>;
}

pub trait TextWidget {
    fn full_rect(&self) -> Rect;
}
