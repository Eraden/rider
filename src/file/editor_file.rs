use sdl2::rect::Rect;
use crate::file::editor_file_section::EditorFileSection;
use crate::renderer::Renderer;
use crate::app::{UpdateResult, WindowCanvas};
use crate::ui::*;

#[derive(Clone)]
pub struct EditorFile {
    pub path: String,
    pub sections: Vec<EditorFileSection>,
}

impl EditorFile {
    pub fn new(path: String, buffer: String) -> Self {
        let section = EditorFileSection::new(buffer);
        let sections = vec![section];
        Self { path, sections }
    }

    fn refresh_characters_position(&mut self) {
        let mut current: Rect = Rect::new(0, 0, 0, 0);
        for section in self.sections.iter_mut() {
            section.update_positions(&mut current);
        }
    }
}

impl Render for EditorFile {
    fn render(&mut self, canvas: &mut WindowCanvas, renderer: &mut Renderer) -> UpdateResult {
        let mut res = UpdateResult::NoOp;
        for section in self.sections.iter_mut() {
            res = section.render(canvas, renderer);
        }
        if res == UpdateResult::RefreshPositions {
            self.refresh_characters_position();
            for section in self.sections.iter_mut() {
                section.render(canvas, renderer);
            }
        }
        UpdateResult::NoOp
    }
}

impl Update for EditorFile {
    fn update(&mut self, ticks: i32) -> UpdateResult {
        let mut result = UpdateResult::NoOp;
        for section in self.sections.iter_mut() {
            result = section.update(ticks);
        }
        result
    }
}
