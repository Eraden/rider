pub mod caret;
pub mod text_character;

use crate::renderer::Renderer;
use crate::app::{WindowCanvas,UpdateResult};

pub trait Render {
    fn render(&mut self, canvas: &mut WindowCanvas, renderer: &mut Renderer) -> UpdateResult;
}

pub trait Update {
    fn update(&mut self, ticks: i32) -> UpdateResult;
}
