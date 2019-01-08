use crate::app::{UpdateResult as UR, WindowCanvas as WC};
use crate::config::*;
use crate::renderer::*;
use crate::ui::*;
use sdl2::pixels::Color;
use sdl2::rect::{Point, Rect};
use std::rc::Rc;
use std::sync::*;

pub struct MenuBar {
    border_color: Color,
    background_color: Color,
    dest: Rect,
    config: ConfigAccess,
}

impl MenuBar {
    pub fn new(config: ConfigAccess) -> Self {
        let (background_color, w, h): (Color, u32, u16) = {
            let c = config.read().unwrap();
            (c.theme().background().into(), c.width(), c.menu_height())
        };
        Self {
            border_color: Color::RGB(10, 10, 10),
            background_color,
            dest: Rect::new(0, 0, w as u32, h as u32),
            config,
        }
    }

    pub fn background_color(&self) -> &Color {
        &self.background_color
    }
}

impl Render for MenuBar {
    fn render(&self, canvas: &mut WC, _renderer: &mut Renderer, context: &RenderContext) {
        use std::borrow::*;

        canvas.set_clip_rect(self.dest.clone());
        canvas.set_draw_color(self.background_color.clone());
        canvas
            .fill_rect(match context.borrow() {
                RenderContext::RelativePosition(p) => move_render_point(p.clone(), self.dest()),
                _ => self.dest.clone(),
            })
            .unwrap_or_else(|_| panic!("Failed to draw main menu background"));

        canvas.set_draw_color(self.border_color.clone());
        canvas
            .draw_rect(match context.borrow() {
                RenderContext::RelativePosition(p) => move_render_point(p.clone(), self.dest()),
                _ => self.dest.clone(),
            })
            .unwrap_or_else(|_| panic!("Failed to draw main menu background"));
    }

    fn prepare_ui(&mut self, _renderer: &mut Renderer) {
        let width = self.config.read().unwrap().width();
        let height = self.config.read().unwrap().menu_height() as u32;
        self.dest = Rect::new(0, 0, width, height);
    }
}

impl Update for MenuBar {
    fn update(&mut self, _ticks: i32, _context: &UpdateContext) -> UR {
        let config = self.config.read().unwrap();
        self.dest.set_width(config.width());
        UR::NoOp
    }
}

impl ClickHandler for MenuBar {
    fn on_left_click(&mut self, _point: &Point, _context: &UpdateContext) -> UR {
        UR::NoOp
    }

    fn is_left_click_target(&self, point: &Point, context: &UpdateContext) -> bool {
        let rect = match context.clone() {
            UpdateContext::ParentPosition(p) => move_render_point(p.clone(), self.dest()),
            _ => self.dest().clone(),
        };
        is_in_rect(point, &rect)
    }
}

impl RenderBox for MenuBar {
    fn render_start_point(&self) -> Point {
        self.dest.top_left()
    }

    fn dest(&self) -> &Rect {
        &self.dest
    }
}

#[cfg(test)]
mod test_getters {
    use crate::app::*;
    use crate::renderer::*;
    use crate::tests::*;
    use crate::ui::*;
    use sdl2::pixels::*;
    use sdl2::rect::*;
    use std::sync::*;

    #[test]
    fn assert_background_color() {
        let config = support::build_config();
        let widget = MenuBar::new(Arc::clone(&config));
        let result = widget.background_color().clone();
        let expected = Color::RGBA(18, 18, 18, 0);
        assert_eq!(result, expected);
    }

    #[test]
    fn assert_dest() {
        let config = support::build_config();
        let (w, h) = {
            let c = config.read().unwrap();
            (c.width() as u32, c.menu_height() as u32)
        };
        let widget = MenuBar::new(Arc::clone(&config));
        let result = widget.dest().clone();
        let expected = Rect::new(0, 0, w, h);
        assert_eq!(result, expected);
    }
}

#[cfg(test)]
mod test_render_box {
    use crate::renderer::*;
    use crate::tests::*;
    use crate::ui::*;
    use sdl2::pixels::*;
    use sdl2::rect::*;
    use std::sync::*;

    #[test]
    fn must_return_top_left_point() {
        let config = support::build_config();
        let widget = MenuBar::new(Arc::clone(&config));
        let result = widget.render_start_point();
        let expected = Point::new(0, 0);
        assert_eq!(result, expected);
    }
}

#[cfg(test)]
mod test_click_handler {
    use crate::app::*;
    use crate::renderer::*;
    use crate::tests::*;
    use crate::ui::*;
    use sdl2::pixels::*;
    use sdl2::rect::*;
    use std::sync::*;

    #[test]
    fn refute_when_not_click_target() {
        let config = support::build_config();
        let widget = MenuBar::new(Arc::clone(&config));
        let point = Point::new(9999, 9999);
        let context = UpdateContext::Nothing;
        let result = widget.is_left_click_target(&point, &context);
        assert_eq!(result, false);
    }

    #[test]
    fn assert_when_click_target() {
        let config = support::build_config();
        let widget = MenuBar::new(Arc::clone(&config));
        let point = Point::new(20, 30);
        let context = UpdateContext::Nothing;
        let result = widget.is_left_click_target(&point, &context);
        assert_eq!(result, true);
    }

    #[test]
    fn refute_when_not_click_target_because_parent() {
        let config = support::build_config();
        let widget = MenuBar::new(Arc::clone(&config));
        let point = Point::new(20, 30);
        let context = UpdateContext::ParentPosition(Point::new(9999, 9999));
        let result = widget.is_left_click_target(&point, &context);
        assert_eq!(result, false);
    }

    #[test]
    fn assert_when_click_target_because_parent() {
        let config = support::build_config();
        let (w, h) = {
            (
                config.read().unwrap().width(),
                config.read().unwrap().menu_height(),
            )
        };
        let widget = MenuBar::new(Arc::clone(&config));
        let point = Point::new(w as i32 + 120, h as i32 + 130);
        let context = UpdateContext::ParentPosition(Point::new(130, 140));
        let result = widget.is_left_click_target(&point, &context);
        assert_eq!(result, true);
    }

    #[test]
    fn assert_on_click_do_nothing() {
        let config = support::build_config();
        let mut widget = MenuBar::new(Arc::clone(&config));
        let point = Point::new(12, 34);
        let context = UpdateContext::ParentPosition(Point::new(678, 293));
        let result = widget.on_left_click(&point, &context);
        let expected = UpdateResult::NoOp;
        assert_eq!(result, expected);
    }
}
