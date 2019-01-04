use sdl2::rect::{Point, Rect};

use crate::app::{UpdateResult, WindowCanvas};
use crate::config::Config;
use crate::ui::file::editor_file_token::EditorFileToken;
use crate::lexer::Language;
use crate::renderer::Renderer;
use crate::ui::*;
use crate::ui::text_character::TextCharacter;

#[derive(Clone)]
pub struct EditorFileSection {
    tokens: Vec<EditorFileToken>,
    language: Language,
}

impl EditorFileSection {
    pub fn new(buffer: String, config: &Config) -> Self {
        use crate::lexer;
        let lexer_tokens = lexer::parse(buffer.clone(), Language::PlainText);

        let mut tokens: Vec<EditorFileToken> = vec![];
        for token_type in lexer_tokens {
            let token = EditorFileToken::new(token_type, config);
            tokens.push(token.clone());
        }
        let language = Language::PlainText;
        Self { tokens, language }
    }

    pub fn update_positions(&mut self, current: &mut Rect, config: &Config) {
        for c in self.tokens.iter_mut() {
            c.update_position(current, config);
        }
    }

    pub fn get_character_at(&self, index: usize) -> Option<&TextCharacter> {
        for token in self.tokens.iter() {
            if let Some(text_character) = token.get_character_at(index) {
                return Some(text_character)
            }
        }
        None
    }
}

impl Render for EditorFileSection {
    fn render(&mut self, canvas: &mut WindowCanvas, renderer: &mut Renderer) -> UpdateResult {
        let mut res = UpdateResult::NoOp;
        for character in self.tokens.iter_mut() {
            let r = character.render(canvas, renderer);
            if res == UpdateResult::NoOp {
                res = r;
            }
        }
        res
    }
}

impl Update for EditorFileSection {
    fn update(&mut self, ticks: i32) -> UpdateResult {
        let mut result = UpdateResult::NoOp;
        for file_char in self.tokens.iter_mut() {
            result = file_char.update(ticks)
        }
        result
    }
}

impl ClickHandler for EditorFileSection {
    fn on_left_click(&mut self, point: &Point, config: &Config) -> UpdateResult {
        for token in self.tokens.iter_mut() {
            if token.is_left_click_target(point) {
                return token.on_left_click(point, config);
            }
        }
        UpdateResult::NoOp
    }

    fn is_left_click_target(&self, point: &Point) -> bool {
        for token in self.tokens.iter() {
            if token.is_left_click_target(point) {
                return true;
            }
        }
        false
    }
}
