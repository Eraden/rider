use crate::app::{UpdateResult as UR, WindowCanvas as WC};
use crate::config::Config;
use crate::lexer::TokenType;
use crate::renderer::managers::{FontDetails, TextDetails};
use crate::renderer::Renderer;
use crate::ui::caret::CaretPosition;
use crate::ui::*;

use sdl2::pixels::Color;
use sdl2::rect::{Point, Rect};
use sdl2::render::Texture;
use sdl2::ttf::Font;
use std::rc::Rc;

#[derive(Clone, Debug)]
pub struct TextCharacter {
    text_character: char,
    position: usize,
    line: usize,
    last_in_line: bool,
    source: Rect,
    dest: Rect,
    color: Color,
    config: Rc<Config>,
}

impl TextCharacter {
    pub fn new(
        text_character: char,
        position: usize,
        line: usize,
        last_in_line: bool,
        color: Color,
        config: Rc<Config>,
    ) -> Self {
        Self {
            text_character,
            position,
            line,
            last_in_line,
            source: Rect::new(0, 0, 0, 0),
            dest: Rect::new(0, 0, 0, 0),
            color,
            config,
        }
    }

    pub fn is_last_in_line(&self) -> bool {
        self.last_in_line
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
            let y = self.source.height() as i32;
            current.set_x(0);
            current.set_y(current.y() + y);
            self.dest.set_x(current.x());
            self.dest.set_y(current.y());
        } else {
            self.dest.set_x(current.x());
            self.dest.set_y(current.y());
            self.dest.set_width(self.source.width());
            self.dest.set_height(self.source.height());
            current.set_x(self.dest.x() + self.source.width() as i32);
        }
    }

    #[inline]
    pub fn is_new_line(&self) -> bool {
        self.text_character == '\n'
    }

    pub fn position(&self) -> usize {
        self.position
    }

    pub fn line(&self) -> usize {
        self.line
    }

    pub fn text_character(&self) -> char {
        self.text_character.clone()
    }
}

impl Render for TextCharacter {
    /**
     * Must first create targets so even if new line appear renderer will know
     * where move render starting point
     */
    fn render(&self, canvas: &mut WC, renderer: &mut Renderer, parent: Parent) -> UR {
        if self.is_new_line() {
            return UR::NoOp;
        }

        let config = renderer.config().editor_config();
        let font_details =
            FontDetails::new(config.font_path().as_str(), config.character_size().clone());
        let font = renderer
            .font_manager()
            .load(&font_details)
            .unwrap_or_else(|_| panic!("Could not load font for {:?}", font_details));

        let c = self.text_character.clone();
        let mut details = TextDetails {
            text: c.to_string(),
            color: self.color.clone(),
            font: font_details.clone(),
        };
        let dest = match parent {
            None => self.dest.clone(),
            Some(parent) => move_render_point(parent.render_start_point(), self.dest()),
        };
        if let Ok(texture) = renderer.texture_manager().load_text(&mut details, &font) {
            renderer.render_texture(canvas, &texture, &self.source, &dest);
        }
//        let c = Color::RGB(255, 0, 0);
//        canvas.set_draw_color(c);
//        canvas.draw_rect(dest.clone()).unwrap();
        UR::NoOp
    }

    fn prepare_ui(&mut self, renderer: &mut Renderer) {
        let config = renderer.config().editor_config();
        let font_details =
            FontDetails::new(config.font_path().as_str(), config.character_size().clone());
        let font = renderer
            .font_manager()
            .load(&font_details)
            .unwrap_or_else(|_| panic!("Font not found {:?}", font_details));

        let c = match self.text_character {
            '\n' => 'W',
            c => c,
        };
        if let Some(rect) = get_text_character_rect(c, renderer) {
            self.source = rect.clone();
            self.dest = rect.clone();
        }
        let mut details = TextDetails {
            text: self.text_character.to_string(),
            color: self.color.clone(),
            font: font_details.clone(),
        };
        renderer
            .texture_manager()
            .load_text(&mut details, &font)
            .unwrap_or_else(|_| panic!("Could not create texture for {:?}", self.text_character));
    }
}

impl Update for TextCharacter {
    fn update(&mut self, _ticks: i32, _context: &UpdateContext) -> UR {
        UR::NoOp
    }
}

impl ClickHandler for TextCharacter {
    fn on_left_click(&mut self, _point: &Point, _context: &UpdateContext) -> UR {
        UR::MoveCaret(
            self.dest().clone(),
            CaretPosition::new(self.position(), self.line(), 0),
        )
    }

    fn is_left_click_target(&self, point: &Point, context: &UpdateContext) -> bool {
        is_in_rect(
            point,
            &match context {
                &UpdateContext::ParentPosition(p) => move_render_point(p.clone(), self.dest()),
                _ => self.dest().clone(),
            },
        )
    }
}

impl RenderBox for TextCharacter {
    fn render_start_point(&self) -> Point {
        self.dest.top_left()
    }
}
