use crate::app::UpdateResult as UR;
use crate::ui::*;
use rider_config::ConfigAccess;
use sdl2::pixels::Color;
use std::ops::{Deref, DerefMut};

pub struct HorizontalScrollBar {
    inner: ScrollBar,
}

impl Deref for HorizontalScrollBar {
    type Target = ScrollBar;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl DerefMut for HorizontalScrollBar {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl HorizontalScrollBar {
    pub fn new(config: ConfigAccess) -> Self {
        Self {
            inner: ScrollBar::new(config),
        }
    }
}

impl Update for HorizontalScrollBar {
    fn update(&mut self, _ticks: i32, _context: &UpdateContext) -> UR {
        if self.full < self.viewport {
            return UR::NoOp;
        }
        let ratio = self.full as f64 / self.viewport as f64;
        let width = (self.viewport as f64 / ratio) as u32;
        self.rect.set_width(width);
        let x = (self.viewport - self.rect.width()) as f64
            * (self.scroll_value().abs() as f64 / (self.full - self.viewport) as f64);
        self.rect.set_x(x as i32);

        UR::NoOp
    }
}

#[cfg_attr(tarpaulin, skip)]
impl HorizontalScrollBar {
    pub fn render<T>(&self, canvas: &mut T, context: &RenderContext)
    where
        T: CanvasAccess,
    {
        if self.full < self.viewport {
            return;
        }

        canvas
            .render_rect(
                match context {
                    RenderContext::ParentPosition(p) => move_render_point(p.clone(), &self.rect),
                    _ => self.rect.clone(),
                },
                Color::RGBA(255, 255, 255, 0),
            )
            .unwrap_or_else(|_| panic!("Failed to render vertical scroll back"));
    }
}

impl Scroll for HorizontalScrollBar {
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
        self.full = n;
    }

    fn set_location(&mut self, n: i32) {
        self.rect.set_y(n);
    }

    fn scrolled_part(&self) -> f64 {
        if self.full < self.viewport() {
            return 1.0;
        }
        self.scroll_value().abs() as f64 / (self.full - self.viewport()) as f64
    }
}

#[cfg(test)]
mod test_update {
    use super::*;
    use crate::tests::support;
    use std::sync::*;

    impl HorizontalScrollBar {
        pub fn rect_mut(&mut self) -> &mut Rect {
            &mut self.rect
        }
    }

    #[test]
    fn assert_do_nothing_when_small_content() {
        let config = support::build_config();
        let mut widget = HorizontalScrollBar::new(Arc::clone(&config));
        widget.set_viewport(100);
        widget.set_full_size(20);
        widget.rect_mut().set_x(30000000);
        widget.rect_mut().set_width(30000000);
        widget.update(0, &UpdateContext::Nothing);
        assert_eq!(widget.rect().x(), 30000000);
        assert_eq!(widget.rect().width(), 30000000);
    }

    #[test]
    fn assert_update_when_huge_content() {
        let config = support::build_config();
        let mut widget = HorizontalScrollBar::new(Arc::clone(&config));
        widget.set_viewport(100);
        widget.set_full_size(200);
        widget.rect_mut().set_x(30000000);
        widget.rect_mut().set_width(30000000);
        widget.update(0, &UpdateContext::Nothing);
        assert_eq!(widget.rect().x(), 0);
        assert_eq!(widget.rect().width(), 50);
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
        let mut widget = HorizontalScrollBar::new(Arc::clone(&config));
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
        let widget = HorizontalScrollBar::new(Arc::clone(&config));
        assert_eq!(widget.scroll_value(), 0);
    }

    #[test]
    fn assert_set_viewport() {
        let config = support::build_config();
        let mut widget = HorizontalScrollBar::new(Arc::clone(&config));
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
        let mut widget = HorizontalScrollBar::new(Arc::clone(&config));
        let old = widget.full;
        widget.set_full_size(157);
        let current = widget.full;
        let expected = 157;
        assert_ne!(old, current);
        assert_eq!(current, expected);
    }

    #[test]
    fn assert_set_location() {
        let config = support::build_config();
        let mut widget = HorizontalScrollBar::new(Arc::clone(&config));
        let old = widget.rect().y();
        widget.set_location(157);
        let current = widget.rect().y();
        let expected = 157;
        assert_ne!(old, current);
        assert_eq!(current, expected);
    }
}
