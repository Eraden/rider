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
    file_editor.caret_mut().move_caret(p, d.top_left());
}

pub fn move_caret_left(file_editor: &mut FileEditor) {
    let file: &EditorFile = match file_editor.file() {
        None => return,
        Some(f) => f,
    };
    if file_editor.caret().text_position() == 0 {
        return;
    }
    let text_character: TextCharacter =
        match file.get_character_at(file_editor.caret().text_position() - 1) {
            Some(text_character) => text_character,
            None => return, // EOF
        };
    let pos = file_editor.caret().position();
    let character_destination = text_character.dest().clone();
    let p = pos.moved(-1, 0, 0);
    file_editor
        .caret_mut()
        .move_caret(p, character_destination.top_left());
}

pub fn move_caret_down(file_editor: &mut FileEditor) {
    let file: &EditorFile = match file_editor.file() {
        None => return,
        Some(f) => f,
    };
    if file_editor.caret().text_position() == 0 {
        return;
    }
    let current_line_number = file_editor.caret().line_number();
    let mut next_line_position = 0;
    let mut desired_line_position = 0;
    let mut text_character: Option<&TextCharacter> = None;
    for c in file.iter_char() {
        match c.line() {
            line if c.position() < file_editor.caret().text_position()
                && current_line_number == line =>
            {
                desired_line_position += 1;
            }
            line if line == current_line_number + 1 => {
                text_character = Some(c);
                if next_line_position == desired_line_position {
                    break;
                }
                next_line_position += 1;
            }
            line if line == current_line_number + 2 => {
                break;
            }
            _ => {}
        }
    }
    let text_character: &TextCharacter = match text_character {
        Some(text_character) => text_character,
        None => return, // EOF
    };
    let character_destination = text_character.dest().clone();
    let pos = text_character.position().clone();
    file_editor.caret_mut().move_caret(
        CaretPosition::new(pos, current_line_number + 1, next_line_position),
        character_destination.top_left(),
    );
}

#[cfg(test)]
mod test_move_right {
    use super::*;
    use crate::renderer::managers::FontDetails;
    use crate::renderer::managers::TextDetails;
    use crate::renderer::renderer::Renderer;
    use crate::tests::support;
    use rider_config::config::Config;
    use rider_config::ConfigHolder;
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
    fn assert_move_with_no_file() {
        let config = support::build_config();
        let mut editor = FileEditor::new(config);

        assert_eq!(move_caret_right(&mut editor), ());
    }

    #[test]
    fn assert_move_caret_with_empty_file() {
        let config = support::build_config();
        let mut renderer = RendererMock::new(config.clone());
        let mut editor = FileEditor::new(config.clone());
        let mut file = EditorFile::new("test.txt".to_owned(), "".to_owned(), config);
        file.prepare_ui(&mut renderer);
        editor.open_file(file);
        editor.prepare_ui(&mut renderer);
        editor.move_caret(MoveDirection::Left);

        assert_eq!(move_caret_right(&mut editor), ());
    }

    #[test]
    fn assert_move_caret_with_filled_file() {
        let config = support::build_config();
        let mut renderer = RendererMock::new(config.clone());
        let mut editor = FileEditor::new(config.clone());
        let mut file = EditorFile::new("test.txt".to_owned(), "hello".to_owned(), config);
        file.prepare_ui(&mut renderer);
        editor.open_file(file);
        editor.prepare_ui(&mut renderer);
        editor.move_caret(MoveDirection::Left);

        assert_eq!(move_caret_right(&mut editor), ());
    }
}

#[cfg(test)]
mod test_move_left {
    use super::*;
    use crate::renderer::managers::FontDetails;
    use crate::renderer::managers::TextDetails;
    use crate::renderer::renderer::Renderer;
    use crate::tests::support;
    use rider_config::config::Config;
    use rider_config::ConfigHolder;
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

        #[cfg_attr(tarpaulin, skip)]
        fn load_image(&mut self, _path: String) -> Result<Rc<Texture>, String> {
            unimplemented!()
        }

        fn load_text_tex(
            &mut self,
            _details: &mut TextDetails,
            _font_details: FontDetails,
        ) -> Result<Rc<Texture>, String> {
            Err("skip render character".to_owned())
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
    fn assert_move_caret_without_file() {
        let config = support::build_config();
        let mut editor = FileEditor::new(config);

        assert_eq!(move_caret_left(&mut editor), ());
    }

    #[test]
    fn assert_move_caret_with_empty_file() {
        let config = support::build_config();
        let mut renderer = RendererMock::new(config.clone());
        let mut editor = FileEditor::new(config.clone());
        let mut file = EditorFile::new("test.txt".to_owned(), "".to_owned(), config);
        file.prepare_ui(&mut renderer);
        editor.open_file(file);
        editor.prepare_ui(&mut renderer);
        editor.move_caret(MoveDirection::Right);

        assert_eq!(move_caret_left(&mut editor), ());
    }

    #[test]
    fn assert_move_caret_with_filled_file() {
        let config = support::build_config();
        let mut renderer = RendererMock::new(config.clone());
        let mut editor = FileEditor::new(config.clone());
        let mut file = EditorFile::new("test.txt".to_owned(), "hello".to_owned(), config);
        file.prepare_ui(&mut renderer);
        editor.open_file(file);
        editor.prepare_ui(&mut renderer);
        editor.move_caret(MoveDirection::Right);

        assert_eq!(move_caret_left(&mut editor), ());
    }
}
