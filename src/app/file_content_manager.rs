use crate::app::AppState;
use crate::renderer::Renderer;
use crate::ui::caret::Caret;
use crate::ui::caret::CaretPosition;
use crate::ui::file::editor_file::EditorFile;
use crate::ui::get_text_character_rect;
use crate::ui::text_character::TextCharacter;
use sdl2::rect::Point;
use sdl2::rect::Rect;

fn get_character_at(app_state: &mut AppState, index: usize) -> Option<&TextCharacter> {
    match app_state.current_file() {
        None => return None,
        Some(f) => f,
    }
    .get_character_at(index)
}

fn current_file_path(app_state: &mut AppState) -> String {
    match app_state.current_file() {
        Some(f) => f.path(),
        _ => String::new(),
    }
}

pub fn delete_front(app_state: &mut AppState) {
    let mut buffer: String = if let Some(file) = app_state.current_file() {
        file
    } else {
        return;
    }
    .buffer();
    let position: CaretPosition = app_state.caret().position().clone();
    if position.text_position() == 0 {
        return;
    }
    let c: char = buffer.chars().collect::<Vec<char>>()[position.text_position() - 1];
    buffer.remove(position.text_position() - 1);
    let position = match c {
        '\n' if position.text_position() > 0 && position.line_number() > 0 => {
            position.moved(-1, -1, 0)
        }
        '\n' => position.clone(),
        _ if position.text_position() > 0 => position.moved(-1, 0, 0),
        _ => position.moved(0, 0, 0),
    };

    let move_to = match get_character_at(app_state, position.text_position()) {
        Some(character) => {
            let dest: &Rect = character.dest();
            Some((position, Point::new(dest.x(), dest.y())))
        }
        _ => None,
    };
    match move_to {
        Some((position, point)) => app_state.caret_mut().move_caret(position, point),
        None => app_state.caret_mut().reset_caret(),
    };
    let new_file = EditorFile::new(
        current_file_path(app_state),
        buffer,
        app_state.config().clone(),
    );
    app_state.replace_current_file(new_file);
}

pub fn delete_back(app_state: &mut AppState) {
    let file: &EditorFile = if let Some(file) = app_state.current_file() {
        file
    } else {
        return;
    };
    let mut buffer: String = file.buffer();
    let position: usize = app_state.caret().text_position();
    if position >= buffer.len() {
        return;
    }
    buffer.remove(position);
    let new_file = EditorFile::new(file.path(), buffer, app_state.config().clone());
    app_state.replace_current_file(new_file);
}

pub fn insert_text(app_state: &mut AppState, text: String, renderer: &mut Renderer) {
    let mut buffer: String = if let Some(file) = app_state.current_file() {
        file
    } else {
        return;
    }
    .buffer();
    let current = match get_character_at(app_state, app_state.caret().text_position()) {
        Some(c) => c,
        _ => return,
    };
    let mut pos = Point::new(current.dest().x(), current.dest().y());
    let mut position: CaretPosition = app_state.caret().position().clone();
    for character in text.chars() {
        buffer.insert(position.text_position(), character);
        if let Some(rect) = get_text_character_rect(character, renderer) {
            pos = pos + Point::new(rect.width() as i32, 0);
            position = position.moved(1, 0, 0);
            app_state.caret_mut().move_caret(position, pos.clone());
        }
    }

    let new_file = EditorFile::new(
        current_file_path(app_state),
        buffer,
        app_state.config().clone(),
    );

    app_state.replace_current_file(new_file);
}

pub fn insert_new_line(app_state: &mut AppState, renderer: &mut Renderer) {
    let mut buffer: String = if let Some(file) = app_state.current_file() {
        file
    } else {
        return;
    }
    .buffer();
    let current = match get_character_at(app_state, app_state.caret().text_position()) {
        Some(c) => c,
        _ => return,
    };
    let mut pos = Point::new(current.dest().x(), current.dest().y());
    let mut position: CaretPosition = app_state.caret().position().clone();
    buffer.insert(position.text_position(), '\n');
    if let Some(rect) = get_text_character_rect('\n', renderer) {
        pos = Point::new(
            app_state.config().editor_left_margin(),
            pos.y() + rect.height() as i32,
        );
        position = position.moved(0, 1, 0);
        app_state.caret_mut().move_caret(position, pos.clone());
    }

    let new_file = EditorFile::new(
        current_file_path(app_state),
        buffer,
        app_state.config().clone(),
    );
    app_state.replace_current_file(new_file);
}
