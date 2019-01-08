use sdl2::rect::*;
use sdl2::pixels::*;
use crate::ui::*;
use crate::app::{UpdateResult as UR, WindowCanvas as WC};
use crate::renderer::*;
use crate::config::*;

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
}

impl Update for VerticalScrollBar {
    fn update(&mut self, _ticks: i32, _context: &UpdateContext) -> UR {
        if self.full_height < self.viewport {
            return UR::NoOp;
        }
        let ratio = self.full_height as f64 / self.viewport as f64;
        self.rect.set_height((self.viewport as f64 / ratio) as u32);
        let y = (self.viewport - self.rect.height()) as f64 * (self.scroll_value().abs() as f64 / (self.full_height - self.viewport) as f64);
        self.rect.set_y(y as i32);

        UR::NoOp
    }
}

impl Render for VerticalScrollBar {
    fn render(&self, canvas: &mut WC, _renderer: &mut Renderer, context: &RenderContext) {
        if self.full_height < self.viewport {
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
}
