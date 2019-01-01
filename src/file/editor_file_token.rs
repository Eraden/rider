use std::rc::Rc;
use sdl2::rect::Rect;
use sdl2::render::Texture;
use sdl2::ttf::Font;
use crate::lexer::TokenType;
use crate::renderer::Renderer;
use crate::renderer::managers::TextDetails;
use crate::renderer::resolve_color::resolve_color;
use crate::app::UpdateResult;
use crate::app::WindowCanvas;
use crate::renderer::managers::FontDetails;

#[derive(Clone)]
pub struct EditorFileToken<'l> {
    pos: usize,
    text: String,
    font_size: u16,
    source: Rect,
    dest: Rect,
    token_type: TokenType,
    texture: Option<Rc<Texture<'l>>>,
}

impl<'l> EditorFileToken<'l> {
    pub fn new(renderer: &'l mut Renderer, pos: usize, token_type: TokenType) -> Self {
        let c = match token_type {
            _ if token_type.is_space() => "°".to_string(),
            _ if token_type.is_new_line() => "\n".to_string(),
            TokenType::Whitespace { .. } => "°".to_string(),
            _ => token_type.get_text(),
        };
        let details = TextDetails {
            text: c.clone(),
            font_details: FontDetails::new(
                renderer.config.editor_config.font_path.as_str(),
                renderer.config.editor_config.character_size.clone(),
            ),
            color: resolve_color(&token_type),
        };
        Self {
            pos,
            text: c,
            font_size: 0,
            source: Rect::new(0, 0, 0, 0),
            dest: Rect::new(0, 0, 0, 0),
            token_type,
            texture: renderer.render_text(details).clone(),
        }
    }

    pub fn update(&mut self, _ticks: i32) -> UpdateResult {
//        if self.font_size != config.editor_config.character_size {
//            self.update_view(renderer);
//            return UpdateResult::RefreshPositions;
//        }
        UpdateResult::NoOp
    }

    pub fn render(&self, canvas: &mut WindowCanvas, renderer: &mut Renderer) {
        if self.token_type.is_new_line() {
            return;
        }
        match &self.texture {
            Some(texture) => {
                renderer.render_texture(canvas, &texture, &self.source, &self.dest)
            }
            _ => {}
        }
    }

    pub fn update_position(&mut self, current: &mut Rect) {
        match self.token_type {
            _ if self.token_type.is_new_line() => {
                current.set_x(0);
                current.set_y(
                    (self.token_type.line() as usize * self.source.height() as usize) as i32,
                );
            }
            _ => {
                self.dest.set_x(current.x());
                self.dest.set_y(current.y());
                self.dest.set_width(self.source.width());
                self.dest.set_height(self.source.height());
                current.set_x(self.dest.x() + self.source.width() as i32);
            }
        };
    }

    fn update_view(&mut self, renderer: &mut Renderer) {
        self.font_size = renderer.config.editor_config.character_size.clone();
        let font_details = FontDetails::new(
            renderer.config.editor_config.font_path.as_str(),
            self.font_size.clone(),
        );

        if let Ok(font) = renderer.font_manager.load(&font_details) {
            if let Some((width, height)) = self.measure_text(&font) {
                self.source.set_width(width as u32);
                self.source.set_height(height as u32);
            }
        };
    }

    fn measure_text(&self, font: &Rc<Font>) -> Option<(usize, usize)> {
        let mut w: usize = 0;
        let mut h: usize = 0;
        for c in self.text.chars() {
            if let Ok((width, height)) = font.size_of_char(c) {
                w += width as usize;
                h = height as usize;
            } else {
                return None;
            }
        }
        Some((w, h))
    }
}
