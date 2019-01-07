use crate::app::{UpdateResult as UR, WindowCanvas as WC};
use crate::config::Config;
use crate::renderer::Renderer;
use crate::ui::text_character::TextCharacter;
use crate::ui::*;
use sdl2::pixels::Color;
use sdl2::rect::{Point, Rect};
use sdl2::render::Texture;
use std::ops::Deref;
use std::rc::Rc;
use std::sync::*;

#[derive(Clone, Debug, PartialEq)]
pub enum CaretState {
    Bright,
    Blur,
}

#[derive(Clone, Debug, PartialEq)]
pub enum MoveDirection {
    Left,
    Right,
    Up,
    Down,
}

//#[derive(Clone, Debug, PartialEq)]
//pub enum CaretLocation {
//    FirstLineFirstCharacter,
//    FirstLine(usize), // with character location
//    LastLineFirstCharacter,
//    LastLine(usize), // with character location
//    FirstCharacter(usize),// with line number
//    LastCharacter(usize), // with line number
//    Other(usize, usize), // with line number and character number
//}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CaretPosition {
    text_position: usize,
    line_number: usize,
    line_position: usize,
}

impl CaretPosition {
    pub fn new(text_position: usize, line_number: usize, line_position: usize) -> Self {
        Self {
            text_position,
            line_number,
            line_position,
        }
    }

    pub fn text_position(&self) -> usize {
        self.text_position.clone()
    }

    pub fn line_number(&self) -> usize {
        self.line_number.clone()
    }

    pub fn line_position(&self) -> usize {
        self.line_position.clone()
    }

    pub fn reset(&mut self) {
        self.text_position = 0;
        self.line_number = 0;
        self.line_position = 0;
    }

    pub fn set_text_position(&mut self, n: usize) {
        self.text_position = n;
    }

    pub fn set_line_number(&mut self, n: usize) {
        self.line_number = n;
    }

    pub fn set_line_position(&mut self, n: usize) {
        self.line_position = n;
    }

    pub fn moved(&self, text_position: i32, line_number: i32, line_position: i32) -> Self {
        Self {
            text_position: (self.text_position as i32 + text_position) as usize,
            line_number: (self.line_number as i32 + line_number) as usize,
            line_position: (self.line_position as i32 + line_position) as usize,
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct CaretRenderPosition {
    dest: Rect,
    reset_position: Rect,
}

#[derive(Clone, Debug, PartialEq)]
pub struct CaretColor {
    bright: Color,
    blur: Color,
}

impl CaretColor {
    pub fn new(bright: Color, blur: Color) -> Self {
        Self { bright, blur }
    }

    pub fn bright(&self) -> &Color {
        &self.bright
    }

    pub fn blur(&self) -> &Color {
        &self.blur
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Caret {
    pending: bool,
    blink_delay: u8,
    state: CaretState,
    position: CaretPosition,
    dest: Rect,
    colors: CaretColor,
}

impl Caret {
    pub fn new(config: Arc<RwLock<Config>>) -> Self {
        let read_config = config.read().unwrap();
        let bright = read_config.theme().caret().bright().color().into();
        let blur = read_config.theme().caret().blur().color().into();
        Self {
            state: CaretState::Bright,
            blink_delay: 0,
            dest: Rect::new(0, 0, 6, 0),
            colors: CaretColor { bright, blur },
            pending: true,
            position: CaretPosition {
                text_position: 0,
                line_number: 0,
                line_position: 0,
            },
        }
    }

    fn toggle_state(&mut self) {
        self.state = if self.state == CaretState::Bright {
            CaretState::Blur
        } else {
            CaretState::Bright
        };
    }

    pub fn reset_caret(&mut self) {
        self.dest.set_x(0);
        self.dest.set_y(0);
        self.position.reset();
    }

    pub fn move_caret(&mut self, position: CaretPosition, pos: Point) {
        self.position = position;
        self.dest.set_x(pos.x());
        self.dest.set_y(pos.y());
    }

    pub fn dest(&self) -> &Rect {
        &self.dest
    }

    pub fn position(&self) -> &CaretPosition {
        &self.position
    }
}

impl Deref for Caret {
    type Target = CaretPosition;

    fn deref(&self) -> &<Self as Deref>::Target {
        self.position()
    }
}

impl Render for Caret {
    fn render(&self, canvas: &mut WC, _renderer: &mut Renderer, parent: Parent) {
        let dest = match parent {
            Some(parent) => move_render_point(parent.render_start_point(), self.dest()),
            None => self.dest().clone(),
        };
        let start = Point::new(dest.x(), dest.y());
        let end = Point::new(dest.x(), dest.y() + dest.height() as i32);
        let color = match self.state {
            CaretState::Bright => self.colors.bright(),
            CaretState::Blur => self.colors.blur(),
        }
        .clone();
        canvas.set_draw_color(color);
        canvas
            .draw_line(start, end)
            .unwrap_or_else(|_| panic!("Failed to draw a caret"));
    }

    fn prepare_ui(&mut self, renderer: &mut Renderer) {
        if !self.pending {
            return;
        }

        if let Some(rect) = get_text_character_rect('W', renderer) {
            self.dest.set_height(rect.height());
        }
        self.pending = false;
    }
}

impl Update for Caret {
    fn update(&mut self, _ticks: i32, _context: &UpdateContext) -> UR {
        self.blink_delay += 1;
        if self.blink_delay >= 30 {
            self.blink_delay = 0;
            self.toggle_state();
        }
        UR::NoOp
    }
}

impl ClickHandler for Caret {
    fn on_left_click(&mut self, _point: &Point, _context: &UpdateContext) -> UR {
        UR::NoOp
    }

    fn is_left_click_target(&self, point: &Point, context: &UpdateContext) -> bool {
        is_in_rect(
            point,
            &match context {
                &UpdateContext::ParentPosition(p) => move_render_point(p, self.dest()),
                _ => self.dest().clone(),
            },
        )
    }
}

impl RenderBox for Caret {
    fn render_start_point(&self) -> Point {
        self.dest().top_left()
    }
}
