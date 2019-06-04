use crate::app::{ConfigAccess, ConfigHolder, UpdateResult as UR};
use crate::renderer::Renderer;
use crate::ui::{
    move_render_point, CanvasAccess, ClickHandler, RenderBox, RenderContext, Update, UpdateContext,
};
use sdl2::rect::{Point, Rect};

const ICON_DEST_WIDTH: u32 = 16;
const ICON_DEST_HEIGHT: u32 = 16;
const ICON_SRC_WIDTH: u32 = 32;
const ICON_SRC_HEIGHT: u32 = 32;

pub struct SaveButton {
    source: Rect,
    dest: Rect,
    config: ConfigAccess,
}

impl SaveButton {
    pub fn new(config: ConfigAccess) -> Self {
        Self {
            dest: Rect::new(0, 0, ICON_DEST_WIDTH, ICON_DEST_HEIGHT),
            source: Rect::new(0, 0, ICON_SRC_WIDTH, ICON_SRC_HEIGHT),
            config,
        }
    }

    pub fn render<C, R>(&self, canvas: &mut C, renderer: &mut R, context: &RenderContext)
    where
        C: CanvasAccess,
        R: Renderer,
    {
        use std::borrow::*;
        let mut dest = match context {
            &RenderContext::ParentPosition(p) => move_render_point(p.clone(), &self.dest),
            _ => self.dest.clone(),
        };

        let mut clipping = dest.clone();
        clipping.set_width(clipping.width() + ICON_DEST_WIDTH);
        clipping.set_height(clipping.height() + ICON_DEST_HEIGHT);
        canvas.set_clipping(clipping);
        let save_texture_path = {
            let c = self.config.read().unwrap();
            let mut themes_dir = c.directories().themes_dir.clone();
            let path = c.theme().images().save_icon();
            themes_dir.push(path);
            themes_dir.to_str().unwrap().to_owned()
        };
        let maybe_tex = renderer.load_image(save_texture_path.clone());
        if let Ok(texture) = maybe_tex {
            dest.set_width(ICON_DEST_WIDTH);
            dest.set_height(ICON_DEST_HEIGHT);
            canvas
                .render_image(texture, self.source.clone(), dest.clone())
                .unwrap_or_else(|_| panic!("Failed to draw directory entry texture"));
        }
    }

    pub fn prepare_ui<'l, T>(&mut self, _renderer: &mut T)
    where
        T: ConfigHolder + Renderer,
    {
    }

    pub fn source(&self) -> &Rect {
        &self.source
    }

    pub fn set_dest(&mut self, rect: &Rect) {
        self.dest = rect.clone();
    }

    pub fn set_source(&mut self, rect: &Rect) {
        self.source = rect.clone();
    }
}

impl Update for SaveButton {
    fn update(&mut self, _ticks: i32, _context: &UpdateContext) -> UR {
        UR::NoOp
    }
}

impl ClickHandler for SaveButton {
    fn on_left_click(&mut self, _point: &Point, _context: &UpdateContext) -> UR {
        UR::SaveCurrentFile
    }

    fn is_left_click_target(&self, point: &Point, context: &UpdateContext) -> bool {
        match *context {
            UpdateContext::ParentPosition(p) => move_render_point(p.clone(), &self.dest),
            _ => self.dest,
        }
        .contains_point(point.clone())
    }
}

impl RenderBox for SaveButton {
    fn render_start_point(&self) -> Point {
        self.dest.top_left()
    }

    fn dest(&self) -> Rect {
        self.dest
    }
}
