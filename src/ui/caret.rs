use crate::app::{UpdateResult, WindowCanvas};
use crate::config::Config;
use crate::renderer::Renderer;
use crate::ui::text_character::TextCharacter;
use crate::ui::*;
use sdl2::pixels::Color;
use sdl2::rect::{Point, Rect};
use sdl2::render::Texture;
use std::rc::Rc;
use std::ops::Deref;

#[derive(Clone, Debug, PartialEq)]
enum CaretState {
    Bright,
    Blur,
}

#[derive(Clone, Debug, PartialEq)]
pub enum MoveDirection {
    Left,
    Right,
    Up,
    Down
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
    pub fn new(text_position: usize, line_number: usize, line_position: usize,) -> Self {
        Self {
            text_position,
            line_number,
            line_position
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

    pub fn moved(&self, text_position: usize, line_number: usize, line_position: usize) -> Self {
        Self {
            text_position: self.text_position + text_position,
            line_number: self.line_number + line_number,
            line_position: self.line_position + line_position
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct CaretRenderPosition {
    dest: Rect,
    reset_position: Rect,
}

impl CaretRenderPosition {
    pub fn dest(&self) -> &Rect {
        &self.dest
    }

    pub fn reset_position(&self) -> &Rect {
        &self.reset_position
    }

    pub fn reset(&mut self) {
        self.dest = self.reset_position.clone()
    }

    pub fn move_to(&mut self, p: &Point) {
        self.dest.set_x(p.x());
        self.dest.set_y(p.y());
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct CaretColor {
    bright: Color,
    blur: Color,
}

impl CaretColor {
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
    render_position: CaretRenderPosition,
    colors: CaretColor,
}

impl Caret {
    pub fn new(config: Rc<Config>) -> Self {
        let bright = config.theme().caret().bright().color().into();
        let blur = config.theme().caret().blur().color().into();
        Self {
            state: CaretState::Bright,
            blink_delay: 0,
            render_position: CaretRenderPosition {
                dest: Rect::new(
                    config.editor_left_margin(),
                    config.editor_top_margin(),
                    4,
                    0,
                ),
                reset_position: Rect::new(
                    config.editor_left_margin(),
                    config.editor_top_margin(),
                    4,
                    0,
                ),
            },
            colors: CaretColor { bright, blur },
            pending: true,
            position: CaretPosition {
                text_position: 0,
                line_number: 0,
                line_position: 0,
            },
            //            config,
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
        self.render_position.reset();
        self.position.reset();
    }

    pub fn move_caret(&mut self, position: CaretPosition, pos: Point) {
        self.position = position;
        self.render_position.move_to(&pos);
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
    fn render(&mut self, canvas: &mut WindowCanvas, renderer: &mut Renderer) -> UpdateResult {
        if self.pending {
            if let Some(rect) = get_text_character_rect('W', renderer) {
                let mut dest = self.render_position.dest().clone();
                dest.set_height(rect.height());
                let reset_position = dest.clone();
                self.render_position = CaretRenderPosition {
                    dest,
                    reset_position,
                };
            }
            self.pending = false;
        }
        let dest = self.render_position.dest();
        let start = Point::new(dest.x(), dest.y());
        let end = Point::new(dest.x(), dest.y() + dest.height() as i32);
        let color = match self.state {
            CaretState::Bright => self.colors.bright(),
            CaretState::Blur => self.colors.blur(),
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
    fn on_left_click(&mut self, _point: &Point) -> UpdateResult {
        //        self.move_caret(Point::new(self.position.x(), self.position.y()));
        UpdateResult::NoOp
    }

    fn is_left_click_target(&self, point: &Point) -> bool {
        is_in_rect(point, &self.render_position.dest())
    }
}
