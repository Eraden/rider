use crate::app::caret_manager;
use crate::app::file_content_manager;
use crate::app::{UpdateResult, WindowCanvas};
use crate::config::Config;
use crate::renderer::Renderer;
use crate::ui::caret::Caret;
use crate::ui::caret::{CaretPosition, MoveDirection};
use crate::ui::file::editor_file::EditorFile;
use crate::ui::file::*;
use crate::ui::menu_bar::MenuBar;
use crate::ui::text_character::TextCharacter;
use crate::ui::*;
use sdl2::rect::{Point, Rect};
use sdl2::VideoSubsystem as VS;
use std::boxed::Box;
use std::rc::Rc;
use std::sync::Arc;

pub struct AppState {
    menu_bar: MenuBar,
    files: Vec<EditorFile>,
    current_file: usize,
    caret: Caret,
    config: Rc<Config>,
}

impl AppState {
    pub fn new(config: Rc<Config>) -> Self {
        Self {
            menu_bar: MenuBar::new(config.clone()),
            files: vec![],
            current_file: 0,
            caret: Caret::new(config.clone()),
            config,
        }
    }

    pub fn open_file(&mut self, file_path: String, renderer: &mut Renderer) {
        use std::fs::read_to_string;

        if let Ok(buffer) = read_to_string(&file_path) {
            let mut file = EditorFile::new(file_path.clone(), buffer, self.config.clone());
            file.prepare_ui(renderer);
            self.current_file = self.files.len();
            self.files.push(file);
        } else {
            eprintln!("Failed to open file: {}", file_path);
        };
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

    pub fn current_file(&self) -> Option<&EditorFile> {
        self.files.get(self.current_file)
    }

    pub fn current_file_mut(&mut self) -> Option<&mut EditorFile> {
        self.files.get_mut(self.current_file)
    }

    fn on_editor_clicked(&mut self, point: &Point, video_subsystem: &mut VS) -> UpdateResult {
        let current_file: &mut EditorFile = if let Some(current_file) = self.current_file_mut() {
            current_file
        } else {
            return UpdateResult::NoOp;
        };
        if !current_file.is_left_click_target(point) {
            return UpdateResult::NoOp;
        }
        video_subsystem.text_input().start();
        match current_file.on_left_click(point) {
            UpdateResult::MoveCaret(rect, position) => {
                self.caret
                    .move_caret(position, Point::new(rect.x(), rect.y()));
            }
            _ => (),
        };

        UpdateResult::NoOp
    }

    pub fn move_caret(&mut self, dir: MoveDirection) {
        match dir {
            MoveDirection::Left => {}
            MoveDirection::Right => caret_manager::move_caret_right(self),
            MoveDirection::Up => {}
            MoveDirection::Down => {}
        }
    }

    pub fn delete_front(&mut self) {
        file_content_manager::delete_front(self);
    }

    pub fn delete_back(&mut self) {
        file_content_manager::delete_back(self);
    }

    pub fn insert_text(&mut self, text: String, renderer: &mut Renderer) {
        file_content_manager::insert_text(self, text, renderer);
    }

    pub fn insert_new_line(&mut self, renderer: &mut Renderer) {
        file_content_manager::insert_new_line(self, renderer);
    }

    pub fn replace_current_file(&mut self, file: EditorFile) {
        self.files[self.current_file] = file;
    }
}

impl Render for AppState {
    fn render(
        &self,
        canvas: &mut WindowCanvas,
        renderer: &mut Renderer,
        _parent: Option<&RenderBox>,
    ) -> UpdateResult {
        self.menu_bar.render(canvas, renderer, None);
        if let Some(file) = self.current_file() {
            file.render(canvas, renderer, None);
        }
        self.caret.render(canvas, renderer, None);
        UpdateResult::NoOp
    }

    fn prepare_ui(&mut self, renderer: &mut Renderer) {
        self.menu_bar.prepare_ui(renderer);
        self.caret.prepare_ui(renderer);
    }
}

impl Update for AppState {
    fn update(&mut self, ticks: i32) -> UpdateResult {
        self.menu_bar.update(ticks);
        if let Some(file) = self.files.get_mut(self.current_file) {
            file.update(ticks);
        }
        self.caret.update(ticks);
        UpdateResult::NoOp
    }
}

impl AppState {
    pub fn on_left_click(&mut self, point: &Point, video_subsystem: &mut VS) -> UpdateResult {
        if self.menu_bar.is_left_click_target(point) {
            video_subsystem.text_input().stop();
            return self.menu_bar.on_left_click(point);
        }
        self.on_editor_clicked(point, video_subsystem);
        UpdateResult::NoOp
    }

    pub fn is_left_click_target(&self, _point: &Point) -> bool {
        true
    }
}
