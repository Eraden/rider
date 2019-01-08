use sdl2::pixels::*;
use sdl2::rect::*;
use std::borrow::*;
use std::mem;
use std::sync::*;

use crate::app::*;
use crate::app::{UpdateResult as UR, WindowCanvas as WS};
use crate::ui::*;
use crate::ui::scroll_bar::Scrollable;
use crate::ui::scroll_bar::vertical_scroll_bar::*;
use crate::ui::scroll_bar::horizontal_scroll_bar::*;

pub struct FileEditor {
    dest: Rect,
    full_rect: Rect,
    scroll: Point,
    caret: Caret,
    file: Option<EditorFile>,
    config: ConfigAccess,
    vertical_scroll_bar: VerticalScrollBar,
    horizontal_scroll_bar: HorizontalScrollBar,
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
            full_rect: Rect::new(0, 0, 0, 0),
            scroll: Point::new(0, 0),
            caret: Caret::new(Arc::clone(&config)),
            vertical_scroll_bar: VerticalScrollBar::new(Arc::clone(&config)),
            horizontal_scroll_bar: HorizontalScrollBar::new(Arc::clone(&config)),
            file: None,
            config,
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
}

impl ScrollableView for FileEditor {
    fn scroll_to(&mut self, x: i32, y: i32) {
        let line_height = match self.file() {
            None => 1,
            Some(f) => f.line_height(),
        };
        let read_config = self.config.read().unwrap();
        let mut nx = self.scroll.x() + (read_config.scroll().speed() * x);
        let mut ny = self.scroll.y() + (read_config.scroll().speed() * y);
        let scroll_rect = move_render_point(self.render_start_point(), &self.full_rect);
        let min_x = scroll_rect.x() + scroll_rect.width() as i32;
        let min_y = scroll_rect.y() + scroll_rect.height() as i32 + self.dest.width() as i32 + line_height as i32;

        match x {
            _ if nx > 0 => {
                nx = 0;
            }
            _ if nx < -min_x => {
                nx = -min_x;
            }
            _ => (),
        }
        match y {
            _ if ny > 0 => {
                ny = 0;
            }
            _ if ny < -min_y => {
                ny = -min_y;
            }
            _ => (),
        }
        self.vertical_scroll_bar.scroll_to(ny);
        self.horizontal_scroll_bar.scroll_to(nx);
        self.scroll = Point::new(nx, ny);
    }

    fn scroll(&self) -> &Point {
        &self.scroll
    }
}

impl FileAccess for FileEditor {
    fn has_file(&self) -> bool {
        self.file.is_some()
    }

    fn file(&self) -> Option<&EditorFile> {
        self.file.as_ref()
    }

    fn file_mut(&mut self) -> Option<&mut EditorFile> {
        self.file.as_mut()
    }

    fn open_file(&mut self, file: EditorFile) -> Option<EditorFile> {
        let mut file = Some(file);
        mem::swap(&mut self.file, &mut file);
        if let Some(f) = self.file.as_ref() {
            self.full_rect = f.full_rect();
        }
        file
    }

    fn drop_file(&mut self) -> Option<EditorFile> {
        if self.has_file() {
            let mut file = None;
            mem::swap(&mut self.file, &mut file);
            file
        } else {
            None
        }
    }

    fn replace_current_file(&mut self, file: EditorFile) {
        self.open_file(file);
    }
}

impl CaretAccess for FileEditor {
    fn caret(&self) -> &Caret {
        &self.caret
    }

    fn caret_mut(&mut self) -> &mut Caret {
        &mut self.caret
    }

    fn move_caret(&mut self, dir: MoveDirection) {
        match dir {
            MoveDirection::Left => {}
            MoveDirection::Right => caret_manager::move_caret_right(self),
            MoveDirection::Up => {}
            MoveDirection::Down => {}
        }
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
    fn render(&self, canvas: &mut WS, renderer: &mut Renderer, _context: &RenderContext) {
        canvas.set_clip_rect(self.dest.clone());
        match self.file() {
            Some(file) => file.render(canvas, renderer, &RenderContext::RelativePosition(self.render_start_point())),
            _ => (),
        };
        self.caret.render(canvas, renderer, &RenderContext::RelativePosition(self.render_start_point()));
        self.vertical_scroll_bar.render(canvas, renderer, &RenderContext::RelativePosition(self.dest.top_left()));
        self.horizontal_scroll_bar.render(canvas, renderer, &RenderContext::RelativePosition(self.dest.top_left()));
    }

    fn prepare_ui(&mut self, renderer: &mut Renderer) {
        self.caret.prepare_ui(renderer);
    }
}

impl Update for FileEditor {
    fn update(&mut self, ticks: i32, context: &UpdateContext) -> UR {
        let (
            width,
            height,
            editor_left_margin,
            editor_top_margin,
            scroll_width,
            scroll_margin,
        ) = {
            let config: RwLockReadGuard<Config> = self.config.read().unwrap();
            (
                config.width(),
                config.height(),
                config.editor_left_margin() as u32,
                config.editor_top_margin() as u32,
                config.scroll().width(),
                config.scroll().margin_right(),
            )
        };
        self.dest.set_width(width - editor_left_margin);
        self.dest.set_height(height - editor_top_margin);

        self.vertical_scroll_bar.set_full_size(self.full_rect.height());
        self.vertical_scroll_bar.set_viewport(self.dest.height());
        self.vertical_scroll_bar.set_location(self.dest.width() as i32 - (scroll_width as i32 + scroll_margin));
        self.vertical_scroll_bar.update(ticks, context);

        self.horizontal_scroll_bar.set_full_size(self.full_rect.width());
        self.horizontal_scroll_bar.set_viewport(self.dest.width());
        self.horizontal_scroll_bar.set_location(self.dest.height() as i32 - (scroll_width as i32 + scroll_margin));
        self.horizontal_scroll_bar.update(ticks, context);

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

    fn dest(&self) -> &Rect {
        &self.dest
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

#[cfg(test)]
mod test_config_holder {
    use crate::app::*;
    use crate::tests::support;
    use crate::ui::*;
    use sdl2::rect::*;
    use sdl2::*;
    use std::borrow::*;
    use std::rc::*;
    use std::sync::*;

    #[test]
    fn assert_config() {
        let config = support::build_config();
        let widget = FileEditor::new(Arc::clone(&config));
        let result = widget.config();
        {
            let mut w = config.write().unwrap();
            w.set_height(1240);
            w.set_width(1024);
        }
        let local = config.read().unwrap();
        let widget_config = result.read().unwrap();
        assert_eq!(widget_config.width(), local.width());
        assert_eq!(widget_config.height(), local.height());
    }
}

#[cfg(test)]
mod test_render_box {
    use crate::app::*;
    use crate::tests::support;
    use crate::ui::*;
    use sdl2::rect::*;
    use sdl2::*;
    use std::borrow::*;
    use std::rc::*;
    use std::sync::*;

    impl FileEditor {
        pub fn set_full_rect(&mut self, r: Rect) {
            self.full_rect = r;
        }
    }

    #[test]
    fn assert_dest() {
        let config = support::build_config();
        let (x, y, mw, mh) = {
            let c = config.read().unwrap();
            (
                c.editor_left_margin(),
                c.editor_top_margin(),
                c.width(),
                c.height(),
            )
        };
        let widget = FileEditor::new(config);
        let result = widget.dest().clone();
        let expected = Rect::new(x, y, mw - x as u32, mh - y as u32);
        assert_eq!(result, expected);
    }

    #[test]
    fn assert_render_start_point() {
        let config = support::build_config();
        let (x, y, ss) = {
            let c = config.read().unwrap();
            (
                c.editor_left_margin(),
                c.editor_top_margin(),
                c.scroll().speed(),
            )
        };
        let mut widget = FileEditor::new(config);
        widget.set_full_rect(Rect::new(0, 0, 9999, 9999));
        widget.scroll_to(-30, -40);
        let result = widget.render_start_point().clone();
        let expected = Point::new(x + (ss * -30), y + (ss * -40));
        assert_eq!(result, expected);
    }
}
