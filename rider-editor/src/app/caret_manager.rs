use crate::ui::*;
use sdl2::rect::{Point, Rect};

pub fn move_caret_right(file_editor: &mut FileEditor) {
    let file: &EditorFile = match file_editor.file() {
        None => return,
        Some(f) => f,
    };
    let c: TextCharacter = match file.get_character_at(file_editor.caret().text_position() + 1) {
        Some(text_character) => text_character,
        None => return, // EOF
    };
    let caret_rect = file_editor.caret().dest().clone();
    let pos = file_editor.caret().position();
    let (d, p): (Rect, CaretPosition) = match (
        c.is_last_in_line(),
        c.is_new_line(),
        c.dest().y() == caret_rect.y(),
    ) {
        (true, true, false) => {
            let prev: TextCharacter = if c.position() != 0 {
                file.get_character_at(c.position() - 1).unwrap_or(c.clone())
            } else {
                c.clone()
            };
            let mut dest = prev.dest().clone();
            dest.set_x(dest.x() + dest.width() as i32);
            (dest, pos.moved(1, 0, 0))
        }
        (false, true, false) => {
            let prev: TextCharacter = if c.position() != 0 {
                file.get_character_at(c.position() - 1).unwrap_or(c.clone())
            } else {
                c.clone()
            };
            let mut dest = prev.dest().clone();
            if !prev.is_new_line() {
                dest.set_x(dest.x() + dest.width() as i32);
            }
            (dest, pos.moved(1, 0, 0))
        }
        (true, false, false) => {
            // move after character, stay on current line
            (c.dest().clone(), pos.moved(1, 0, 0))
        }
        (true, false, true) => {
            // move to new line
            (c.dest().clone(), pos.moved(1, 0, 0))
        }
        _ => (c.dest().clone(), pos.moved(1, 0, 0)),
    };
    file_editor
        .caret_mut()
        .move_caret(p, Point::new(d.x(), d.y()));
}

pub fn move_caret_left(_file_editor: &mut FileEditor) {
    //    let _file: &EditorFile = match file_editor.file() {
    //        None => return,
    //        Some(f) => f,
    //    };
    //    let _line = file_editor.caret().line_number();
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
