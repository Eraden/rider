use std::rc::Rc;
use sdl2::rect::{Point, Rect};

use crate::app::{UpdateResult, WindowCanvas};
use crate::config::Config;
use crate::ui::file::editor_file_section::EditorFileSection;
use crate::renderer::Renderer;
use crate::ui::*;
use crate::ui::text_character::TextCharacter;

#[derive(Clone)]
pub struct EditorFile {
    path: String,
    sections: Vec<EditorFileSection>,
    render_position: Rect,
    buffer: String,
    config: Rc<Config>
}

impl EditorFile {
    pub fn new(path: String, buffer: String, config: Rc<Config>) -> Self {
        let sections = vec![
            EditorFileSection::new(buffer.clone(), config.clone())
        ];
        let x = config.editor_left_margin();
        let y = config.editor_top_margin();
        Self {
            path,
            sections,
            render_position: Rect::new(x, y, 0, 0),
            buffer,
            config
        }
    }

    pub fn buffer(&self) -> String {
        self.buffer.clone()
    }

    pub fn path(&self) -> String {
        self.path.clone()
    }

    pub fn get_character_at(&self, index: usize) -> Option<&TextCharacter> {
        for section in self.sections.iter() {
            if let Some(text_character) = section.get_character_at(index) {
                return Some(text_character)
            }
        }
        None
    }

    fn refresh_characters_position(&mut self) {
        let mut current: Rect = self.render_position.clone();
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

impl ClickHandler for EditorFile {
    fn on_left_click(&mut self, point: &Point) -> UpdateResult {
        for section in self.sections.iter_mut() {
            if section.is_left_click_target(point) {
                return section.on_left_click(point);
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
