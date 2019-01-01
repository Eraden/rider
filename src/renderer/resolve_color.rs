use sdl2::pixels::Color;

use crate::lexer::TokenType;

pub fn resolve_color(token_type: &TokenType) -> Color {
    match token_type {
        &TokenType::Whitespace { .. } => Color::RGBA(220, 220, 220, 90),
        _ => Color::RGBA(0, 0, 0, 0),
    }
}
