use crate::app::*;
use crate::renderer::renderer::Renderer;
use crate::ui::*;
use sdl2::rect::Point;
use std::sync::*;

pub fn current_file_path(file_editor: &mut FileEditor) -> String {
    file_editor
        .file()
        .map_or_else(|| String::new(), |f| f.path())
}

#[cfg_attr(tarpaulin, skip)]
pub fn delete_front<R>(file_editor: &mut FileEditor, renderer: &mut R)
where
    R: ConfigHolder + CharacterSizeManager + Renderer,
{
    let mut buffer: String = match file_editor.file() {
        Some(file) => file.buffer(),
        _ => return,
    };
    let position: CaretPosition = file_editor.caret().position().clone();
    if position.text_position() == 0 {
        return;
    }
    let c: char = buffer.chars().collect::<Vec<char>>()[position.text_position() - 1];
    buffer.remove(position.text_position() - 1);
    let position = match c {
        '\n' if !position.is_first() => position.moved(-1, -1, 0),
        '\n' => position.clone(),
        _ if position.text_position() > 0 => position.moved(-1, 0, 0),
        _ => position.moved(0, 0, 0),
    };
    let move_to = file_editor
        .file()
        .and_then(|f| f.get_character_at(position.text_position()))
        .map(|character| character.dest())
        .map(|dest| (position, Point::new(dest.x(), dest.y())));
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

#[cfg_attr(tarpaulin, skip)]
pub fn delete_back<R>(file_editor: &mut FileEditor, renderer: &mut R)
where
    R: ConfigHolder + CharacterSizeManager + Renderer,
{
    let file: &EditorFile = match file_editor.file() {
        Some(file) => file,
        None => return,
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

pub fn insert_text<R>(file_editor: &mut FileEditor, text: String, renderer: &mut R)
where
    R: ConfigHolder + CharacterSizeManager + Renderer,
{
    let mut buffer: String = match file_editor.file() {
        Some(f) => f.buffer(),
        None => return,
    };

    let maybe_character = file_editor
        .file()
        .and_then(|file| file.get_character_at(file_editor.caret().text_position()));

    let mut pos = match maybe_character {
        Some(ref current) => current.dest().top_left(),
        None => Point::new(0, 0),
    };
    let mut position: CaretPosition = file_editor.caret().position().clone();
    for c in text.chars() {
        buffer.insert(position.text_position(), c);
        let rect = renderer.load_character_size(c);
        pos = pos + Point::new(rect.width() as i32, 0);
        position = position.moved(1, 0, 0);
        file_editor.caret_mut().move_caret(position, pos.clone());
    }

    let mut new_file = EditorFile::new(
        file_editor.file().map_or(String::new(), |f| f.path()),
        buffer,
        file_editor.config().clone(),
    );
    new_file.prepare_ui(renderer);
    file_editor.replace_current_file(new_file);
}

pub fn insert_new_line<R>(file_editor: &mut FileEditor, renderer: &mut R)
where
    R: ConfigHolder + CharacterSizeManager + Renderer,
{
    let mut buffer: String = match file_editor.file() {
        Some(file) => file.buffer(),
        None => return,
    };

    let maybe_character = file_editor
        .file()
        .and_then(|file| file.get_character_at(file_editor.caret().text_position()));
    let mut pos = match maybe_character {
        None => Point::new(0, 0),
        Some(current) => current.dest().top_left(),
    };
    let mut position: CaretPosition = file_editor.caret().position().clone();
    buffer.insert(position.text_position(), '\n');
    let rect = renderer.load_character_size('\n');
    pos = Point::new(0, pos.y() + rect.height() as i32);
    position = position.moved(1, 1, -(position.line_position() as i32));
    file_editor.caret_mut().move_caret(position, pos.clone());

    let mut new_file = EditorFile::new(
        current_file_path(file_editor),
        buffer,
        Arc::clone(file_editor.config()),
    );
    new_file.prepare_ui(renderer);
    file_editor.replace_current_file(new_file);
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::renderer::managers::FontDetails;
    use crate::renderer::managers::TextDetails;
    use crate::tests::support;
    use sdl2::rect::Rect;
    use sdl2::render::Texture;
    use sdl2::ttf::Font;
    use std::rc::Rc;
    use std::sync::Arc;
    use std::sync::RwLock;

    struct RendererMock {
        pub config: Arc<RwLock<Config>>,
    }

    impl RendererMock {
        pub fn new(config: Arc<RwLock<Config>>) -> Self {
            Self { config }
        }
    }

    impl Renderer for RendererMock {
        #[cfg_attr(tarpaulin, skip)]
        fn load_font(&mut self, _details: FontDetails) -> Rc<Font> {
            unimplemented!()
        }

        fn load_text_tex(
            &mut self,
            _details: &mut TextDetails,
            _font_details: FontDetails,
        ) -> Result<Rc<Texture>, String> {
            Err("skip render character".to_owned())
        }

        #[cfg_attr(tarpaulin, skip)]
        fn load_image(&mut self, _path: String) -> Result<Rc<Texture>, String> {
            unimplemented!()
        }
    }

    impl ConfigHolder for RendererMock {
        fn config(&self) -> &Arc<RwLock<Config>> {
            &self.config
        }
    }

    impl CharacterSizeManager for RendererMock {
        fn load_character_size(&mut self, c: char) -> Rect {
            match c {
                '\n' => Rect::new(0, 0, 12, 13),
                _ => Rect::new(0, 0, 14, 15),
            }
        }
    }

    #[test]
    fn must_return_empty_string_when_no_file() {
        let config = support::build_config();
        let mut editor = FileEditor::new(config);
        let result = current_file_path(&mut editor);
        assert_eq!(result, String::new());
    }

    #[test]
    fn must_return_path_string_when_file_was_set() {
        let config = support::build_config();
        let mut editor = FileEditor::new(Arc::clone(&config));
        let file = EditorFile::new(
            "/foo/bar".to_owned(),
            "hello world".to_owned(),
            Arc::clone(&config),
        );
        editor.open_file(file);
        let result = current_file_path(&mut editor);
        assert_eq!(result, "/foo/bar".to_owned());
    }

    //##################################################
    // insert text
    //##################################################

    #[test]
    fn assert_insert_text_without_file() {
        let config = support::build_config();
        let mut renderer = RendererMock::new(config.clone());
        let mut widget = FileEditor::new(config.clone());
        widget.prepare_ui(&mut renderer);
        widget.insert_text("foo".to_owned(), &mut renderer);
        let expected = CaretPosition::new(0, 0, 0);
        assert_eq!(widget.caret().position(), &expected);
    }

    #[test]
    fn assert_insert_text_to_empty_file() {
        let config = support::build_config();
        let mut renderer = RendererMock::new(config.clone());
        let mut widget = FileEditor::new(config.clone());
        let file = EditorFile::new("".to_owned(), "".to_owned(), config.clone());
        widget.open_file(file);
        widget.prepare_ui(&mut renderer);
        widget.insert_text("foo".to_owned(), &mut renderer);
        let expected = CaretPosition::new(3, 0, 0);
        assert_eq!(widget.caret().position(), &expected);
    }

    #[test]
    fn assert_insert_text_to_file_without_new_line() {
        let config = support::build_config();
        let mut renderer = RendererMock::new(config.clone());
        let mut widget = FileEditor::new(config.clone());
        let mut file = EditorFile::new("".to_owned(), "bar".to_owned(), config.clone());
        file.prepare_ui(&mut renderer);
        widget.open_file(file);
        widget.prepare_ui(&mut renderer);
        widget.insert_text("foo".to_owned(), &mut renderer);
        let expected = CaretPosition::new(3, 0, 0);
        assert_eq!(widget.caret().position(), &expected);
        assert_eq!(widget.file().is_some(), true);
        let buffer = widget.file().unwrap().buffer();
        let expected = "foobar";
        assert_eq!(buffer, expected);
    }

    #[test]
    fn assert_insert_text_to_file_with_new_line() {
        let config = support::build_config();
        let mut renderer = RendererMock::new(config.clone());
        let mut widget = FileEditor::new(config.clone());
        let mut file = EditorFile::new("".to_owned(), "bar\n".to_owned(), config.clone());
        file.prepare_ui(&mut renderer);
        widget.open_file(file);
        widget.prepare_ui(&mut renderer);
        widget.insert_text("foo".to_owned(), &mut renderer);
        let expected = CaretPosition::new(3, 0, 0);
        assert_eq!(widget.caret().position(), &expected);
        assert_eq!(widget.file().is_some(), true);
        let buffer = widget.file().unwrap().buffer();
        let expected = "foobar\n";
        assert_eq!(buffer, expected);
    }

    #[test]
    fn assert_insert_text_to_file_with_new_line_with_caret_at_new_line() {
        let config = support::build_config();
        let mut renderer = RendererMock::new(config.clone());
        let mut widget = FileEditor::new(config.clone());
        let mut file = EditorFile::new("".to_owned(), "old content\n".to_owned(), config.clone());
        file.prepare_ui(&mut renderer);
        widget.open_file(file);
        widget.prepare_ui(&mut renderer);
        widget.set_caret_to_end_of_line(0);
        widget.insert_text("new content".to_owned(), &mut renderer);
        let expected = CaretPosition::new(22, 0, 0);
        assert_eq!(widget.caret().position(), &expected);
        assert_eq!(widget.file().is_some(), true);
        let buffer = widget.file().unwrap().buffer();
        let expected = "old contentnew content\n";
        assert_eq!(buffer, expected);
    }

    //##################################################
    // insert new line
    //##################################################

    #[test]
    fn assert_insert_new_line_without_file() {
        let config = support::build_config();
        let mut renderer = RendererMock::new(config.clone());
        let mut widget = FileEditor::new(config.clone());
        widget.prepare_ui(&mut renderer);
        widget.insert_new_line(&mut renderer);
        let expected = CaretPosition::new(0, 0, 0);
        assert_eq!(widget.caret().position(), &expected);
        let expected = Rect::new(0, 0, 6, 15);
        assert_eq!(widget.caret().dest(), expected);
    }

    #[test]
    fn assert_insert_new_line_to_empty_file() {
        let config = support::build_config();
        let mut renderer = RendererMock::new(config.clone());
        let mut widget = FileEditor::new(config.clone());
        let file = EditorFile::new("".to_owned(), "".to_owned(), config.clone());
        widget.open_file(file);
        widget.prepare_ui(&mut renderer);
        widget.insert_new_line(&mut renderer);
        let expected = CaretPosition::new(1, 1, 0);
        assert_eq!(widget.caret().position(), &expected);
        let expected = Rect::new(0, 13, 6, 15);
        assert_eq!(widget.caret().dest(), expected);
    }

    #[test]
    fn assert_insert_new_line_to_file_at_beginning() {
        let config = support::build_config();
        let mut renderer = RendererMock::new(config.clone());
        let mut widget = FileEditor::new(config.clone());
        let file = EditorFile::new("".to_owned(), "foo".to_owned(), config.clone());
        widget.open_file(file);
        widget.prepare_ui(&mut renderer);
        widget.insert_new_line(&mut renderer);
        let expected = CaretPosition::new(1, 1, 0);
        assert_eq!(widget.caret().position(), &expected);
        let expected = Rect::new(0, 13, 6, 15);
        assert_eq!(widget.caret().dest(), expected);
        assert_eq!(widget.file().is_some(), true);
        assert_eq!(widget.file().unwrap().buffer(), "\nfoo".to_owned());
    }

    #[test]
    fn assert_insert_new_line_to_file_in_middle() {
        let config = support::build_config();
        let mut renderer = RendererMock::new(config.clone());
        let mut widget = FileEditor::new(config.clone());
        let mut file = EditorFile::new("hello.txt".to_owned(), "abcd".to_owned(), config.clone());
        file.prepare_ui(&mut renderer);
        widget.open_file(file);
        widget.prepare_ui(&mut renderer);
        widget.move_caret(MoveDirection::Right);
        widget.move_caret(MoveDirection::Right);
        widget.insert_new_line(&mut renderer);
        let expected = CaretPosition::new(3, 1, 0);
        assert_eq!(widget.caret().position(), &expected);
        let expected = Rect::new(0, 13, 6, 15);
        assert_eq!(widget.caret().dest(), expected);
        assert_eq!(widget.file().is_some(), true);
        assert_eq!(widget.file().unwrap().buffer(), "ab\ncd".to_owned());
    }
}
