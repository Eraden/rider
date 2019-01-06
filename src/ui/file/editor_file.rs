use sdl2::rect::{Point, Rect};
use std::rc::Rc;
use std::sync::*;

use crate::app::{UpdateResult as UR, WindowCanvas as WC};
use crate::config::Config;
use crate::renderer::Renderer;
use crate::ui::file::editor_file_section::EditorFileSection;
use crate::ui::text_character::TextCharacter;
use crate::ui::*;

#[derive(Clone, Debug)]
pub struct EditorFile {
    path: String,
    sections: Vec<EditorFileSection>,
    render_position: Rect,
    buffer: String,
    config: Arc<RwLock<Config>>,
    line_height: u32,
}

impl EditorFile {
    pub fn new(path: String, buffer: String, config: Arc<RwLock<Config>>) -> Self {
        use std::path::Path;
        let ext = Path::new(&path)
            .extension()
            .and_then(|p| p.to_str())
            .map_or("txt", |s| s)
            .to_string();
        let sections = vec![EditorFileSection::new(
            buffer.clone(),
            ext,
            Arc::clone(&config),
        )];
        let render_position = {
            let c = config.read().unwrap();
            let x = c.editor_left_margin();
            let y = c.editor_top_margin();
            Rect::new(x, y, 0, 0)
        };

        Self {
            path,
            sections,
            render_position,
            buffer,
            config,
            line_height: 0,
        }
    }

    pub fn buffer(&self) -> String {
        self.buffer.clone()
    }

    pub fn path(&self) -> String {
        self.path.clone()
    }

    pub fn sections(&self) -> &Vec<EditorFileSection> {
        &self.sections
    }

    pub fn line_height(&self) -> u32 {
        self.line_height
    }

    pub fn render_position(&self) -> &Rect {
        &self.render_position
    }

    pub fn get_character_at(&self, index: usize) -> Option<TextCharacter> {
        for section in self.sections.iter() {
            let character = section.get_character_at(index);
            if character.is_some() {
                return character;
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

    pub fn get_last_at_line(&self, line: usize) -> Option<TextCharacter> {
        let mut current = None;
        for section in self.sections.iter() {
            let c = section.get_last_at_line(line);
            if c.is_some() {
                current = c;
            }
        }
        current
    }

    pub fn get_section_at_mut(&mut self, index: usize) -> Option<&mut EditorFileSection> {
        self.sections.get_mut(index)
    }

    fn refresh_characters_position(&mut self) {
        let mut current: Rect = Rect::new(0, 0, 0, 0);
        for section in self.sections.iter_mut() {
            section.update_positions(&mut current);
        }
    }
}

impl Render for EditorFile {
    fn render(&self, canvas: &mut WC, renderer: &mut Renderer, parent: Parent) -> UR {
        for section in self.sections.iter() {
            section.render(canvas, renderer, parent);
        }
        UR::NoOp
    }

    fn prepare_ui(&mut self, renderer: &mut Renderer) {
        for section in self.sections.iter_mut() {
            section.prepare_ui(renderer);
        }
        if let Some(r) = get_text_character_rect('W', renderer) {
            self.line_height = r.height();
        }
        self.refresh_characters_position();
    }
}

impl Update for EditorFile {
    fn update(&mut self, ticks: i32, context: &UpdateContext) -> UR {
        let mut result = UR::NoOp;
        for section in self.sections.iter_mut() {
            result = section.update(ticks, context);
        }
        result
    }
}

impl ClickHandler for EditorFile {
    fn on_left_click(&mut self, point: &Point, context: &UpdateContext) -> UR {
        let mut index = -1;
        for (i, section) in self.sections.iter().enumerate() {
            if section.is_left_click_target(point, context) {
                index = i as i32;
                break;
            }
        }
        if index >= 0 {
            let context = UpdateContext::ParentPosition(self.render_start_point());
            return self
                .get_section_at_mut(index as usize)
                .unwrap()
                .on_left_click(point, &context);
        }
        UR::NoOp
    }

    fn is_left_click_target(&self, point: &Point, _context: &UpdateContext) -> bool {
        let context = UpdateContext::ParentPosition(self.render_start_point());
        for section in self.sections.iter() {
            if section.is_left_click_target(point, &context) {
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
