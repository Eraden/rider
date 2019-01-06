use sdl2::rect::*;
use std::borrow::*;
use std::mem;
use std::rc::Rc;

use crate::app::*;
use crate::app::{UpdateResult as UR, WindowCanvas as WS};
use crate::ui::*;

pub struct FileEditor {
    dest: Rect,
    caret: Caret,
    file: Option<EditorFile>,
    config: Rc<Config>,
}

impl FileEditor {
    pub fn new(dest: Rect, config: Rc<Config>) -> Self {
        Self {
            dest,
            caret: Caret::new(config.clone()),
            file: None,
            config,
        }
    }

    pub fn config(&self) -> &Rc<Config> {
        &self.config
    }

    pub fn caret(&self) -> &Caret {
        &self.caret
    }

    pub fn caret_mut(&mut self) -> &mut Caret {
        &mut self.caret
    }

    pub fn has_file(&self) -> bool {
        self.file.is_some()
    }

    pub fn drop_file(&mut self) -> Option<EditorFile> {
        if self.has_file() {
            let mut file = None;
            mem::swap(&mut self.file, &mut file);
            file
        } else {
            None
        }
    }

    pub fn open_file(&mut self, file: EditorFile) -> Option<EditorFile> {
        let mut file = Some(file);
        mem::swap(&mut self.file, &mut file);
        file
    }

    pub fn file(&self) -> Option<&EditorFile> {
        self.file.as_ref()
    }

    pub fn file_mut(&mut self) -> Option<&mut EditorFile> {
        self.file.as_mut()
    }

    pub fn move_caret(&mut self, dir: MoveDirection) {
        match dir {
            MoveDirection::Left => {}
            MoveDirection::Right => caret_manager::move_caret_right(self),
            MoveDirection::Up => {}
            MoveDirection::Down => {}
        }
    }

    pub fn delete_front(&mut self, renderer: &mut Renderer) {
        file_content_manager::delete_front(self, renderer);
    }

    pub fn delete_back(&mut self, renderer: &mut Renderer) {
        file_content_manager::delete_back(self, renderer);
    }

    pub fn insert_text(&mut self, text: String, renderer: &mut Renderer) {
        file_content_manager::insert_text(self, text, renderer);
    }

    pub fn insert_new_line(&mut self, renderer: &mut Renderer) {
        file_content_manager::insert_new_line(self, renderer);
    }

    pub fn replace_current_file(&mut self, file: EditorFile) {
        self.open_file(file);
    }

    fn is_text_character_clicked(&self, point: &Point) -> bool {
        let context = UpdateContext::ParentPosition(self.render_start_point());
        self.file()
            .map_or(false, |file| file.is_left_click_target(point, &context))
    }

    fn is_editor_clicked(&self, point: &Point) -> bool {
        self.dest
            .contains_point(move_render_point(point.clone(), &self.dest).top_left())
    }

    fn resolve_line_from_point(&self, point: &Point) -> i32 {
        let file = match self.file() {
            Some(f) => f,
            _ => return 0,
        };
        let mut y = point.y() - file.render_position().y();
        if y < 0 {
            y = 0;
        }
        y / (file.line_height() as i32)
    }

    fn set_caret_to_end_of_line(&mut self, line: i32) {
        let file = match self.file_mut() {
            Some(f) => f,
            _ => return,
        };
        let mut line = line;
        while line >= 0 {
            match file.get_last_at_line(line.clone() as usize) {
                Some(text_character) => {
                    let rect = text_character.dest();
                    let position =
                        CaretPosition::new(text_character.position() + 1, line as usize, 0);
                    let p = if text_character.is_last_in_line() && text_character.is_new_line() {
                        rect.top_left()
                    } else {
                        rect.top_right()
                    };
                    self.caret.move_caret(position, p);
                    break;
                }
                _ => {
                    line -= 1;
                }
            }
        }
    }
}

impl Render for FileEditor {
    fn render(&self, canvas: &mut WS, renderer: &mut Renderer, _parent: Parent) -> UR {
        match self.file() {
            Some(file) => file.render(canvas, renderer, Some(self)),
            _ => UR::NoOp,
        };
        self.caret.render(canvas, renderer, Some(self))
    }

    fn prepare_ui(&mut self, renderer: &mut Renderer) {
        self.caret.prepare_ui(renderer);
    }
}

impl Update for FileEditor {
    fn update(&mut self, ticks: i32, context: &UpdateContext) -> UR {
        self.caret.update(ticks, context);
        match self.file_mut() {
            Some(file) => file.update(ticks, context),
            _ => UR::NoOp,
        }
    }
}

impl ClickHandler for FileEditor {
    fn on_left_click(&mut self, point: &Point, _context: &UpdateContext) -> UR {
        let context = UpdateContext::ParentPosition(self.render_start_point());
        if self.is_text_character_clicked(point) {
            let file = if let Some(file) = self.file_mut() {
                file
            } else {
                return UR::NoOp;
            };
            match file.on_left_click(point, &context) {
                UR::MoveCaret(rect, position) => {
                    self.caret
                        .move_caret(position, Point::new(rect.x(), rect.y()));
                }
                _ => {}
            }
        } else {
            self.set_caret_to_end_of_line(self.resolve_line_from_point(point));
        }
        UR::NoOp
    }

    fn is_left_click_target(&self, point: &Point, _context: &UpdateContext) -> bool {
        self.is_text_character_clicked(point) || self.is_editor_clicked(point)
    }
}

impl RenderBox for FileEditor {
    fn render_start_point(&self) -> Point {
        self.dest.top_left()
    }
}

#[cfg(test)]
mod tests {
    use crate::app::*;
    use crate::ui::*;
    use sdl2::rect::*;
    use sdl2::*;
    use std::borrow::*;
    use std::rc::*;

    #[test]
    fn replace_file() {
        let config = Rc::new(Config::new());
        let mut editor = FileEditor::new(Rect::new(0, 0, 100, 100), config.clone());
        let first_file =
            EditorFile::new("./foo.txt".to_string(), "foo".to_string(), config.clone());
        let second_file =
            EditorFile::new("./bar.txt".to_string(), "bar".to_string(), config.clone());
        editor.open_file(first_file.clone());
        let result = editor.open_file(second_file.clone());
        assert_eq!(result.is_some(), true);
        let file = result.as_ref().unwrap();
        assert_eq!(file.path(), first_file.path());
        assert_eq!(file.buffer(), first_file.buffer());
    }

    #[test]
    fn add_text() {
        let config = Rc::new(Config::new());
        let sdl_context = sdl2::init().unwrap();
        let video_subsystem = sdl_context.video().unwrap();
        let window = video_subsystem
            .window("Test", 1, 1)
            .borderless()
            .opengl()
            .build()
            .unwrap();
        let canvas = window.into_canvas().accelerated().build().unwrap();
        let font_context = sdl2::ttf::init().unwrap();
        let texture_creator = canvas.texture_creator();
        let mut renderer = Renderer::new(config.clone(), &font_context, &texture_creator);

        let mut editor = FileEditor::new(Rect::new(0, 0, 100, 100), config.clone());
        let mut file = EditorFile::new("./foo.txt".to_string(), "foo".to_string(), config.clone());
        file.prepare_ui(&mut renderer);
        assert_eq!(editor.open_file(file).is_none(), true);
        assert_eq!(editor.caret().position().text_position(), 0);
        assert_eq!(editor.file().is_some(), true);
        assert_eq!(editor.file().unwrap().sections().len(), 1);
        assert_eq!(editor.file().unwrap().get_character_at(0).is_some(), true);

        editor.insert_text("z".to_string(), &mut renderer);
        assert_eq!(editor.caret().position().text_position(), 1);
        assert_eq!(editor.file().is_some(), true);
        assert_eq!(editor.file().unwrap().buffer(), "zfoo".to_string());
    }
}
