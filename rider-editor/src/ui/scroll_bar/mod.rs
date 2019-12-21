use sdl2::rect::{Point, Rect};

pub use crate::ui::scroll_bar::horizontal_scroll_bar::HorizontalScrollBar;
pub use crate::ui::scroll_bar::vertical_scroll_bar::VerticalScrollBar;
use rider_config::{ConfigAccess, ConfigHolder};

pub mod horizontal_scroll_bar;
pub mod vertical_scroll_bar;

pub trait ScrollView<VS, HS>: ConfigHolder
where
    VS: Scroll,
    HS: Scroll,
{
    fn scroll_by(&mut self, x: i32, y: i32) {
        let speed = self.config().read().unwrap().scroll().speed();
        let old_x = self.horizontal_scroll_value();
        let old_y = self.vertical_scroll_value();

        match (self.mut_horizontal_scroll_handler(), speed * x, old_x) {
            (Some(ref mut s), dist, old) if dist + old >= 0 => {
                s.scroll_to(dist + old);
                if s.scrolled_part() > 1.0 {
                    s.scroll_to(old);
                }
            }
            _ => (),
        };
        match (self.mut_vertical_scroll_handler(), speed * y, old_y) {
            (Some(ref mut s), dist, old) if dist + old >= 0 => {
                s.scroll_to(dist + old);
                if s.scrolled_part() > 1.0 {
                    s.scroll_to(old);
                }
            }
            _ => (),
        };
    }

    fn scroll(&self) -> Point {
        Point::new(
            -self.horizontal_scroll_value(),
            -self.vertical_scroll_value(),
        )
    }

    fn horizontal_scroll_value(&self) -> i32 {
        self.horizontal_scroll_handler()
            .map_or(0, |s| s.scroll_value())
    }

    fn vertical_scroll_value(&self) -> i32 {
        self.vertical_scroll_handler()
            .map_or(0, |s| s.scroll_value())
    }

    fn vertical_scrolled_part(&self) -> f64 {
        self.vertical_scroll_handler()
            .map_or(1.0, |s| s.scrolled_part())
    }

    fn horizontal_scrolled_part(&self) -> f64 {
        self.horizontal_scroll_handler()
            .map_or(1.0, |s| s.scrolled_part())
    }

    fn mut_horizontal_scroll_handler(&mut self) -> Option<&mut HS>;
    fn horizontal_scroll_handler(&self) -> Option<&HS>;
    fn mut_vertical_scroll_handler(&mut self) -> Option<&mut VS>;
    fn vertical_scroll_handler(&self) -> Option<&VS>;
}

pub trait Scroll {
    fn scroll_to(&mut self, n: i32);

    fn scroll_value(&self) -> i32;

    fn set_viewport(&mut self, n: u32);

    fn set_full_size(&mut self, n: u32);

    fn set_location(&mut self, n: i32);

    fn scrolled_part(&self) -> f64;
}

pub struct ScrollBar {
    scroll_value: i32,
    viewport: u32,
    full: u32,
    rect: Rect,
}

impl ScrollBar {
    pub fn new(config: ConfigAccess) -> Self {
        let width = { config.read().unwrap().scroll().width() };
        Self {
            scroll_value: 0,
            viewport: 1,
            full: 1,
            rect: Rect::new(0, 0, width, 0),
        }
    }

    #[inline]
    pub fn viewport(&self) -> u32 {
        self.viewport
    }

    #[inline]
    pub fn full(&self) -> u32 {
        self.full
    }

    #[inline]
    pub fn rect(&self) -> &Rect {
        &self.rect
    }
}
