use sdl2::rect::Rect;
use crate::file::editor_file_section::EditorFileSection;
use crate::renderer::Renderer;
use crate::app::UpdateResult;
use crate::app::WindowCanvas;

#[derive(Clone)]
pub struct EditorFile<'l> {
    pub path: String,
    pub sections: Vec<EditorFileSection<'l>>,
}

impl<'l> EditorFile<'l> {
    pub fn new(path: String, buffer: String, renderer: &'l mut Renderer) -> Self {
        let section = EditorFileSection::new(buffer, renderer);
        let sections = vec![section];
        Self { path, sections }
    }

    pub fn update(&mut self, ticks: i32) -> UpdateResult {
        let mut result = UpdateResult::NoOp;
        for section in self.sections.iter_mut() {
            result = section.update(ticks);
        }

        if result == UpdateResult::RefreshPositions {
            self.refresh_characters_position();
            result = UpdateResult::NoOp;
        }
        result
    }

    pub fn render(&self, canvas: &mut WindowCanvas, renderer: &mut Renderer) {
        for ref section in self.sections.iter() {
            section.render(canvas, renderer);
        }
    }

    fn refresh_characters_position(&mut self) {
        let mut current: Rect = Rect::new(0, 0, 0, 0);
        for section in self.sections.iter_mut() {
            section.update_positions(&mut current);
        }
    }
}
