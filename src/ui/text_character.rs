use std::rc::Rc;
use sdl2::rect::Rect;
use sdl2::render::Texture;
use sdl2::ttf::Font;
use sdl2::pixels::Color;

use crate::lexer::TokenType;
use crate::renderer::Renderer;
use crate::renderer::managers::TextDetails;
use crate::app::{UpdateResult, WindowCanvas};
use crate::renderer::managers::FontDetails;
use crate::ui::*;

#[derive(Clone)]
pub struct TextCharacter {
    pending: bool,
    text_character: char,
    line: usize,
    source: Rect,
    dest: Rect,
    color: Color,
}

impl TextCharacter {
    pub fn new(text_character: char, line: usize, color: Color) -> Self {
        Self {
            pending: true,
            text_character,
            line,
            source: Rect::new(0, 0, 0, 0),
            dest: Rect::new(0, 0, 0, 0),
            color,
        }
    }

    pub fn dest(&self) -> &Rect {
        &self.dest
    }

    pub fn source(&self) -> &Rect {
        &self.source
    }

    pub fn color(&self) -> &Color {
        &self.color
    }

    pub fn update_position(&mut self, current: &mut Rect) {
        if self.is_new_line() {
            let y = (self.line * self.source.height() as usize) as i32;
            current.set_x(0);
            current.set_y(y);
        } else {
            self.dest.set_x(current.x());
            self.dest.set_y(current.y());
            self.dest.set_width(self.source.width());
            self.dest.set_height(self.source.height());
            current.set_x(self.dest.x() + self.source.width() as i32);
        }
    }

    pub fn update_view(&mut self, renderer: &mut Renderer) -> UpdateResult {
        let config = &renderer.config.editor_config;
        let font_details = FontDetails::new(
            config.font_path.as_str(),
            config.character_size.clone(),
        );
        let font = renderer.font_manager
            .load(&font_details)
            .unwrap_or_else(|_| panic!("Font not found {:?}", font_details));

        let c = self.text_character.clone();
        if let Ok((width, height)) = font.size_of_char(c) {
            self.source = Rect::new(0, 0, width, height);
            self.dest = Rect::new(0, 0, width, height);
        }
        let mut details = TextDetails {
            text: c.to_string(),
            color: self.color.clone(),
            font: font_details.clone(),
        };
        renderer.texture_manager
            .load_text(&mut details, &font)
            .unwrap_or_else(|_| panic!("Could not create texture for {:?}", c));
        println!("texture for '{}' created", self.text_character);

        self.pending = false;
        UpdateResult::RefreshPositions
    }

    #[inline]
    fn is_new_line(&self) -> bool {
        self.text_character == '\n'
    }

    #[inline]
    fn is_pending(&self) -> bool {
        self.pending
    }
}

impl Render for TextCharacter {
    /**
    * Must first create targets so even if new line appear renderer will know
    * where move render starting point
    */
    fn render(&mut self, canvas: &mut WindowCanvas, renderer: &mut Renderer) -> UpdateResult {
        if self.is_pending() {
            return self.update_view(renderer);
        }
        if self.is_new_line() {
            return UpdateResult::NoOp;
        }

        let config = &renderer.config.editor_config;
        let font_details = FontDetails::new(
            config.font_path.as_str(),
            config.character_size.clone(),
        );
        let font = renderer.font_manager
            .load(&font_details)
            .unwrap_or_else(|_| panic!("Could not load font for {:?}", font_details));

        let c = self.text_character.clone();
        let mut details = TextDetails {
            text: c.to_string(),
            color: self.color.clone(),
            font: font_details.clone(),
        };
        if let Ok(texture) = renderer.texture_manager.load_text(&mut details, &font) {
            renderer.render_texture(canvas, &texture, &self.source, &self.dest);
        }
        UpdateResult::NoOp
    }
}

impl Update for TextCharacter {
    fn update(&mut self, _ticks: i32) -> UpdateResult {
        UpdateResult::NoOp
    }
}
