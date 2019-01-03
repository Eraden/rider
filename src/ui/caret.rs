use sdl2::rect::Rect;
use sdl2::render::Texture;
use sdl2::pixels::Color;
use crate::app::{WindowCanvas, UpdateResult};
use crate::ui::*;
use crate::ui::text_character::TextCharacter;
use crate::renderer::Renderer;

const CARET_CHARACTER: char = '│';

#[derive(Clone, Debug, PartialEq)]
enum CaretState {
    Bright,
    Blur,
}

#[derive(Clone)]
pub struct Caret {
    state: CaretState,
    bright_character: TextCharacter,
    blur_character: TextCharacter,
    blink_delay: u8,
}

impl Caret {
    pub fn new() -> Self {
        Self {
            bright_character: TextCharacter::new(CARET_CHARACTER, 0, Color::RGBA(0, 0, 0, 0)),
            blur_character: TextCharacter::new(CARET_CHARACTER, 0, Color::RGBA(100, 100, 100, 0)),
            state: CaretState::Bright,
            blink_delay: 0,
        }
    }

    fn toggle_state(&mut self) {
        self.state = if self.state == CaretState::Bright {
            CaretState::Blur
        } else {
            CaretState::Bright
        };
    }
}

impl Render for Caret {
    fn render(&mut self, canvas: &mut WindowCanvas, renderer: &mut Renderer) -> UpdateResult {
        match self.state {
            CaretState::Bright => {
                self.bright_character.update_position(&mut Rect::new(100, 220, 0, 0));
                self.bright_character.render(canvas, renderer)
            },
            CaretState::Blur => {
                self.blur_character.update_position(&mut Rect::new(100, 220, 0, 0));
                self.blur_character.render(canvas, renderer)
            },
        }
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
