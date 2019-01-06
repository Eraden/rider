use crate::app::AppState;
use crate::ui::*;
use sdl2::rect::{Point, Rect};

pub fn move_caret_right(file_editor: &mut FileEditor) {
    let file: &EditorFile = match file_editor.file() {
        None => return,
        Some(f) => f,
    };
    let line = match file.get_character_at(file_editor.caret().text_position()) {
        Some(ref t) if t.is_new_line() => file_editor.caret().line_number().clone() + 1,
        Some(_) => file_editor.caret().line_number().clone(),
        None => 0,
    };

    let characters: Vec<&TextCharacter> = match file.get_line(&line) {
        None => return,
        Some(characters) => characters,
    };
    let mut idx = 0;
    for (i, c) in characters.iter().enumerate() {
        if c.position() == file_editor.caret().text_position() {
            idx = i + 1;
            break;
        }
    }
    let text_character: &TextCharacter = match characters.get(idx) {
        Some(text_character) => text_character,
        None => return,
    };
    let line = line - file_editor.caret().line_number();
    let pos = file_editor.caret().position().moved(1, line as i32, 0);
    let mut d: Rect = text_character.dest().clone();
    if text_character.is_new_line() && idx > 0 {
        let prev: &TextCharacter = match characters.get(idx - 1) {
            Some(c) => c,
            _ => return,
        };
        d = prev.dest().clone();
        d.set_x(d.x() + d.width() as i32);
    }
    file_editor
        .caret_mut()
        .move_caret(pos, Point::new(d.x(), d.y()));
}

pub fn move_caret_left(file_editor: &mut FileEditor) {
    let _file: &EditorFile = match file_editor.file() {
        None => return,
        Some(f) => f,
    };
    let _line = file_editor.caret().line_number();
}
