use std::rc::Rc;
use std::sync::Arc;
use std::boxed::Box;
use crate::app::{UpdateResult, WindowCanvas};
use crate::ui::*;
use crate::ui::caret::Caret;
use crate::file::*;
use crate::file::editor_file::EditorFile;
use crate::renderer::Renderer;

pub struct AppState {
    pub files: Vec<EditorFile>,
    pub current_file: i16,
    caret: Caret,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            files: vec![],
            current_file: -1,
            caret: Caret::new(),
        }
    }

    pub fn open_file(&mut self, file_path: String) {
        use std::fs::read_to_string;
        if let Ok(buffer) = read_to_string(&file_path) {
            println!("read: {}\n{}", file_path, buffer);
            let file = EditorFile::new(file_path.clone(), buffer);
            self.current_file = self.files.len() as i16;
            self.files.push(file);
        };
    }
}

impl Render for AppState {
    fn render(&mut self, canvas: &mut WindowCanvas, renderer: &mut Renderer) -> UpdateResult {
        if let Some(file) = self.files.get_mut(self.current_file as usize) {
            file.render(canvas, renderer);
        }
        self.caret.render(canvas, renderer);
        UpdateResult::NoOp
    }
}

impl Update for AppState {
    fn update(&mut self, ticks: i32) -> UpdateResult {
        if let Some(file) = self.files.get_mut(self.current_file as usize) {
            file.update(ticks);
        }
        self.caret.update(ticks);
        UpdateResult::NoOp
    }
}
