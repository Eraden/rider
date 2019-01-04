use crate::app::{UpdateResult, WindowCanvas};
use crate::config::Config;
use crate::renderer::Renderer;
use crate::ui::*;
use sdl2::pixels::Color;
use sdl2::rect::{Point, Rect};
use std::rc::Rc;

pub struct MenuBar {
    background_color: Color,
    dest: Rect,
    config: Rc<Config>,
}

impl MenuBar {
    pub fn new(config: Rc<Config>) -> Self {
        Self {
            background_color: Color::RGB(10, 10, 10),
            dest: Rect::new(0, 0, 0, 0),
            config,
        }
    }

    pub fn background_color(&self) -> &Color {
        &self.background_color
    }

    pub fn dest(&self) -> &Rect {
        &self.dest
    }
}

impl Render for MenuBar {
    fn render(&mut self, canvas: &mut WindowCanvas, _renderer: &mut Renderer) -> UpdateResult {
        let width = self.config.width();
        let height = self.config.menu_height() as u32;
        self.dest = Rect::new(0, 0, width, height);
        canvas.set_draw_color(self.background_color.clone());
        canvas.draw_rect(self.dest.clone()).unwrap();
        UpdateResult::NoOp
    }
}

impl Update for MenuBar {
    fn update(&mut self, _ticks: i32) -> UpdateResult {
        UpdateResult::NoOp
    }
}

impl ClickHandler for MenuBar {
    fn on_left_click(&mut self, _point: &Point) -> UpdateResult {
        unimplemented!()
    }

    fn is_left_click_target(&self, point: &Point) -> bool {
        is_in_rect(point, self.dest())
    }
}
