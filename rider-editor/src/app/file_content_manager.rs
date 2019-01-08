use crate::app::*;
use crate::renderer::Renderer;
use crate::ui::*;
use sdl2::rect::{Point, Rect};
use std::sync::*;

fn current_file_path(file_editor: &mut FileEditor) -> String {
    file_editor
        .file()
        .map_or_else(|| String::new(), |f| f.path())
}

pub fn delete_front(file_editor: &mut FileEditor, renderer: &mut Renderer) {
    let mut buffer: String = if let Some(file) = file_editor.file() {
        file
    } else {
        return;
    }
    .buffer();
    let position: CaretPosition = file_editor.caret().position().clone();
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
    let move_to = file_editor
        .file()
        .and_then(|f| f.get_character_at(file_editor.caret().text_position()))
        .and_then(|character| {
            let dest: Rect = character.dest();
            Some((position, Point::new(dest.x(), dest.y())))
        });
    match move_to {
        Some((position, point)) => file_editor.caret_mut().move_caret(position, point),
        None => file_editor.caret_mut().reset_caret(),
    };
    let mut new_file = EditorFile::new(
        current_file_path(file_editor),
        buffer,
        file_editor.config().clone(),
    );
    new_file.prepare_ui(renderer);
    file_editor.replace_current_file(new_file);
}

pub fn delete_back(file_editor: &mut FileEditor, renderer: &mut Renderer) {
    let file: &EditorFile = if let Some(file) = file_editor.file() {
        file
    } else {
        return;
    };
    let mut buffer: String = file.buffer();
    let position: usize = file_editor.caret().text_position();
    if position >= buffer.len() {
        return;
    }
    buffer.remove(position);
    let mut new_file = EditorFile::new(file.path(), buffer, file_editor.config().clone());
    new_file.prepare_ui(renderer);
    file_editor.replace_current_file(new_file);
}

pub fn insert_text(file_editor: &mut FileEditor, text: String, renderer: &mut Renderer) {
    let mut buffer: String = file_editor.file().map_or(String::new(), |f| f.buffer());
    if buffer.is_empty() {
        return;
    }

    let current = match file_editor
        .file()
        .and_then(|file| file.get_character_at(file_editor.caret().text_position()))
    {
        Some(c) => c,
        _ => return,
    };
    let mut pos = Point::new(current.dest().x(), current.dest().y());
    let mut position: CaretPosition = file_editor.caret().position().clone();
    for character in text.chars() {
        buffer.insert(position.text_position(), character);
        if let Some(rect) = get_text_character_rect(character, renderer) {
            pos = pos + Point::new(rect.width() as i32, 0);
            position = position.moved(1, 0, 0);
            file_editor.caret_mut().move_caret(position, pos.clone());
        }
    }

    let mut new_file = EditorFile::new(
        file_editor.file().map_or(String::new(), |f| f.path()),
        buffer,
        file_editor.config().clone(),
    );
    new_file.prepare_ui(renderer);
    file_editor.replace_current_file(new_file);
}

pub fn insert_new_line(file_editor: &mut FileEditor, renderer: &mut Renderer) {
    let mut buffer: String = if let Some(file) = file_editor.file() {
        file
    } else {
        return;
    }
    .buffer();
    let current = match file_editor
        .file()
        .and_then(|file| file.get_character_at(file_editor.caret().text_position()))
    {
        Some(c) => c,
        _ => return,
    };
    let mut pos = Point::new(current.dest().x(), current.dest().y());
    let mut position: CaretPosition = file_editor.caret().position().clone();
    buffer.insert(position.text_position(), '\n');
    if let Some(rect) = get_text_character_rect('\n', renderer) {
        pos = Point::new(
            file_editor.config().read().unwrap().editor_left_margin(),
            pos.y() + rect.height() as i32,
        );
        position = position.moved(0, 1, 0);
        file_editor.caret_mut().move_caret(position, pos.clone());
    }

    let mut new_file = EditorFile::new(
        current_file_path(file_editor),
        buffer,
        Arc::clone(file_editor.config()),
    );
    new_file.prepare_ui(renderer);
    file_editor.replace_current_file(new_file);
}
