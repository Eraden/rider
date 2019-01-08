use crate::app::{UpdateResult as UR, WindowCanvas as WC};
use crate::config::*;
use crate::lexer::TokenType;
use crate::renderer::managers::{FontDetails, TextDetails};
use crate::renderer::*;
use crate::ui::*;
use sdl2::pixels::Color;
use sdl2::rect::{Point, Rect};
use sdl2::render::Texture;
use sdl2::ttf::Font;
use std::rc::Rc;
use std::sync::*;

impl TokenType {
    pub fn to_color(&self, config: &Arc<RwLock<Config>>) -> Color {
        let config = config.read().unwrap();
        let ch = config.theme().code_highlighting();
        match self {
            &TokenType::Whitespace { .. } => ch.whitespace().color().into(),
            &TokenType::Keyword { .. } => ch.keyword().color().into(),
            &TokenType::String { .. } => ch.string().color().into(),
            &TokenType::Identifier { .. } => ch.identifier().color().into(),
            &TokenType::Literal { .. } => ch.literal().color().into(),
            &TokenType::Comment { .. } => ch.comment().color().into(),
            &TokenType::Operator { .. } => ch.operator().color().into(),
            &TokenType::Separator { .. } => ch.separator().color().into(),
        }
    }
}

#[derive(Clone, Debug)]
pub struct EditorFileToken {
    last_in_line: bool,
    characters: Vec<TextCharacter>,
    token_type: Rc<TokenType>,
    config: Arc<RwLock<Config>>,
}

impl EditorFileToken {
    pub fn new(token_type: &TokenType, last_in_line: bool, config: Arc<RwLock<Config>>) -> Self {
        Self {
            last_in_line,
            characters: vec![],
            token_type: Rc::new(token_type.clone()),
            config,
        }
    }

    pub fn is_last_in_line(&self) -> bool {
        self.last_in_line
    }

    pub fn update_position(&mut self, current: &mut Rect) {
        for text_character in self.characters.iter_mut() {
            text_character.update_position(current);
        }
    }
}

impl TextWidget for EditorFileToken {
    fn full_rect(&self) -> Rect {
        let mut rect = Rect::new(0, 0, 0, 0);
        match self.characters.first() {
            Some(c) => {
                rect.set_x(c.dest().x());
                rect.set_y(c.dest().y());
            },
            _ => return rect,
        };
        match self.characters.last() {
            Some(c) => {
                rect.set_width(c.dest().width());
                rect.set_height(c.dest().height());
            },
            _ => return rect,
        };
        rect
    }
}

impl TextCollection for EditorFileToken {
    fn get_character_at(&self, index: usize) -> Option<TextCharacter> {
        for character in self.characters.iter() {
            if character.position() == index {
                return Some(character.clone());
            }
        }
        None
    }

    fn get_line(&self, line: &usize) -> Option<Vec<&TextCharacter>> {
        let mut vec: Vec<&TextCharacter> = vec![];
        for c in self.characters.iter() {
            match (
                line.clone(),
                c.line().clone(),
                self.token_type.is_new_line(),
            ) {
                (0, 0, true) => {
                    vec.push(c);
                }
                (a, b, true) if (a + 1) == b => {
                    vec.push(c);
                }
                (a, b, true) if a != (b + 1) => (),
                (a, b, false) if a == b => {
                    vec.push(c);
                }
                _t => (),
            }
        }
        if vec.is_empty() {
            None
        } else {
            Some(vec)
        }
    }

    fn get_last_at_line(&self, line: usize) -> Option<TextCharacter> {
        let mut current: Option<&TextCharacter> = None;
        for text_character in self.characters.iter() {
            if !text_character.is_last_in_line() {
                continue;
            }
            if text_character.line() == line {
                current = Some(text_character);
            }
        }
        current.map(|c| c.clone())
    }
}

impl Render for EditorFileToken {
    /**
     * Must first create targets so even if new line appear renderer will know
     * where move render starting point
     */
    fn render(&self, canvas: &mut WC, renderer: &mut Renderer, parent: Parent) {
        if self.token_type.is_new_line() {
            return;
        }
        for text_character in self.characters.iter() {
            text_character.render(canvas, renderer, parent);
        }
    }

    fn prepare_ui(&mut self, renderer: &mut Renderer) {
        if !self.characters.is_empty() {
            return;
        }
        let color: Color = self.token_type.to_color(renderer.config());
        let chars: Vec<char> = self.token_type.text().chars().collect();
        for (index, c) in chars.iter().enumerate() {
            let last_in_line = self.last_in_line && index + 1 == chars.len();
            let mut text_character: TextCharacter = TextCharacter::new(
                c.clone(),
                self.token_type.start() + index,
                self.token_type.line(),
                last_in_line,
                color,
                self.config.clone(),
            );
            text_character.prepare_ui(renderer);
            self.characters.push(text_character);
        }
    }
}

impl Update for EditorFileToken {
    fn update(&mut self, ticks: i32, context: &UpdateContext) -> UR {
        for text_character in self.characters.iter_mut() {
            text_character.update(ticks, context);
        }
        UR::NoOp
    }
}

impl ClickHandler for EditorFileToken {
    fn on_left_click(&mut self, point: &Point, context: &UpdateContext) -> UR {
        for text_character in self.characters.iter_mut() {
            if text_character.is_left_click_target(point, context) {
                return text_character.on_left_click(point, context);
            }
        }
        UR::NoOp
    }

    fn is_left_click_target(&self, point: &Point, context: &UpdateContext) -> bool {
        for text_character in self.characters.iter() {
            if text_character.is_left_click_target(point, context) {
                return true;
            }
        }
        false
    }
}
