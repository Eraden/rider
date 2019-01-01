use std::rc::Rc;
use std::sync::Arc;
use std::boxed::Box;
use crate::app::{UpdateResult, WindowCanvas};
use crate::file::*;
use crate::renderer::Renderer;
use crate::file::editor_file::EditorFile;

pub struct AppState<'a> {
    pub files: Vec<EditorFile<'a>>,
    pub current_file: i16,
}

impl<'a> AppState<'a> {
    pub fn new() -> Self {
        Self {
            files: vec![],
            current_file: -1,
        }
    }

    pub fn open_file(&mut self, file_path: String, renderer: &mut Renderer) {
        use std::fs::read_to_string;
        if let Ok(buffer) = read_to_string(&file_path) {
            println!("read: {}\n{}", file_path, buffer);
            let file = EditorFile::new(file_path.clone(), buffer, renderer);
            self.current_file = self.files.len() as i16;
            self.files.push(file);
        };
    }

    pub fn update(&mut self, ticks: i32) -> UpdateResult {
        if let Some(ref mut file) = self.files.get(self.current_file as usize) {
            file.update(ticks);
        }
        UpdateResult::NoOp
    }

    pub fn render(&mut self, canvas: &mut WindowCanvas, renderer: &mut Renderer) {
        if let Some(ref mut file) = self.files.get(self.current_file as usize) {
            file.render(canvas, renderer);
        }
    }
}
