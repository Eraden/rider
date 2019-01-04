use crate::app::{UpdateResult, WindowCanvas};
use crate::config::Config;
use crate::ui::file::*;
use crate::ui::file::editor_file::EditorFile;
use crate::renderer::Renderer;
use crate::ui::*;
use crate::ui::caret::Caret;
use crate::ui::menu_bar::MenuBar;
use std::boxed::Box;
use std::rc::Rc;
use std::sync::Arc;
use sdl2::rect::{Rect, Point};

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

    pub fn open_file(&mut self, file_path: String) {
        use std::fs::read_to_string;
        if let Ok(buffer) = read_to_string(&file_path) {
            let file = EditorFile::new(file_path.clone(), buffer, self.config.clone());
            self.current_file = self.files.len();
            self.files.push(file);
        };
    }

    pub fn caret(&mut self) -> &mut Caret {
        &mut self.caret
    }

    pub fn delete_front(&mut self) {
        let file: &mut EditorFile = if let Some(file) = self.files.get_mut(self.current_file) {
            file
        } else {
            return;
        };
        let mut buffer: String = file.buffer();
        let caret: &mut Caret = &mut self.caret;
        let position: usize = caret.text_position();
        if position == 0 {
            return;
        }
        buffer.remove(position);
        match file.get_character_at(position - 1) {
            Some(character) => {
                let dest: &Rect = character.dest();
                caret.move_caret(position - 1, Point::new(
                    dest.x(), dest.y(),
                ));
            }
            _ => {
                caret.reset_caret();
            }
        }
        let new_file = EditorFile::new(
            file.path(),
            buffer,
            self.config.clone(),
        );
        self.files[self.current_file] = new_file;
    }

    pub fn delete_back(&mut self) {
        let file: &mut EditorFile = if let Some(file) = self.files.get_mut(self.current_file) {
            file
        } else {
            return;
        };
        let mut buffer: String = file.buffer();
        let caret: &mut Caret = &mut self.caret;
        let position: usize = caret.text_position();
        if position >= buffer.len() {
            return;
        }
        buffer.remove(position);
        let new_file = EditorFile::new(
            file.path(),
            buffer,
            self.config.clone(),
        );
        self.files[self.current_file] = new_file;
    }

    pub fn insert_character(&mut self, character: char, renderer: &mut Renderer) {
        let file: &mut EditorFile = if let Some(file) = self.files.get_mut(self.current_file) {
            file
        } else {
            return;
        };
        let mut buffer: String = file.buffer();
        let caret: &mut Caret = &mut self.caret;
        let position: usize = caret.text_position();
        buffer.insert(position, character.clone());
        let new_file = EditorFile::new(
            file.path(),
            buffer,
            self.config.clone(),
        );
        if let Some(rect) = get_text_character_rect(character, renderer) {
            if let Some(current) = file.get_character_at(position) {
                caret.move_caret(position + 1, Point::new(
                    current.dest().x() + rect.width() as i32,
                    current.dest().y(),
                ));
            }
        }
        self.files[self.current_file] = new_file;
    }

    fn current_file(&mut self) -> Option<&mut EditorFile> {
        self.files.get_mut(self.current_file)
    }

    fn on_editor_clicked(&mut self, point: &Point) -> UpdateResult {
        let current_file: &mut EditorFile = if let Some(current_file) = self.current_file() {
            current_file
        } else {
            return UpdateResult::NoOp;
        };
        if !current_file.is_left_click_target(point) {
            return UpdateResult::NoOp;
        }
        match current_file.on_left_click(point) {
            UpdateResult::MoveCaret(rect, position) => {
                self.caret.move_caret(position, Point::new(rect.x(), rect.y()));
            }
            _ => (),
        };

        UpdateResult::NoOp
    }
}

impl Render for AppState {
    fn render(&mut self, canvas: &mut WindowCanvas, renderer: &mut Renderer) -> UpdateResult {
        self.menu_bar.render(canvas, renderer);
        if let Some(file) = self.files.get_mut(self.current_file) {
            file.render(canvas, renderer);
        }
        self.caret.render(canvas, renderer);
        UpdateResult::NoOp
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

impl ClickHandler for AppState {
    fn on_left_click(&mut self, point: &Point) -> UpdateResult {
        if self.menu_bar.is_left_click_target(point) {
            return self.menu_bar.on_left_click(point);
        }
        self.on_editor_clicked(point);
        UpdateResult::NoOp
    }

    fn is_left_click_target(&self, _point: &Point) -> bool {
        true
    }
}
