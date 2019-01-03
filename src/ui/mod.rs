use crate::app::{UpdateResult, WindowCanvas};
use crate::config::Config;
use crate::renderer::Renderer;
use sdl2::rect::{Point, Rect};

pub mod caret;
pub mod menu_bar;
pub mod text_character;

pub fn is_in_rect(point: &Point, rect: &Rect) -> bool {
    let start = Point::new(rect.x(), rect.y());
    let end = Point::new(
        rect.x() + (rect.width() as i32),
        rect.y() + (rect.height() as i32),
    );
    start.x() <= point.x() && start.y() <= point.y() && end.x() >= point.x() && end.y() >= point.y()
}

pub trait Render {
    fn render(&mut self, canvas: &mut WindowCanvas, renderer: &mut Renderer) -> UpdateResult;
}

pub trait Update {
    fn update(&mut self, ticks: i32) -> UpdateResult;
}

pub trait ClickHandler {
    fn on_left_click(&mut self, point: &Point, config: &Config) -> UpdateResult;

    fn is_left_click_target(&self, point: &Point) -> bool;
}
