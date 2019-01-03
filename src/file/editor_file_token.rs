use crate::app::{UpdateResult, WindowCanvas};
use crate::config::Config;
use crate::lexer::TokenType;
use crate::renderer::managers::{FontDetails, TextDetails};
use crate::renderer::Renderer;
use crate::ui::*;
use crate::ui::text_character::*;
use sdl2::pixels::Color;
use sdl2::rect::{Point, Rect};
use sdl2::render::Texture;
use sdl2::ttf::Font;
use std::rc::Rc;

#[derive(Clone)]
pub struct EditorFileToken {
    characters: Vec<TextCharacter>,
    token_type: TokenType,
}

impl EditorFileToken {
    pub fn new(token_type: TokenType, _config: &Config) -> Self {
        Self {
            characters: vec![],
            token_type,
        }
    }

    pub fn update_position(&mut self, current: &mut Rect, config: &Config) {
        for text_character in self.characters.iter_mut() {
            text_character.update_position(current, config);
        }
    }

    fn update_view(&mut self, renderer: &mut Renderer) -> UpdateResult {
        let config = renderer.config().theme().code_highlighting();
        let color: Color = match self.token_type {
            TokenType::Whitespace { .. } => config.whitespace().color().into(),
            TokenType::Keyword { .. } => config.keyword().color().into(),
            TokenType::String { .. } => config.string().color().into(),
            TokenType::Number { .. } => config.number().color().into(),
            TokenType::Identifier { .. } => config.identifier().color().into(),
            TokenType::Literal { .. } => config.literal().color().into(),
            TokenType::Comment { .. } => config.comment().color().into(),
            TokenType::Operator { .. } => config.operator().color().into(),
            TokenType::Separator { .. } => config.separator().color().into(),
        };
        for c in self.token_type.text().chars() {
            let mut text_character =
                TextCharacter::new(c.clone(), self.token_type.line(), color.clone());
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

impl ClickHandler for EditorFileToken {
    fn on_left_click(&mut self, point: &Point, config: &Config) -> UpdateResult {
        for text_character in self.characters.iter_mut() {
            if text_character.is_left_click_target(point) {
                return text_character.on_left_click(point, config);
            }
        }
        UpdateResult::NoOp
    }

    fn is_left_click_target(&self, point: &Point) -> bool {
        for text_character in self.characters.iter() {
            if text_character.is_left_click_target(point) {
                return true;
            }
        }
        false
    }
}
