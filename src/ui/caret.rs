use crate::app::{UpdateResult, WindowCanvas};
use crate::config::Config;
use crate::renderer::Renderer;
use crate::ui::*;
use crate::ui::text_character::TextCharacter;
use sdl2::pixels::Color;
use sdl2::rect::{Point, Rect};
use sdl2::render::Texture;

#[derive(Clone, Debug, PartialEq)]
enum CaretState {
    Bright,
    Blur,
}

pub struct Caret {
    state: CaretState,
    blink_delay: u8,
    position: Rect,
    bright_character_color: Color,
    blur_character_color: Color,
    pending: bool,
}

impl Caret {
    pub fn new(config: &Config) -> Self {
        let bright_character_color = config.theme().caret().bright().color().into();
        let blur_character_color = config.theme().caret().blur().color().into();
        Self {
            state: CaretState::Bright,
            blink_delay: 0,
            position: Rect::new(
                config.editor_left_margin(),
                config.editor_top_margin(),
                4,
                0,
            ),
            bright_character_color,
            blur_character_color,
            pending: true,
        }
    }

    fn toggle_state(&mut self) {
        self.state = if self.state == CaretState::Bright {
            CaretState::Blur
        } else {
            CaretState::Bright
        };
    }

    pub fn move_caret(&mut self, pos: Point) {
        self.position.set_x(pos.x());
        self.position.set_y(pos.y());
    }
}

impl Render for Caret {
    fn render(&mut self, canvas: &mut WindowCanvas, renderer: &mut Renderer) -> UpdateResult {
        if self.pending {
            use crate::renderer::managers::FontDetails;
            let config = renderer.config().clone();
            let font = renderer
                .font_manager()
                .load(&FontDetails {
                    path: config.editor_config().font_path().clone(),
                    size: config.editor_config().character_size(),
                })
                .unwrap_or_else(|_| panic!("Unable to load font"));
            if let Ok((_, h)) = font.size_of_char('W') {
                self.position.set_height(h);
            }
            self.pending = false;
        }
        let start = Point::new(self.position.x(), self.position.y());
        let end = Point::new(
            self.position.x(),
            self.position.y() + self.position.height() as i32,
        );
        let color = match self.state {
            CaretState::Bright => &self.bright_character_color,
            CaretState::Blur => &self.blur_character_color,
        };
        canvas.set_draw_color(color.clone());
        canvas
            .draw_line(start, end)
            .unwrap_or_else(|_| panic!("Failed to draw a caret"));
        UpdateResult::NoOp
    }
}

impl Update for Caret {
    fn update(&mut self, _ticks: i32) -> UpdateResult {
        self.blink_delay += 1;
        if self.blink_delay >= 30 {
            self.blink_delay = 0;
            self.toggle_state();
        }
        UpdateResult::NoOp
    }
}

impl ClickHandler for Caret {
    fn on_left_click(&mut self, _point: &Point, _config: &Config) -> UpdateResult {
        //        self.move_caret(Point::new(self.position.x(), self.position.y()));
        UpdateResult::NoOp
    }

    fn is_left_click_target(&self, point: &Point) -> bool {
        is_in_rect(point, &self.position)
    }
}
