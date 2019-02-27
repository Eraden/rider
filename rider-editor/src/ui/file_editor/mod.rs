pub use crate::ui::file_editor::file_editor::*;
use crate::ui::*;
use sdl2::rect::Point;

pub mod file_editor;

pub trait FileAccess {
    fn has_file(&self) -> bool;

    fn file(&self) -> Option<&EditorFile>;

    fn file_mut(&mut self) -> Option<&mut EditorFile>;

    fn open_file(&mut self, file: EditorFile) -> Option<EditorFile>;

    fn drop_file(&mut self) -> Option<EditorFile>;

    fn replace_current_file(&mut self, file: EditorFile);
}

pub trait CaretAccess {
    fn caret(&self) -> &Caret;

    fn caret_mut(&mut self) -> &mut Caret;

    fn move_caret(&mut self, dir: MoveDirection);

    fn set_caret_to_end_of_line(&mut self, line: i32);
}

pub trait ScrollableView {
    fn scroll_by(&mut self, x: i32, y: i32);

    fn scroll(&self) -> Point;
}
