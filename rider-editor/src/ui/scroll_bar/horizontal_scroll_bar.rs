use crate::ui::*;
use rider_config::ConfigAccess;
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

#[cfg_attr(tarpaulin, skip)]
impl HorizontalScrollBar {
    pub fn new(config: ConfigAccess) -> Self {
        Self {
            inner: ScrollBar::new(config),
        }
    }
}

impl ScrollWidget for HorizontalScrollBar {
    fn update_rect(&mut self, pos: i32, max: u32) {
        self.mut_rect().set_width(max);
        self.mut_rect().set_x(pos);
    }

    #[inline]
    fn scroll_to(&mut self, n: i32) {
        self.scroll_value = n;
    }

    #[inline]
    fn scroll_value(&self) -> i32 {
        self.scroll_value
    }

    #[inline]
    fn set_viewport(&mut self, n: u32) {
        self.viewport = n;
    }

    #[inline]
    fn set_full_size(&mut self, n: u32) {
        self.full = n;
    }

    #[inline]
    fn set_location(&mut self, n: i32) {
        self.rect.set_y(n);
    }

    #[inline]
    fn viewport(&self) -> u32 {
        self.viewport
    }

    #[inline]
    fn full(&self) -> u32 {
        self.full
    }

    #[inline]
    fn rect(&self) -> &sdl2::rect::Rect {
        &self.rect
    }

    #[inline]
    fn mut_rect(&mut self) -> &mut sdl2::rect::Rect {
        &mut self.rect
    }
}

#[cfg(test)]
mod test_update {
    use super::*;
    use crate::tests::*;
    use std::sync::*;

    impl HorizontalScrollBar {
        pub fn rect_mut(&mut self) -> &mut sdl2::rect::Rect {
            &mut self.rect
        }
    }

    #[test]
    fn assert_do_nothing_when_small_content() {
        let config = build_config();
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
        let config = build_config();
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
    use crate::tests::*;
    use std::sync::*;

    #[test]
    fn assert_scroll_to() {
        let config = build_config();
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
        let config = build_config();
        let widget = HorizontalScrollBar::new(Arc::clone(&config));
        assert_eq!(widget.scroll_value(), 0);
    }

    #[test]
    fn assert_set_viewport() {
        let config = build_config();
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
        let config = build_config();
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
        let config = build_config();
        let mut widget = HorizontalScrollBar::new(Arc::clone(&config));
        let old = widget.rect().y();
        widget.set_location(157);
        let current = widget.rect().y();
        let expected = 157;
        assert_ne!(old, current);
        assert_eq!(current, expected);
    }
}
