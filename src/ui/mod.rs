use sdl2::rect::{Point, Rect};

use crate::app::{UpdateResult, WindowCanvas};
use crate::config::Config;
use crate::renderer::Renderer;
use crate::renderer::managers::FontDetails;

pub mod caret;
pub mod menu_bar;
pub mod text_character;
pub mod file;

pub fn is_in_rect(point: &Point, rect: &Rect) -> bool {
    let start = Point::new(rect.x(), rect.y());
    let end = Point::new(
        rect.x() + (rect.width() as i32),
        rect.y() + (rect.height() as i32),
    );
    start.x() <= point.x() && start.y() <= point.y() && end.x() >= point.x() && end.y() >= point.y()
}

pub fn get_text_character_rect(c: char, renderer: &mut Renderer) -> Option<Rect> {
    let config = renderer.config().editor_config();
    let font_details =
        FontDetails::new(config.font_path().as_str(), config.character_size().clone());
    let font = renderer
        .font_manager()
        .load(&font_details)
        .unwrap_or_else(|_| panic!("Font not found {:?}", font_details));

    if let Ok((width, height)) = font.size_of_char(c) {
        Some(Rect::new(0, 0, width, height))
    } else {
        None
    }
}

pub trait Render {
    fn render(&mut self, canvas: &mut WindowCanvas, renderer: &mut Renderer) -> UpdateResult;
}

pub trait Update {
    fn update(&mut self, ticks: i32) -> UpdateResult;
}

pub trait ClickHandler {
    fn on_left_click(&mut self, point: &Point) -> UpdateResult;

    fn is_left_click_target(&self, point: &Point) -> bool;
}
