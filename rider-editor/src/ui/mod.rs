use sdl2::rect::{Point, Rect};
use sdl2::render::Texture;

use std::rc::Rc;

use crate::app::application::WindowCanvas;
use crate::app::{UpdateResult as UR, UpdateResult};
use crate::renderer::managers::*;
use rider_config::*;

pub mod buttons;
pub mod caret;
pub mod file;
pub mod file_editor;
pub mod filesystem;
pub mod label;
pub mod menu_bar;
pub mod modal;
pub mod project_tree;
pub mod scroll_bar;
pub mod text_character;

pub use self::buttons::*;
pub use self::caret::*;
pub use self::file::*;
pub use self::file_editor::*;
pub use self::filesystem::*;
pub use self::label::*;
pub use self::menu_bar::*;
pub use self::modal::*;
pub use self::project_tree::*;
pub use self::scroll_bar::*;
pub use self::text_character::*;
use crate::renderer::Renderer;

#[derive(Debug)]
pub enum UpdateContext<'l> {
    Nothing,
    ParentPosition(Point),
    CurrentFile(&'l mut EditorFile),
}

#[derive(Clone, PartialEq, Debug)]
pub enum RenderContext {
    Nothing,
    ParentPosition(Point),
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

pub struct WidgetInner {
    source: Rect,
    dest: Rect,
    config: ConfigAccess,
}

impl WidgetInner {
    pub fn new(config: ConfigAccess, source: Rect, dest: Rect) -> Self {
        Self {
            dest,
            source,
            config,
        }
    }
}

pub trait Widget {
    fn texture_path(&self) -> Option<String>;

    fn render<C, R>(&self, canvas: &mut C, renderer: &mut R, context: &RenderContext)
    where
        C: CanvasAccess,
        R: Renderer,
    {
        let mut dest = match context {
            &RenderContext::ParentPosition(p) => move_render_point(p.clone(), &self.dest()),
            _ => self.dest().clone(),
        };

        canvas.set_clipping(self.clipping(&dest));
        self.texture_path()
            .and_then(|path| renderer.load_image(path).ok())
            .and_then(|texture| {
                dest.set_width(self.dest().width());
                dest.set_height(self.dest().height());
                canvas
                    .render_image(texture.clone(), self.source().clone(), dest.clone())
                    .unwrap_or_else(|_| panic!("Failed to draw widget texture"));
                Some(())
            });
    }

    fn prepare_ui<'l, T>(&mut self, _renderer: &mut T)
    where
        T: ConfigHolder + Renderer,
    {
    }

    fn dest(&self) -> &Rect;

    fn set_dest(&mut self, rect: &Rect);

    fn source(&self) -> &Rect;

    fn set_source(&mut self, rect: &Rect);

    fn update(&mut self, _ticks: i32, _context: &UpdateContext) -> UpdateResult {
        UpdateResult::NoOp
    }

    fn on_left_click(&mut self, _point: &Point, _context: &UpdateContext) -> UpdateResult {
        UpdateResult::NoOp
    }

    fn is_left_click_target(&self, point: &Point, context: &UpdateContext) -> bool {
        match *context {
            UpdateContext::ParentPosition(p) => move_render_point(p.clone(), &self.dest()),
            _ => self.dest().clone(),
        }
        .contains_point(point.clone())
    }

    fn render_start_point(&self) -> Point {
        self.dest().top_left()
    }

    fn clipping(&self, relative_dest: &Rect) -> Rect {
        Rect::new(
            relative_dest.x(),
            relative_dest.y(),
            relative_dest.width() + self.padding_width(),
            relative_dest.height() + self.padding_height(),
        )
    }

    fn padding_width(&self) -> u32 {
        0
    }

    fn padding_height(&self) -> u32 {
        0
    }
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
