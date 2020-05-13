use crate::app::*;
use crate::renderer::renderer::Renderer;
use crate::ui::caret::caret::Caret;
use crate::ui::caret::caret_position::CaretPosition;
use crate::ui::caret::MoveDirection;
use crate::ui::file::editor_file::EditorFile;
use crate::ui::file::TextCollection;
use crate::ui::file::TextWidget;
use crate::ui::scroll_bar::horizontal_scroll_bar::*;
use crate::ui::scroll_bar::vertical_scroll_bar::*;
use crate::ui::scroll_bar::ScrollWidget;
use crate::ui::text_character::CharacterSizeManager;
use crate::ui::RenderContext;
use crate::ui::UpdateContext;
use crate::ui::{move_render_point, ScrollView};
use crate::ui::{CanvasAccess, Widget};
use sdl2::rect::Point;
use sdl2::rect::Rect;
use std::mem;
use std::sync::*;

pub trait FileAccess {
    fn has_file(&self) -> bool {
        self.file().is_some()
    }

    fn file(&self) -> Option<&EditorFile>;

    fn file_mut(&mut self) -> Option<&mut EditorFile>;

    fn open_file(&mut self, file: EditorFile) -> Option<EditorFile>;

    fn drop_file(&mut self) -> Option<EditorFile>;

    fn replace_current_file(&mut self, file: EditorFile) {
        self.open_file(file);
    }
}

pub trait CaretAccess: FileAccess {
    fn caret(&self) -> &Caret;

    fn caret_mut(&mut self) -> &mut Caret;

    fn move_caret(&mut self, dir: MoveDirection) {
        match dir {
            MoveDirection::Left => caret_manager::move_caret_left(self),
            MoveDirection::Right => caret_manager::move_caret_right(self),
            MoveDirection::Up => caret_manager::move_caret_up(self),
            MoveDirection::Down => caret_manager::move_caret_down(self),
        };
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
                    let p = if text_character.is_new_line() {
                        file.get_character_at(text_character.position() + 1)
                            .map_or_else(
                                || text_character.dest().top_left(),
                                |tc| tc.dest().top_left(),
                            )
                    } else {
                        rect.top_right()
                    };
                    self.caret_mut().move_caret(position, p);
                    break;
                }
                _ => {
                    line -= 1;
                }
            }
        }
    }
}

pub struct FileEditor {
    dest: Rect,
    full_rect: Rect,
    caret: Caret,
    file: Option<EditorFile>,
    config: ConfigAccess,
    vertical_scroll_bar: VerticalScrollBar,
    horizontal_scroll_bar: HorizontalScrollBar,
}

impl Widget for FileEditor {
    fn texture_path(&self) -> Option<String> {
        None
    }

    fn dest(&self) -> &Rect {
        &self.dest
    }

    fn set_dest(&mut self, rect: &Rect) {
        self.dest = rect.clone();
    }

    fn source(&self) -> &Rect {
        &self.dest
    }

    fn set_source(&mut self, _rect: &Rect) {}

    fn update(&mut self, ticks: i32, context: &UpdateContext) -> UpdateResult {
        let (width, height, editor_left_margin, editor_top_margin, scroll_width, scroll_margin) = {
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
        let editor_left_margin = match context {
            UpdateContext::ParentPosition(p) => p.x() as u32,
            _ => editor_left_margin as u32,
        };
        self.dest.set_x(editor_left_margin.clone() as i32);
        self.dest.set_width(width - editor_left_margin);
        self.dest.set_height(height - editor_top_margin);

        self.vertical_scroll_bar
            .set_full_size(self.full_rect.height());
        self.vertical_scroll_bar.set_viewport(self.dest.height());
        self.vertical_scroll_bar
            .set_location(self.dest.width() as i32 - (scroll_width as i32 + scroll_margin));
        self.vertical_scroll_bar.update(ticks, context);

        self.horizontal_scroll_bar
            .set_full_size(self.full_rect.width());
        self.horizontal_scroll_bar.set_viewport(self.dest.width());
        self.horizontal_scroll_bar
            .set_location(self.dest.height() as i32 - (scroll_width as i32 + scroll_margin));
        self.horizontal_scroll_bar.update(ticks, context);

        self.caret.update(ticks, context);
        match self.file_mut() {
            Some(file) => file.update(ticks, context),
            _ => UpdateResult::NoOp,
        }
    }

    fn on_left_click(&mut self, point: &Point, _context: &UpdateContext) -> UpdateResult {
        let it = match self.file() {
            Some(f) => f.iter_char(),
            _ => return UpdateResult::NoOp,
        };
        let scroll = self.scroll();
        let render_point = self.render_start_point();
        let moved_by = self.scroll().offset(render_point.x(), render_point.y());
        let scroll_context = UpdateContext::ScrolledBy(moved_by.clone());
        let mut target: Option<(Point, CaretPosition)> = None;
        for char in it {
            if char.is_left_click_target(point, &scroll_context) {
                let position = CaretPosition::new(char.position(), char.line(), 0);
                let point = char.dest().top_left();
                target = Some((point, position));
                break;
            }
        }
        if let Some((point, position)) = target {
            self.caret.move_caret(position, point);
        } else {
            self.set_caret_to_end_of_line(
                self.resolve_line_from_point(&point.offset(-scroll.x(), -scroll.y())),
            );
        }
        UpdateResult::NoOp
    }

    fn is_left_click_target(&self, point: &Point, _context: &UpdateContext) -> bool {
        self.is_text_character_clicked(point) || self.is_editor_clicked(point)
    }

    fn use_clipping(&self) -> bool {
        true
    }

    fn render<C, R>(&self, canvas: &mut C, renderer: &mut R, _context: &RenderContext)
    where
        C: CanvasAccess,
        R: Renderer + CharacterSizeManager + ConfigHolder,
    {
        if self.use_clipping() {
            canvas.set_clipping(self.dest.clone());
        }
        match self.file() {
            Some(file) => file.render(
                canvas,
                renderer,
                &RenderContext::ParentPosition(self.render_start_point() + self.scroll()),
            ),
            _ => (),
        };
        self.caret.render(
            canvas,
            renderer,
            &RenderContext::ParentPosition(self.render_start_point() + self.scroll()),
        );
        self.vertical_scroll_bar
            .render(canvas, &RenderContext::ParentPosition(self.dest.top_left()));
        self.horizontal_scroll_bar
            .render(canvas, &RenderContext::ParentPosition(self.dest.top_left()));
    }

    fn prepare_ui<T>(&mut self, renderer: &mut T)
    where
        T: Renderer + CharacterSizeManager + ConfigHolder,
    {
        if let Some(ref mut file) = self.file {
            file.prepare_ui(renderer);
        }
        self.caret.prepare_ui(renderer);
    }
}

impl FileEditor {
    pub fn new(config: ConfigAccess) -> Self {
        Self {
            dest: {
                let c = config.read().unwrap();
                Rect::new(
                    c.editor_left_margin(),
                    c.editor_top_margin(),
                    c.width() - c.editor_left_margin() as u32,
                    c.height() - c.editor_top_margin() as u32,
                )
            },
            full_rect: Rect::new(0, 0, 0, 0),
            caret: Caret::new(Arc::clone(&config)),
            vertical_scroll_bar: VerticalScrollBar::new(Arc::clone(&config)),
            horizontal_scroll_bar: HorizontalScrollBar::new(Arc::clone(&config)),
            file: None,
            config,
        }
    }

    pub fn delete_front<R>(&mut self, renderer: &mut R)
    where
        R: ConfigHolder + CharacterSizeManager + Renderer,
    {
        file_content_manager::delete_front(self, renderer);
    }

    pub fn delete_back<R>(&mut self, renderer: &mut R)
    where
        R: ConfigHolder + CharacterSizeManager + Renderer,
    {
        file_content_manager::delete_back(self, renderer);
    }

    pub fn insert_text<R>(&mut self, text: String, renderer: &mut R)
    where
        R: ConfigHolder + CharacterSizeManager + Renderer,
    {
        file_content_manager::insert_text(self, text, renderer);
    }

    pub fn insert_new_line<R>(&mut self, renderer: &mut R) -> Result<(), String>
    where
        R: ConfigHolder + CharacterSizeManager + Renderer,
    {
        file_content_manager::insert_new_line(self, renderer)
    }

    pub fn delete_current_line<R>(&mut self, renderer: &mut R) -> Result<(), String>
    where
        R: ConfigHolder + CharacterSizeManager + Renderer,
    {
        file_content_manager::delete_current_line(self, renderer)
    }

    fn is_text_character_clicked(&self, point: &Point) -> bool {
        let file = match self.file() {
            Some(f) => f,
            _ => return false,
        };
        let moved_by = self
            .scroll()
            .offset(self.render_start_point().x(), self.render_start_point().y());
        let scroll_context = UpdateContext::ScrolledBy(moved_by.clone());
        for char in file.iter_char() {
            if char.is_left_click_target(point, &scroll_context) {
                return true;
            }
        }
        false
    }

    fn is_editor_clicked(&self, point: &Point) -> bool {
        self.dest()
            .contains_point(move_render_point(point.clone(), self.dest()).top_left())
    }

    fn resolve_line_from_point(&self, point: &Point) -> i32 {
        let file = match self.file() {
            Some(f) => f,
            _ => return 0,
        };
        let y = point.y() - self.render_start_point().y();
        match (y, file.line_height()) {
            (y, _) if y <= 0 => 0,
            (_, 0) => 0,
            (_, line_height) => y / (line_height as i32),
        }
    }
}

impl ScrollView<VerticalScrollBar, HorizontalScrollBar> for FileEditor {
    fn mut_horizontal_scroll_handler(&mut self) -> Option<&mut HorizontalScrollBar> {
        Some(&mut self.horizontal_scroll_bar)
    }

    fn horizontal_scroll_handler(&self) -> Option<&HorizontalScrollBar> {
        Some(&self.horizontal_scroll_bar)
    }

    fn mut_vertical_scroll_handler(&mut self) -> Option<&mut VerticalScrollBar> {
        Some(&mut self.vertical_scroll_bar)
    }

    fn vertical_scroll_handler(&self) -> Option<&VerticalScrollBar> {
        Some(&self.vertical_scroll_bar)
    }
}

impl FileAccess for FileEditor {
    fn file(&self) -> Option<&EditorFile> {
        self.file.as_ref()
    }

    fn file_mut(&mut self) -> Option<&mut EditorFile> {
        self.file.as_mut()
    }

    fn open_file(&mut self, file: EditorFile) -> Option<EditorFile> {
        let new_path = file.path();
        let mut file = Some(file);
        let old_path = match self.file() {
            Some(f) => f.path(),
            _ => format!(""),
        };
        mem::swap(&mut self.file, &mut file);
        if let Some(ref f) = self.file {
            self.full_rect = f.full_rect();
        }
        if old_path != new_path {
            self.vertical_scroll_bar.reset();
            self.horizontal_scroll_bar.reset();
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
}

impl CaretAccess for FileEditor {
    fn caret(&self) -> &Caret {
        &self.caret
    }

    fn caret_mut(&mut self) -> &mut Caret {
        &mut self.caret
    }
}

impl ConfigHolder for FileEditor {
    fn config(&self) -> &ConfigAccess {
        &self.config
    }
}

#[cfg(test)]
mod tests {
    use crate::app::UpdateResult;
    use crate::tests::*;
    use crate::ui::*;
    use rider_config::{Config, ConfigHolder};
    use rider_derive::*;
    use sdl2::rect::{Point, Rect};
    use std::sync::*;

    #[cfg_attr(tarpaulin, skip)]
    impl FileEditor {
        pub fn set_full_rect(&mut self, r: Rect) {
            self.full_rect = r;
        }

        pub fn set_dest(&mut self, r: Rect) {
            self.dest = r;
        }
    }

    // Widget

    #[test]
    fn assert_texture_path() {
        let config = build_config();
        let widget = FileEditor::new(config);
        let result = widget.texture_path();
        assert!(result.is_none());
    }

    #[test]
    fn assert_dest() {
        let config = build_config();
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
    fn assert_set_dest() {
        let config = build_config();
        let mut widget = FileEditor::new(config);
        widget.set_dest(Rect::new(100, 200, 300, 400));
        assert_eq!(widget.dest(), &Rect::new(100, 200, 300, 400));
    }

    #[test]
    fn assert_source() {
        let config = build_config();
        let widget = FileEditor::new(config);
        let result = widget.source();
        assert_eq!(result, &Rect::new(10, 50, 1014, 810));
    }

    #[test]
    fn assert_set_source() {
        let config = build_config();
        let mut widget = FileEditor::new(config);
        widget.set_source(&Rect::new(1, 2, 3, 4));
        assert_ne!(widget.source(), &Rect::new(1, 2, 3, 4));
    }

    #[test]
    fn assert_update() {
        let config = build_config();
        let mut widget = FileEditor::new(config);
        let result = widget.update(0, &UpdateContext::Nothing);
        assert_eq!(result, UpdateResult::NoOp);
    }

    #[test]
    fn assert_on_left_click() {
        let config = build_config();
        let mut widget = FileEditor::new(config);
        let result = widget.on_left_click(&Point::new(600, 800), &UpdateContext::Nothing);
        assert_eq!(result, UpdateResult::NoOp);
    }

    #[test]
    fn assert_is_left_click_target() {
        let config = build_config();
        let widget = FileEditor::new(config);
        let result = widget.is_left_click_target(&Point::new(600, 800), &UpdateContext::Nothing);
        assert_eq!(result, true);
    }

    #[test]
    fn assert_use_clipping() {
        let config = build_config();
        let widget = FileEditor::new(config);
        let result = widget.use_clipping();
        assert_eq!(result, true);
    }

    #[test]
    fn assert_render() {
        build_test_renderer!(renderer);
        let mut canvas = CanvasMock::new();
        let widget = FileEditor::new(config);
        let result = widget.render(&mut canvas, &mut renderer, &RenderContext::Nothing);
        assert_eq!(result, ());
    }

    #[test]
    fn assert_prepare_ui() {
        build_test_renderer!(renderer);
        let mut widget = FileEditor::new(config);
        let result = widget.prepare_ui(&mut renderer);
        assert_eq!(result, ());
    }

    // File manipulation

    #[test]
    fn assert_has_file() {
        let config = build_config();
        let widget = FileEditor::new(config);
        assert_eq!(widget.has_file(), false);
    }

    #[test]
    fn drop_file_when_no_file_is_selected() {
        let config = build_config();
        let mut widget = FileEditor::new(config);
        let result = widget.drop_file();
        assert!(result.is_none());
    }

    #[test]
    fn drop_file_when_file_is_selected() {
        let config = build_config();
        let mut widget = FileEditor::new(config.clone());
        let file = EditorFile::new(
            "/tmp/drop_file_when_file_is_selected".to_owned(),
            "foo bar".to_owned(),
            config,
        );
        widget.open_file(file);
        let result = widget.drop_file();
        assert!(result.is_some())
    }

    #[test]
    fn mut_file_when_no_file_is_selected() {
        let config = build_config();
        let mut widget = FileEditor::new(config);
        let result = widget.file_mut();
        assert!(result.is_none());
    }

    #[test]
    fn mut_file_when_file_is_selected() {
        let config = build_config();
        let mut widget = FileEditor::new(config.clone());
        let file = EditorFile::new(
            "/tmp/drop_file_when_file_is_selected".to_owned(),
            "foo bar".to_owned(),
            config,
        );
        widget.open_file(file);
        let result = widget.file_mut();
        assert!(result.is_some())
    }

    #[test]
    fn file_when_no_file_is_selected() {
        let config = build_config();
        let widget = FileEditor::new(config);
        let result = widget.file();
        assert!(result.is_none());
    }

    #[test]
    fn file_when_file_is_selected() {
        let config = build_config();
        let mut widget = FileEditor::new(config.clone());
        let file = EditorFile::new(
            "/tmp/drop_file_when_file_is_selected".to_owned(),
            "foo bar".to_owned(),
            config,
        );
        widget.open_file(file);
        let result = widget.file();
        assert!(result.is_some())
    }

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

    // config

    #[test]
    fn assert_config() {
        let config = build_config();
        let widget = FileEditor::new(config.clone());
        assert!(widget
            .config()
            .write()
            .and_then(|ref mut w| {
                w.set_height(1240);
                w.set_width(1024);
                Ok(())
            })
            .is_ok());
        let local: (u32, u32) = config
            .read()
            .map_or_else(|_| (0, 0), |ref w| (w.width(), w.height()));
        let widget_config: (u32, u32) = widget
            .config()
            .read()
            .map_or_else(|_| (0, 0), |ref w| (w.width(), w.height()));
        assert_eq!(widget_config, local);
    }
}

#[cfg(test)]
mod own_methods_tests {
    use crate::tests::*;
    use crate::ui::{CaretAccess, EditorFile, FileAccess, FileEditor};
    use rider_config::ConfigAccess;
    use rider_derive::*;
    use sdl2::rect::Point;

    fn build_testable_file<S>(buffer: S, config: ConfigAccess) -> EditorFile
    where
        S: Into<String> + Sized,
    {
        EditorFile::new(
            "/tmp/drop_file_when_file_is_selected".to_owned(),
            buffer.into(),
            config,
        )
    }

    #[test]
    fn delete_front() {
        build_test_renderer!(renderer);
        let mut widget = FileEditor::new(config.clone());
        widget.open_file(build_testable_file("foo bar", config));
        widget.caret_mut().set_text_position(1);
        widget.delete_front(&mut renderer);
        let buffer = widget
            .file()
            .map_or_else(|| "".to_owned(), |ref f| f.buffer());
        assert_eq!(buffer, "oo bar".to_owned());
    }

    #[test]
    fn delete_back() {
        build_test_renderer!(renderer);
        let mut widget = FileEditor::new(config.clone());
        widget.open_file(build_testable_file("foo bar", config));
        widget.caret_mut().set_text_position(6);
        widget.delete_back(&mut renderer);
        let buffer = widget
            .file()
            .map_or_else(|| "".to_owned(), |ref f| f.buffer());
        assert_eq!(buffer, "foo ba".to_owned());
    }

    #[test]
    fn insert_text() {
        build_test_renderer!(renderer);
        let mut widget = FileEditor::new(config.clone());
        widget.open_file(build_testable_file("foo bar", config));
        widget.insert_text("hello world ".to_owned(), &mut renderer);
        let buffer = widget
            .file()
            .map_or_else(|| "".to_owned(), |ref f| f.buffer());
        assert_eq!(buffer, "hello world foo bar".to_owned());
    }

    #[test]
    fn insert_new_line() {
        build_test_renderer!(renderer);
        let mut widget = FileEditor::new(config.clone());
        widget.open_file(build_testable_file("foo bar", config));
        assert!(widget.insert_new_line(&mut renderer).is_ok());
        let buffer = widget
            .file()
            .map_or_else(|| "".to_owned(), |ref f| f.buffer());
        assert_eq!(buffer, "\nfoo bar".to_owned());
    }

    #[test]
    fn delete_current_line() {
        build_test_renderer!(renderer);
        let mut widget = FileEditor::new(config.clone());
        widget.open_file(build_testable_file("foo bar\nfoz baz", config));
        assert!(widget.delete_current_line(&mut renderer).is_ok());
        let buffer = widget
            .file()
            .map_or_else(|| "".to_owned(), |ref f| f.buffer());
        assert_eq!(buffer, "foz baz");
    }

    #[test]
    fn is_text_character_clicked() {
        let config = build_config();
        let mut widget = FileEditor::new(config.clone());
        widget.open_file(build_testable_file("foo bar", config));
        assert_eq!(
            widget.is_text_character_clicked(&Point::new(1000, 1000)),
            false
        );
    }

    #[test]
    fn is_editor_clicked() {
        let config = build_config();
        let mut widget = FileEditor::new(config.clone());
        widget.open_file(build_testable_file("foo bar", config));
        assert_eq!(widget.is_editor_clicked(&Point::new(1000, 1000)), false);
    }

    #[test]
    fn resolve_line_from_point() {
        let config = build_config();
        let mut widget = FileEditor::new(config.clone());
        widget.open_file(build_testable_file("foo bar", config));
        assert_eq!(widget.resolve_line_from_point(&Point::new(100, 100)), 0);
    }
}
