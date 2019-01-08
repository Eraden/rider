use sdl2::rect::{Point, Rect};
use std::sync::*;

use crate::app::{UpdateResult as UR, WindowCanvas as WC};
use crate::renderer::Renderer;
use crate::ui::file::editor_file_token::EditorFileToken;
use crate::ui::text_character::TextCharacter;
use crate::ui::*;
use rider_config::Config;
use rider_lexers;
use rider_lexers::Language;

#[derive(Clone, Debug)]
pub struct EditorFileSection {
    tokens: Vec<EditorFileToken>,
    language: Language,
    config: Arc<RwLock<Config>>,
}

impl EditorFileSection {
    pub fn new(buffer: String, ext: String, config: Arc<RwLock<Config>>) -> Self {
        let language = config
            .read()
            .unwrap()
            .extensions_mapping()
            .get(ext.as_str())
            .unwrap_or(&Language::PlainText)
            .clone();
        let lexer_tokens = rider_lexers::parse(buffer.clone(), &language);

        let mut tokens: Vec<EditorFileToken> = vec![];
        let mut iterator = lexer_tokens.iter().peekable();
        loop {
            let token_type = match iterator.next() {
                Some(t) => t,
                _ => break,
            };
            let next = iterator.peek();
            let token = EditorFileToken::new(
                token_type,
                next.map_or(true, |t| t.is_new_line()),
                config.clone(),
            );
            tokens.push(token);
        }
        let language = Language::PlainText;
        Self {
            tokens,
            language,
            config,
        }
    }

    pub fn update_positions(&mut self, current: &mut Rect) {
        for c in self.tokens.iter_mut() {
            c.update_position(current);
        }
    }
}

impl TextWidget for EditorFileSection {
    fn full_rect(&self) -> Rect {
        let mut current_line_width = 0;
        let mut max_line_width = 0;
        let mut height = 0;
        for (index, token) in self.tokens.iter().enumerate() {
            let r = token.full_rect();

            if index == 0 {
                height = r.height();
                current_line_width = r.width();
                max_line_width = r.width();
            } else if token.is_new_line() {
                height += r.height();
                if max_line_width < current_line_width {
                    max_line_width = current_line_width;
                }
                current_line_width = 0;
            } else {
                current_line_width += r.width();
            }
        }
        Rect::new(0, 0, max_line_width, height)
    }
}

impl TextCollection for EditorFileSection {
    fn get_character_at(&self, index: usize) -> Option<TextCharacter> {
        for token in self.tokens.iter() {
            let character = token.get_character_at(index);
            if character.is_some() {
                return character;
            }
        }
        None
    }

    fn get_line(&self, line: &usize) -> Option<Vec<&TextCharacter>> {
        let mut vec: Vec<&TextCharacter> = vec![];
        for token in self.tokens.iter() {
            match token.get_line(line) {
                Some(v) => vec.append(&mut v.clone()),
                _ => (),
            };
        }
        if vec.is_empty() {
            None
        } else {
            Some(vec)
        }
    }

    fn get_last_at_line(&self, line: usize) -> Option<TextCharacter> {
        let mut current: Option<TextCharacter> = None;
        for token in self.tokens.iter() {
            if !token.is_last_in_line() {
                continue;
            }
            let c = token.get_last_at_line(line);
            if c.is_some() {
                current = c;
            }
        }
        current
    }
}

impl Render for EditorFileSection {
    fn render(&self, canvas: &mut WC, renderer: &mut Renderer, context: &RenderContext) {
        for token in self.tokens.iter() {
            token.render(canvas, renderer, context);
        }
    }

    fn prepare_ui(&mut self, renderer: &mut Renderer) {
        for token in self.tokens.iter_mut() {
            token.prepare_ui(renderer);
        }
    }
}

impl Update for EditorFileSection {
    fn update(&mut self, ticks: i32, context: &UpdateContext) -> UR {
        let mut result = UR::NoOp;
        for token in self.tokens.iter_mut() {
            result = token.update(ticks, context)
        }
        result
    }
}

impl ClickHandler for EditorFileSection {
    fn on_left_click(&mut self, point: &Point, context: &UpdateContext) -> UR {
        for token in self.tokens.iter_mut() {
            if token.is_left_click_target(point, context) {
                return token.on_left_click(point, context);
            }
        }
        UR::NoOp
    }

    fn is_left_click_target(&self, point: &Point, context: &UpdateContext) -> bool {
        let mut i = 0;
        loop {
            if i == self.tokens.len() {
                break;
            }
            match self.tokens.get(i) {
                Some(token) => {
                    if token.is_left_click_target(point, context) {
                        return true;
                    }
                }
                None => break,
            }
            i += 1;
        }
        false
    }
}
