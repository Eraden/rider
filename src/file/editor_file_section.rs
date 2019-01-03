use sdl2::rect::Rect;
use crate::lexer::Language;
use crate::app::UpdateResult;
use crate::app::WindowCanvas;
use crate::renderer::Renderer;
use crate::file::editor_file_token::EditorFileToken;
use crate::ui::*;

#[derive(Clone)]
pub struct EditorFileSection {
    pub tokens: Vec<EditorFileToken>,
    pub language: Language,
}

impl EditorFileSection {
    pub fn new(buffer: String) -> Self {
        use crate::lexer;
        let lexer_tokens = lexer::parse(buffer.clone(), Language::PlainText);

        let mut tokens: Vec<EditorFileToken> = vec![];
        for token_type in lexer_tokens {
            let token = EditorFileToken::new(token_type);
            tokens.push(token.clone());
        }
        let language = Language::PlainText;
        Self { tokens, language }
    }

    pub fn update_positions(&mut self, current: &mut Rect) {
        for c in self.tokens.iter_mut() {
            c.update_position(current);
        }
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
