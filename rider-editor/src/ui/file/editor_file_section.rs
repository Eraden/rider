use sdl2::rect::{Point, Rect};
use std::sync::*;

use crate::app::UpdateResult as UR;
use crate::renderer::renderer::Renderer;
use crate::ui::file::editor_file_token::EditorFileToken;
use crate::ui::text_character::TextCharacter;
use crate::ui::*;
use rider_config::Config;
use rider_config::ConfigHolder;
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
        let lexer_tokens = rider_lexers::parse(buffer.clone(), language.clone());

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
        Self {
            tokens,
            language,
            config,
        }
    }

    pub fn language(&self) -> Language {
        self.language
    }

    pub fn update_positions(&mut self, current: &mut Rect) {
        for c in self.tokens.iter_mut() {
            c.update_position(current);
        }
    }

    pub fn render<R, C>(&self, canvas: &mut C, renderer: &mut R, context: &RenderContext)
    where
        R: Renderer + ConfigHolder,
        C: CanvasAccess,
    {
        for token in self.tokens.iter() {
            token.render(canvas, renderer, context);
        }
    }

    pub fn prepare_ui<'l, T>(&mut self, renderer: &mut T)
    where
        T: ConfigHolder + CharacterSizeManager + Renderer,
    {
        for token in self.tokens.iter_mut() {
            token.prepare_ui(renderer);
        }
    }

    fn iter_char(&self) -> EditorFileSectionIterator {
        EditorFileSectionIterator::new(self)
    }

    pub fn tokens(&self) -> &Vec<EditorFileToken> {
        &self.tokens
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

pub struct EditorFileSectionIterator<'a> {
    section: &'a EditorFileSection,
    current_token: usize,
    current_character: usize,
}

impl<'a> EditorFileSectionIterator<'a> {
    pub fn new(section: &'a EditorFileSection) -> Self {
        Self {
            section,
            current_token: 0,
            current_character: 0,
        }
    }

    fn get_token(&self) -> Option<&'a EditorFileToken> {
        self.section.tokens.get(self.current_token)
    }

    fn get_character(&mut self, token: &'a EditorFileToken) -> Option<&'a TextCharacter> {
        token
            .characters()
            .get(self.current_character)
            .or_else(|| {
                self.current_character = 0;
                self.current_token += 1;
                self.get_character(self.get_token()?)
            })
            .and_then(|c| {
                self.current_character += 1;
                Some(c)
            })
    }
}

impl<'a> std::iter::Iterator for EditorFileSectionIterator<'a> {
    type Item = &'a TextCharacter;

    fn next(&mut self) -> Option<Self::Item> {
        self.get_character(self.get_token()?)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::support::{build_config, SimpleRendererMock};

    impl EditorFileSection {
        pub fn tokens_count(&self) -> usize {
            self.tokens.len()
        }
    }

    #[test]
    fn assert_new() {
        let config = build_config();
        let widget = EditorFileSection::new("".to_owned(), "rs".to_owned(), config);
        assert_eq!(widget.language(), Language::Rust);
        assert_eq!(widget.tokens_count(), 0);
    }

    #[test]
    fn assert_new_with_content() {
        let config = build_config();
        let widget = EditorFileSection::new("fn main() {}".to_owned(), "rs".to_owned(), config);
        assert_eq!(widget.language(), Language::Rust);
        assert_eq!(widget.tokens_count(), 8);
    }

    #[test]
    fn assert_simple_char_iteration() {
        let config = build_config();
        let mut renderer = SimpleRendererMock::new(config.clone());
        let mut section = EditorFileSection::new("a b c d".to_owned(), ".txt".to_owned(), config);
        section.prepare_ui(&mut renderer);
        for (index, c) in section.iter_char().enumerate() {
            match index {
                0 => assert_eq!(c.text_character(), 'a'),
                1 => assert_eq!(c.text_character(), ' '),
                2 => assert_eq!(c.text_character(), 'b'),
                3 => assert_eq!(c.text_character(), ' '),
                4 => assert_eq!(c.text_character(), 'c'),
                5 => assert_eq!(c.text_character(), ' '),
                6 => assert_eq!(c.text_character(), 'd'),
                _ => assert_eq!("must have 7 entries", "have more than 7 entries"),
            }
        }
    }

    #[test]
    fn assert_complex_char_iteration() {
        let config = build_config();
        let mut renderer = SimpleRendererMock::new(config.clone());
        let mut section = EditorFileSection::new("let a = 1".to_owned(), ".rs".to_owned(), config);
        section.prepare_ui(&mut renderer);
        assert_eq!(section.tokens.len(), 7);
        for (index, c) in section.iter_char().enumerate() {
            match index {
                0 => assert_eq!(c.text_character(), 'l'),
                1 => assert_eq!(c.text_character(), 'e'),
                2 => assert_eq!(c.text_character(), 't'),
                3 => assert_eq!(c.text_character(), ' '),
                4 => assert_eq!(c.text_character(), 'a'),
                5 => assert_eq!(c.text_character(), ' '),
                6 => assert_eq!(c.text_character(), '='),
                7 => assert_eq!(c.text_character(), ' '),
                8 => assert_eq!(c.text_character(), '1'),
                _ => assert_eq!("must have 9 entries", "have more than 9 entries"),
            }
        }
    }
}
