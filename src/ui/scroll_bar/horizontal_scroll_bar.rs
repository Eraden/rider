use sdl2::rect::*;
use sdl2::pixels::*;
use crate::ui::*;
use crate::app::{UpdateResult as UR, WindowCanvas as WC};
use crate::renderer::*;
use crate::config::*;

pub struct HorizontalScrollBar {
    scroll_value: i32,
    viewport: u32,
    full_width: u32,
    rect: Rect,
}

impl HorizontalScrollBar {
    pub fn new(config: ConfigAccess) -> Self {
        let width = { config.read().unwrap().scroll().width() };
        Self {
            scroll_value: 0,
            viewport: 1,
            full_width: 1,
            rect: Rect::new(0, 0, width, 0),
        }
    }
}

impl Update for HorizontalScrollBar {
    fn update(&mut self, _ticks: i32, _context: &UpdateContext) -> UR {
        if self.full_width < self.viewport {
            return UR::NoOp;
        }
        let ratio = self.full_width as f64 / self.viewport as f64;
        self.rect.set_width((self.viewport as f64 / ratio) as u32);
        let x = (self.viewport - self.rect.width()) as f64 * (self.scroll_value().abs() as f64 / (self.full_width - self.viewport) as f64);
        self.rect.set_x(x as i32);

        UR::NoOp
    }
}

impl Render for HorizontalScrollBar {
    fn render(&self, canvas: &mut WC, _renderer: &mut Renderer, context: &RenderContext) {
        if self.full_width < self.viewport {
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

impl Scrollable for HorizontalScrollBar {
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
        self.full_width = n;
    }

    fn set_location(&mut self, n: i32) {
        self.rect.set_y(n);
    }
}
