pub mod managers;

use crate::renderer::managers::{FontManager, TextureManager};

use sdl2::rect::{Point, Rect};
use sdl2::render::{Texture, TextureCreator};
use sdl2::ttf::Sdl2TtfContext;
use sdl2::video::WindowContext;

use std::rc::Rc;

use crate::app::config::Config;
use crate::app::WindowCanvas;
use crate::renderer::managers::TextDetails;

pub struct Renderer<'a> {
    pub config: Config,
    pub font_manager: FontManager<'a>,
    pub texture_manager: TextureManager<'a, WindowContext>,
    pub scroll: Point,
}

impl<'a> Renderer<'a> {
    pub fn new(
        config: Config,
        font_context: &'a Sdl2TtfContext,
        texture_creator: &'a TextureCreator<WindowContext>,
    ) -> Self {
        Self {
            config,
            font_manager: FontManager::new(&font_context),
            texture_manager: TextureManager::new(&texture_creator),
            scroll: (0, 0).into(),
        }
    }

    pub fn render_texture(&mut self, canvas: &mut WindowCanvas, texture: &Rc<Texture>, src: &Rect, dest: &Rect) {
        canvas
            .copy_ex(
                texture,
                Some(src.clone()),
                Some(dest.clone()),
                0.0,
                None,
                false,
                false,
            )
            .unwrap();
    }

    pub fn render_text(&mut self, details: TextDetails) -> Option<Rc<Texture>> {
        let font = self.font_manager.load(&details.font).unwrap();
        let surface = font
            .render(details.text.as_str())
            .blended(details.color);
        let surface = if let Ok(s) = surface {
            s
        } else {
            return None;
        };
        let texture = self
            .texture_manager
            .loader()
            .create_texture_from_surface(&surface);
        let texture = if let Ok(t) = texture {
            Rc::new(t)
        } else {
            return None;
        };
        Some(texture)
    }
}
