use sdl2::rect::{Point, Rect};
use std::rc::Rc;

use crate::app::{UpdateResult, WindowCanvas};
use crate::config::Config;
use crate::renderer::Renderer;
use crate::ui::file::editor_file_section::EditorFileSection;
use crate::ui::text_character::TextCharacter;
use crate::ui::*;

#[derive(Clone)]
pub struct EditorFile {
    path: String,
    sections: Vec<EditorFileSection>,
    render_position: Rect,
    buffer: String,
    config: Rc<Config>,
}

impl EditorFile {
    pub fn new(path: String, buffer: String, config: Rc<Config>) -> Self {
        use std::path::Path;

        let p = Path::new(&path);
        let ext = match p.extension() {
            Some(s) => s.to_str().unwrap_or("txt"),
            None => "txt",
        }
        .to_string();
        let sections = vec![EditorFileSection::new(buffer.clone(), ext, config.clone())];
        let x = config.editor_left_margin();
        let y = config.editor_top_margin();

        Self {
            path,
            sections,
            render_position: Rect::new(x, y, 0, 0),
            buffer,
            config,
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
                return Some(text_character);
            }
        }
        None
    }

    pub fn get_line(&self, line: &usize) -> Option<Vec<&TextCharacter>> {
        let mut vec: Vec<&TextCharacter> = vec![];

        for section in self.sections.iter() {
            match section.get_line(line) {
                Some(v) => vec.append(&mut v.clone()),
                _ => (),
            }
        }

        if vec.is_empty() {
            None
        } else {
            Some(vec)
        }
    }

    fn refresh_characters_position(&mut self) {
        let mut current: Rect = self.render_position.clone();
        for section in self.sections.iter_mut() {
            section.update_positions(&mut current);
        }
    }
}

impl Render for EditorFile {
    fn render(
        &self,
        canvas: &mut WindowCanvas,
        renderer: &mut Renderer,
        _parent: Option<&RenderBox>,
    ) -> UpdateResult {
        for section in self.sections.iter() {
            section.render(canvas, renderer, Some(self));
        }
        UpdateResult::NoOp
    }

    fn prepare_ui(&mut self, renderer: &mut Renderer) {
        for section in self.sections.iter_mut() {
            section.prepare_ui(renderer);
        }
        self.refresh_characters_position();
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

impl RenderBox for EditorFile {
    fn render_start_point(&self) -> Point {
        self.render_position.top_left()
    }
}
