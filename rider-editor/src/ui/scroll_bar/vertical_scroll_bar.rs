use crate::app::{UpdateResult as UR, WindowCanvas as WC};
use crate::renderer::*;
use crate::ui::*;
use rider_config::ConfigAccess;
use sdl2::pixels::Color;
use sdl2::rect::Rect;

pub struct VerticalScrollBar {
    scroll_value: i32,
    viewport: u32,
    full_height: u32,
    rect: Rect,
}

impl VerticalScrollBar {
    pub fn new(config: ConfigAccess) -> Self {
        let width = { config.read().unwrap().scroll().width() };
        Self {
            scroll_value: 0,
            viewport: 1,
            full_height: 1,
            rect: Rect::new(0, 0, width, 0),
        }
    }

    #[inline]
    pub fn viewport(&self) -> u32 {
        self.viewport
    }

    #[inline]
    pub fn full_height(&self) -> u32 {
        self.full_height
    }

    #[inline]
    pub fn rect(&self) -> &Rect {
        &self.rect
    }
}

impl Update for VerticalScrollBar {
    fn update(&mut self, _ticks: i32, _context: &UpdateContext) -> UR {
        if self.full_height() < self.viewport() {
            return UR::NoOp;
        }
        let ratio = self.full_height() as f64 / self.viewport() as f64;
        self.rect
            .set_height((self.viewport() as f64 / ratio) as u32);
        let y = (self.viewport() - self.rect.height()) as f64 * self.scrolled_part();
        self.rect.set_y(y as i32);

        UR::NoOp
    }
}

#[cfg_attr(tarpaulin, skip)]
impl Render for VerticalScrollBar {
    fn render(&self, canvas: &mut WC, _renderer: &mut Renderer, context: &RenderContext) {
        if self.full_height() < self.viewport() {
            return;
        }

        canvas.set_draw_color(Color::RGBA(255, 255, 255, 0));
        canvas
            .fill_rect(match context {
                RenderContext::RelativePosition(p) => move_render_point(p.clone(), &self.rect),
                _ => self.rect.clone(),
            })
            .unwrap_or_else(|_| panic!("Failed to render vertical scroll back"));
    }

    fn prepare_ui(&mut self, _renderer: &mut Renderer) {}
}

impl Scrollable for VerticalScrollBar {
    fn scroll_to(&mut self, n: i32) {
        self.scroll_value = n;
    }

    fn scroll_value(&self) -> i32 {
        self.scroll_value
    }

    fn set_viewport(&mut self, n: u32) {
        self.viewport = n;
    }

    fn set_full_size(&mut self, n: u32) {
        self.full_height = n;
    }

    fn set_location(&mut self, n: i32) {
        self.rect.set_x(n);
    }

    fn scrolled_part(&self) -> f64 {
        if self.full_height() <= self.viewport() {
            return 1.0;
        }
        self.scroll_value().abs() as f64 / (self.full_height() - self.viewport()) as f64
    }
}

#[cfg(test)]
mod test_update {
    use super::*;
    use crate::tests::support;
    use std::sync::*;

    impl VerticalScrollBar {
        pub fn rect_mut(&mut self) -> &mut Rect {
            &mut self.rect
        }
    }

    #[test]
    fn assert_do_nothing_when_small_content() {
        let config = support::build_config();
        let mut widget = VerticalScrollBar::new(Arc::clone(&config));
        widget.set_viewport(100);
        widget.set_full_size(20);
        widget.rect_mut().set_y(30000000);
        widget.rect_mut().set_height(30000000);
        widget.update(0, &UpdateContext::Nothing);
        assert_eq!(widget.rect().y(), 30000000);
        assert_eq!(widget.rect().height(), 30000000);
    }

    #[test]
    fn assert_update_when_huge_content() {
        let config = support::build_config();
        let mut widget = VerticalScrollBar::new(Arc::clone(&config));
        widget.set_viewport(100);
        widget.set_full_size(200);
        widget.rect_mut().set_y(30000000);
        widget.rect_mut().set_height(30000000);
        widget.update(0, &UpdateContext::Nothing);
        assert_eq!(widget.rect().y(), 0);
        assert_eq!(widget.rect().height(), 50);
    }
}

#[cfg(test)]
mod test_scrollable {
    use super::*;
    use crate::tests::support;
    use std::sync::*;

    #[test]
    fn assert_scroll_to() {
        let config = support::build_config();
        let mut widget = VerticalScrollBar::new(Arc::clone(&config));
        let old = widget.scroll_value();
        widget.scroll_to(157);
        let current = widget.scroll_value();
        let expected = 157;
        assert_ne!(old, current);
        assert_eq!(current, expected);
    }

    #[test]
    fn assert_scroll_value() {
        let config = support::build_config();
        let widget = VerticalScrollBar::new(Arc::clone(&config));
        assert_eq!(widget.scroll_value(), 0);
    }

    #[test]
    fn assert_set_viewport() {
        let config = support::build_config();
        let mut widget = VerticalScrollBar::new(Arc::clone(&config));
        let old = widget.viewport();
        widget.set_viewport(157);
        let current = widget.viewport();
        let expected = 157;
        assert_ne!(old, current);
        assert_eq!(current, expected);
    }

    #[test]
    fn assert_set_full_size() {
        let config = support::build_config();
        let mut widget = VerticalScrollBar::new(Arc::clone(&config));
        let old = widget.full_height();
        widget.set_full_size(157);
        let current = widget.full_height();
        let expected = 157;
        assert_ne!(old, current);
        assert_eq!(current, expected);
    }

    #[test]
    fn assert_set_location() {
        let config = support::build_config();
        let mut widget = VerticalScrollBar::new(Arc::clone(&config));
        let old = widget.rect().x();
        widget.set_location(157);
        let current = widget.rect().x();
        let expected = 157;
        assert_ne!(old, current);
        assert_eq!(current, expected);
    }
}