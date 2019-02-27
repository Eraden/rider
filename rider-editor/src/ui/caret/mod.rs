#[derive(Clone, Debug, PartialEq)]
pub enum CaretState {
    Bright,
    Blur,
}

#[derive(Clone, Debug, PartialEq)]
pub enum MoveDirection {
    Left,
    Right,
    Up,
    Down,
}

pub mod caret;
pub mod caret_color;
pub mod caret_position;

pub use crate::ui::caret::caret::*;
pub use crate::ui::caret::caret_color::*;
pub use crate::ui::caret::caret_position::*;
