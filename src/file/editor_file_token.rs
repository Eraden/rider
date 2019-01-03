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
use crate::ui::text_character::*;

#[derive(Clone)]
pub struct TextCharacterMeasure {
    source: Rect,
    dest: Rect,
}

#[derive(Clone)]
pub struct EditorFileToken {
    characters: Vec<TextCharacter>,
    token_type: TokenType,
}

impl Into<Color> for TokenType {
    fn into(self) -> Color {
        match &self {
            &TokenType::Whitespace { .. } => Color::RGBA(220, 220, 220, 90),
            _ => Color::RGBA(0, 0, 0, 0),
        }
    }
}

impl EditorFileToken {
    pub fn new(token_type: TokenType) -> Self {
        Self {
            characters: vec![],
            token_type,
        }
    }

    pub fn update_position(&mut self, current: &mut Rect) {
        for text_character in self.characters.iter_mut() {
            text_character.update_position(current);
        }
    }

    fn update_view(&mut self, renderer: &mut Renderer) -> UpdateResult {
        for c in self.token_type.text().chars() {
            let mut text_character = TextCharacter::new(
                c.clone(),
                self.token_type.line(),
                self.token_type.clone().into(),
            );
            text_character.update_view(renderer);
            self.characters.push(text_character);
        }

        UpdateResult::RefreshPositions
    }
}

impl Render for EditorFileToken {
    /**
    * Must first create targets so even if new line appear renderer will know
    * where move render starting point
    */
    fn render(&mut self, canvas: &mut WindowCanvas, renderer: &mut Renderer) -> UpdateResult {
        if self.characters.is_empty() {
            return self.update_view(renderer);
        }
        if self.token_type.is_new_line() {
            return UpdateResult::NoOp;
        }
        for text_character in self.characters.iter_mut() {
            text_character.render(canvas, renderer);
        }
        UpdateResult::NoOp
    }
}

impl Update for EditorFileToken {
    fn update(&mut self, ticks: i32) -> UpdateResult {
        for text_character in self.characters.iter_mut() {
            text_character.update(ticks);
        }
        UpdateResult::NoOp
    }
}
