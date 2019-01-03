use crate::app::{UpdateResult, WindowCanvas};
use crate::config::Config;
use crate::file::editor_file_section::EditorFileSection;
use crate::renderer::Renderer;
use crate::ui::*;
use sdl2::rect::{Point, Rect};

#[derive(Clone)]
pub struct EditorFile {
    path: String,
    sections: Vec<EditorFileSection>,
    render_position: Rect,
}

impl EditorFile {
    pub fn new(path: String, buffer: String, config: &Config) -> Self {
        let section = EditorFileSection::new(buffer, config);
        let sections = vec![section];
        let x = config.editor_left_margin();
        let y = config.editor_top_margin();
        Self {
            path,
            sections,
            render_position: Rect::new(x, y, 0, 0),
        }
    }

    fn refresh_characters_position(&mut self, config: &Config) {
        let mut current: Rect = self.render_position.clone();
        for section in self.sections.iter_mut() {
            section.update_positions(&mut current, config);
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
            self.refresh_characters_position(renderer.config());
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

impl ClickHandler for EditorFile {
    fn on_left_click(&mut self, point: &Point, config: &Config) -> UpdateResult {
        for section in self.sections.iter_mut() {
            if section.is_left_click_target(point) {
                return section.on_left_click(point, config);
            }
        }
        UpdateResult::NoOp
    }

    fn is_left_click_target(&self, point: &Point) -> bool {
        for section in self.sections.iter() {
            if section.is_left_click_target(point) {
                return true;
            }
        }
        false
    }
}
