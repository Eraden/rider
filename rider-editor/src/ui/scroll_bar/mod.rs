pub use crate::ui::scroll_bar::horizontal_scroll_bar::HorizontalScrollBar;
pub use crate::ui::scroll_bar::vertical_scroll_bar::VerticalScrollBar;

pub mod horizontal_scroll_bar;
pub mod vertical_scroll_bar;

pub trait Scrollable {
    fn scroll_to(&mut self, n: i32);

    fn scroll_value(&self) -> i32;

    fn set_viewport(&mut self, n: u32);

    fn set_full_size(&mut self, n: u32);

    fn set_location(&mut self, n: i32);

    fn scrolled_part(&self) -> f64;
}
