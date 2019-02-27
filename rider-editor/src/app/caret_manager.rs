use crate::ui::*;
use sdl2::rect::Point;

pub fn move_caret_right(file_editor: &mut FileEditor) {
    let file: &EditorFile = match file_editor.file() {
        None => return,
        Some(f) => f,
    };
    let c: TextCharacter = match file.get_character_at(file_editor.caret().text_position() + 1) {
        Some(text_character) => text_character,
        None => return, // EOF
    };
    let pos = file_editor.caret().position();
    let d = c.dest().clone();
    let p = pos.moved(1, 0, 0);
    file_editor
        .caret_mut()
        .move_caret(p, Point::new(d.x(), d.y()));
}

pub fn move_caret_left(file_editor: &mut FileEditor) {
    let file: &EditorFile = match file_editor.file() {
        None => return,
        Some(f) => f,
    };
    if file_editor.caret().text_position() == 0 {
        return;
    }
    let c: TextCharacter = match file.get_character_at(file_editor.caret().text_position() - 1) {
        Some(text_character) => text_character,
        None => return, // EOF
    };
    let pos = file_editor.caret().position();
    let d = c.dest().clone();
    let p = pos.moved(-1, 0, 0);
    file_editor
        .caret_mut()
        .move_caret(p, Point::new(d.x(), d.y()));
}

#[cfg(test)]
mod test_move_right {
    use super::*;
    use crate::tests::support;

    #[test]
    fn must_do_nothing() {
        let config = support::build_config();
        let mut editor = FileEditor::new(config);

        assert_eq!(move_caret_right(&mut editor), ());
    }
}

#[cfg(test)]
mod test_move_left {
    use super::*;
    use crate::tests::support;

    #[test]
    fn must_do_nothing() {
        let config = support::build_config();
        let mut editor = FileEditor::new(config);

        assert_eq!(move_caret_left(&mut editor), ());
    }
}
