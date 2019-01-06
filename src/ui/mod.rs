use sdl2::rect::{Point, Rect};

use crate::app::{UpdateResult, WindowCanvas};
use crate::config::Config;
use crate::renderer::managers::FontDetails;
use crate::renderer::Renderer;

pub mod caret;
pub mod file;
pub mod file_editor;
pub mod menu_bar;
pub mod project_tree;
pub mod text_character;

pub use crate::ui::caret::*;
pub use crate::ui::file::*;
pub use crate::ui::file_editor::*;
pub use crate::ui::menu_bar::*;
pub use crate::ui::project_tree::*;
pub use crate::ui::text_character::*;

pub type Parent<'l> = Option<&'l RenderBox>;
pub type ParentMut<'l> = Option<&'l mut RenderBox>;

pub enum UpdateContext<'l> {
    Nothing,
    ParentPosition(Point),
    CurrentFile(&'l mut EditorFile),
}

#[inline]
pub fn is_in_rect(point: &Point, rect: &Rect) -> bool {
    let start = rect.top_left();
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

#[inline]
pub fn move_render_point(p: Point, d: &Rect) -> Rect {
    Rect::new(d.x() + p.x(), d.y() + p.y(), d.width(), d.height())
}

pub trait Render {
    fn render(
        &self,
        canvas: &mut WindowCanvas,
        renderer: &mut Renderer,
        parent: Parent,
    ) -> UpdateResult;

    fn prepare_ui(&mut self, renderer: &mut Renderer);
}

pub trait Update {
    fn update(&mut self, ticks: i32, context: &UpdateContext) -> UpdateResult;
}

pub trait ClickHandler {
    fn on_left_click(&mut self, point: &Point, context: &UpdateContext) -> UpdateResult;

    fn is_left_click_target(&self, point: &Point, context: &UpdateContext) -> bool;
}

pub trait RenderBox {
    fn render_start_point(&self) -> Point;
}
