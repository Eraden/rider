use sdl2::rect::Rect;
use crate::lexer::Language;
use crate::app::UpdateResult;
use crate::app::WindowCanvas;
use crate::renderer::Renderer;
use crate::file::editor_file_token::EditorFileToken;

#[derive(Clone)]
pub struct EditorFileSection<'l> {
    pub tokens: Vec<EditorFileToken<'l>>,
    pub language: Language,
}

impl<'l> EditorFileSection<'l> {
    pub fn new(buffer: String, renderer: &'l mut Renderer) -> Self {
        use crate::lexer;
        let lexer_tokens = lexer::parse(buffer.clone(), Language::PlainText);

        let mut tokens: Vec<EditorFileToken> = vec![];
        for token_type in lexer_tokens {
            let token = EditorFileToken::new(
                renderer,
                token_type.get_start(),
                token_type.clone(),
            );
            tokens.push(token.clone());
        }
        let language = Language::PlainText;
        Self { tokens, language }
    }

    pub fn update(&mut self, ticks: i32) -> UpdateResult {
        let mut result = UpdateResult::NoOp;
        for file_char in self.tokens.iter_mut() {
            result = file_char.update(ticks)
        }
        result
    }

    pub fn render(&self, canvas: &mut WindowCanvas, renderer: &mut Renderer) {
        for ref character in self.tokens.iter() {
            character.render(canvas, renderer);
        }
    }

    pub fn update_positions(&mut self, current: &mut Rect) {
        for c in self.tokens.iter_mut() {
            c.update_position(current);
        }
    }
}
