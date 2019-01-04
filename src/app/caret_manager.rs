use crate::app::AppState;
use crate::ui::file::editor_file::EditorFile;
use crate::ui::text_character::TextCharacter;
use sdl2::rect::{Point, Rect};

pub fn move_caret_right(app_state: &mut AppState) {
    let file: &EditorFile = match app_state.current_file() {
        None => return,
        Some(f) => f,
    };
    let line = match file.get_character_at(app_state.caret().text_position()) {
        Some(t) if t.is_new_line() => app_state.caret().line_number().clone() + 1,
        Some(_) => app_state.caret().line_number().clone(),
        None => 0,
    };

    let characters: Vec<&TextCharacter> = match file.get_line(&line) {
        None => return,
        Some(characters) => characters,
    };
    let mut idx = 0;
    for (i, c) in characters.iter().enumerate() {
        if c.position() == app_state.caret().text_position() {
            idx = i + 1;
            break;
        }
    }
    let text_character: &TextCharacter = match characters.get(idx) {
        Some(text_character) => text_character,
        None => return,
    };
    let line = line - app_state.caret().line_number();
    let pos = app_state.caret().position().moved(1, line as i32, 0);
    let mut d: Rect = text_character.dest().clone();
    if text_character.is_new_line() && idx > 0 {
        let prev = match characters.get(idx - 1) {
            Some(c) => c,
            _ => return,
        };
        d = prev.dest().clone();
        d.set_x(d.x() + d.width() as i32);
    }
    app_state
        .caret_mut()
        .move_caret(pos, Point::new(d.x(), d.y()));
}

pub fn move_caret_left(app_state: &mut AppState) {
    let _file: &EditorFile = match app_state.current_file() {
        None => return,
        Some(f) => f,
    };
    let _line = app_state.caret().line_number();
}
