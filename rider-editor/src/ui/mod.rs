use sdl2::rect::{Point, Rect};
use sdl2::render::Texture;

use std::rc::Rc;

use crate::app::application::WindowCanvas;
use crate::app::{UpdateResult as UR};
use crate::renderer::managers::*;
use crate::renderer::CanvasRenderer;
use rider_config::*;

pub mod caret;
pub mod file;
pub mod file_editor;
pub mod filesystem;
pub mod menu_bar;
pub mod modal;
pub mod project_tree;
pub mod scroll_bar;
pub mod text_character;

pub use crate::ui::caret::*;
pub use crate::ui::file::*;
pub use crate::ui::file_editor::*;
pub use crate::ui::filesystem::*;
pub use crate::ui::menu_bar::*;
pub use crate::ui::modal::*;
pub use crate::ui::project_tree::*;
pub use crate::ui::scroll_bar::*;
pub use crate::ui::text_character::*;

#[derive(Debug)]
pub enum UpdateContext<'l> {
    Nothing,
    ParentPosition(Point),
    CurrentFile(&'l mut EditorFile),
}

#[derive(Clone, PartialEq, Debug)]
pub enum RenderContext {
    Nothing,
    RelativePosition(Point),
}

pub trait CanvasAccess {
    fn render_rect(&mut self, rect: Rect, color: sdl2::pixels::Color) -> Result<(), String>;
    fn render_border(&mut self, rect: Rect, color: sdl2::pixels::Color) -> Result<(), String>;
    fn render_image(&mut self, tex: Rc<Texture>, src: Rect, dest: Rect) -> Result<(), String>;
    fn render_line(
        &mut self,
        start: Point,
        end: Point,
        color: sdl2::pixels::Color,
    ) -> Result<(), String>;

    fn set_clipping(&mut self, rect: Rect);
}

impl CanvasAccess for WindowCanvas {
    fn render_rect(&mut self, rect: Rect, color: sdl2::pixels::Color) -> Result<(), String> {
        self.set_draw_color(color);
        self.fill_rect(rect)
    }

    fn render_border(&mut self, rect: Rect, color: sdl2::pixels::Color) -> Result<(), String> {
        self.set_draw_color(color);
        self.draw_rect(rect)
    }

    fn render_image(&mut self, tex: Rc<Texture>, src: Rect, dest: Rect) -> Result<(), String> {
        self.copy_ex(&tex, Some(src), Some(dest), 0.0, None, false, false)
    }

    fn render_line(
        &mut self,
        start: Point,
        end: Point,
        color: sdl2::pixels::Color,
    ) -> Result<(), String> {
        self.set_draw_color(color);
        self.draw_line(start, end)
    }

    fn set_clipping(&mut self, rect: Rect) {
        self.set_clip_rect(rect);
    }
}

#[inline]
pub fn is_in_rect(point: &Point, rect: &Rect) -> bool {
    rect.contains_point(point.clone())
}

#[inline]
pub fn build_font_details<T>(config_holder: &T) -> FontDetails
where
    T: ConfigHolder,
{
    let c = config_holder.config().read().unwrap();
    FontDetails::new(
        c.editor_config().font_path().as_str(),
        c.editor_config().character_size(),
    )
}

pub fn get_text_character_rect<'l, T>(c: char, renderer: &mut T) -> Option<Rect>
where
    T: ManagersHolder<'l> + ConfigHolder,
{
    let font_details = renderer.config().read().unwrap().editor_config().into();
    renderer
        .font_manager()
        .load(&font_details)
        .ok()
        .and_then(|font| font.size_of_char(c).ok())
        .and_then(|(width, height)| Some(Rect::new(0, 0, width, height)))
}

#[inline]
pub fn move_render_point(p: Point, d: &Rect) -> Rect {
    Rect::new(d.x() + p.x(), d.y() + p.y(), d.width(), d.height())
}

pub trait Update {
    fn update(&mut self, ticks: i32, context: &UpdateContext) -> UR;
}

pub trait ClickHandler {
    fn on_left_click(&mut self, point: &Point, context: &UpdateContext) -> UR;

    fn is_left_click_target(&self, point: &Point, context: &UpdateContext) -> bool;
}

pub trait RenderBox {
    fn render_start_point(&self) -> Point;

    fn dest(&self) -> Rect;
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::support;

    struct ConfigWrapper {
        pub inner: ConfigAccess,
    }

    impl ConfigHolder for ConfigWrapper {
        fn config(&self) -> &ConfigAccess {
            &self.inner
        }
    }

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

    #[test]
    fn must_build_font_details() {
        let config = support::build_config();
        let wrapper = ConfigWrapper {
            inner: config.clone(),
        };
        let details = build_font_details(&wrapper);
        let c = config.read().unwrap();
        assert_eq!(details.path, c.editor_config().font_path().to_string());
        assert_eq!(details.size, c.editor_config().character_size());
    }
}
