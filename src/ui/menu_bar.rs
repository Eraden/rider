use crate::app::{UpdateResult as UR, WindowCanvas as WC};
use crate::config::Config;
use crate::renderer::Renderer;
use crate::ui::*;
use sdl2::pixels::Color;
use sdl2::rect::{Point, Rect};
use std::rc::Rc;
use std::sync::*;

pub struct MenuBar {
    border_color: Color,
    background_color: Color,
    dest: Rect,
    config: Arc<RwLock<Config>>,
    pending: bool,
}

impl MenuBar {
    pub fn new(config: Arc<RwLock<Config>>) -> Self {
        let background_color = { config.read().unwrap().theme().background().into() };
        Self {
            border_color: Color::RGB(10, 10, 10),
            background_color,
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
    fn render(&self, canvas: &mut WC, _renderer: &mut Renderer, parent: Parent) -> UR {
        canvas.set_clip_rect(self.dest.clone());
        canvas.set_draw_color(self.background_color.clone());
        canvas
            .fill_rect(match parent {
                None => self.dest.clone(),
                Some(parent) => move_render_point(parent.render_start_point(), self.dest()),
            })
            .unwrap_or_else(|_| panic!("Failed to draw main menu background"));

        canvas.set_draw_color(self.border_color.clone());
        canvas
            .draw_rect(match parent {
                None => self.dest.clone(),
                Some(parent) => move_render_point(parent.render_start_point(), self.dest()),
            })
            .unwrap_or_else(|_| panic!("Failed to draw main menu background"));

        UR::NoOp
    }

    fn prepare_ui(&mut self, _renderer: &mut Renderer) {
        if !self.pending {
            return;
        }
        let width = self.config.read().unwrap().width();
        let height = self.config.read().unwrap().menu_height() as u32;
        self.dest = Rect::new(0, 0, width, height);
        self.pending = false;
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
