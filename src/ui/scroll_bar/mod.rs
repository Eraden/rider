pub mod vertical_scroll_bar;
pub mod horizontal_scroll_bar;

use crate::ui::scroll_bar::vertical_scroll_bar::*;
use crate::ui::scroll_bar::horizontal_scroll_bar::*;

pub trait Scrollable {
    fn scroll_to(&mut self, n: i32);

    fn scroll_value(&self) -> i32;

    fn set_viewport(&mut self, n: u32);

    fn set_full_size(&mut self, n: u32);

    fn set_location(&mut self, n: i32);
}
