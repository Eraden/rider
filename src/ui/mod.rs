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
    rect.contains_point(point.clone())
}

pub fn get_text_character_rect(c: char, renderer: &mut Renderer) -> Option<Rect> {
    let font_details = FontDetails::new(
        renderer
            .config()
            .read()
            .unwrap()
            .editor_config()
            .font_path()
            .as_str(),
        renderer
            .config()
            .read()
            .unwrap()
            .editor_config()
            .character_size()
            .clone(),
    );
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

#[cfg(test)]
mod tests {
    use super::*;
    use sdl2::rect::*;

    #[test]
    fn must_return_true_if_inside_rect() {
        let rect = Rect::new(10, 10, 30, 30);
        let point = Point::new(20, 20);
        assert_eq!(is_in_rect(&point, &rect), true);
    }

    #[test]
    fn must_return_not_if_not_inside_rect() {
        let rect = Rect::new(10, 10, 30, 30);
        let point = Point::new(41, 41);
        assert_eq!(is_in_rect(&point, &rect), false);
    }

    #[test]
    fn must_return_moved_rect() {
        let rect = Rect::new(10, 20, 30, 40);
        let point = Point::new(11, 11);
        assert_eq!(move_render_point(point, &rect), Rect::new(21, 31, 30, 40));
    }
}
