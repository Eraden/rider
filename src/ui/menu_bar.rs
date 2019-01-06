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
    pending: bool,
}

impl MenuBar {
    pub fn new(config: Rc<Config>) -> Self {
        Self {
            background_color: Color::RGB(10, 10, 10),
            dest: Rect::new(0, 0, 0, 0),
            config,
            pending: true,
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
    fn render(
        &self,
        canvas: &mut WindowCanvas,
        _renderer: &mut Renderer,
        parent: Parent,
    ) -> UpdateResult {
        canvas.set_draw_color(self.background_color.clone());
        canvas
            .draw_rect(match parent {
                None => self.dest.clone(),
                Some(parent) => move_render_point(parent.render_start_point(), self.dest()),
            })
            .unwrap_or_else(|_| panic!("Failed to draw main menu background"));
        UpdateResult::NoOp
    }

    fn prepare_ui(&mut self, _renderer: &mut Renderer) {
        if !self.pending {
            return;
        }
        let width = self.config.width();
        let height = self.config.menu_height() as u32;
        self.dest = Rect::new(0, 0, width, height);
        self.pending = false;
    }
}

impl Update for MenuBar {
    fn update(&mut self, _ticks: i32, _context: &UpdateContext) -> UpdateResult {
        UpdateResult::NoOp
    }
}

impl ClickHandler for MenuBar {
    fn on_left_click(&mut self, _point: &Point, _context: &UpdateContext) -> UpdateResult {
        unimplemented!()
    }

    fn is_left_click_target(&self, point: &Point, context: &UpdateContext) -> bool {
        is_in_rect(
            point,
            &match context {
                &UpdateContext::ParentPosition(p) => move_render_point(p, self.dest()),
                _ => self.dest().clone(),
            },
        )
    }
}

impl RenderBox for MenuBar {
    fn render_start_point(&self) -> Point {
        self.dest.top_left()
    }
}
