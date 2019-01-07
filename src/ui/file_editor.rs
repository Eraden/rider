use sdl2::rect::*;
use std::borrow::*;
use std::mem;
use std::rc::Rc;
use std::sync::*;

use crate::app::*;
use crate::app::{UpdateResult as UR, WindowCanvas as WS};
use crate::config::*;
use crate::ui::*;

pub struct FileEditor {
    dest: Rect,
    scroll: Point,
    caret: Caret,
    file: Option<EditorFile>,
    config: ConfigAccess,
}

impl FileEditor {
    pub fn new(config: ConfigAccess) -> Self {
        let dest = {
            let c = config.read().unwrap();
            Rect::new(
                c.editor_left_margin(),
                c.editor_top_margin(),
                c.width() - c.editor_left_margin() as u32,
                c.height() - c.editor_top_margin() as u32,
            )
        };
        Self {
            dest,
            scroll: Point::new(0, 0),
            caret: Caret::new(Arc::clone(&config)),
            file: None,
            config,
        }
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

    pub fn scroll_to(&mut self, x: i32, y: i32) {
        let read_config = self.config.read().unwrap();
        self.scroll = self.scroll
            + Point::new(
                read_config.scroll_speed() * x,
                read_config.scroll_speed() * y,
            );
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
        let mut y = point.y() - self.render_start_point().y();
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
    fn render(&self, canvas: &mut WS, renderer: &mut Renderer, _parent: Parent) {
        canvas.set_clip_rect(self.dest.clone());
        match self.file() {
            Some(file) => file.render(canvas, renderer, Some(self)),
            _ => (),
        };
        self.caret.render(canvas, renderer, Some(self));
    }

    fn prepare_ui(&mut self, renderer: &mut Renderer) {
        self.caret.prepare_ui(renderer);
    }
}

impl Update for FileEditor {
    fn update(&mut self, ticks: i32, context: &UpdateContext) -> UR {
        {
            let config = self.config.read().unwrap();
            self.dest
                .set_width(config.width() - config.editor_left_margin() as u32);
            self.dest
                .set_height(config.height() - config.editor_top_margin() as u32);
        }
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
        self.dest.top_left() + self.scroll
    }
}

impl ConfigHolder for FileEditor {
    fn config(&self) -> &ConfigAccess {
        &self.config
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
    use std::sync::*;

    #[test]
    fn replace_file() {
        let config = Arc::new(RwLock::new(Config::new()));
        let mut editor = FileEditor::new(Arc::clone(&config));
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
}
