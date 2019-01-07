use crate::app::WindowCanvas as WC;
use crate::config::{Config, ConfigAccess, ConfigHolder};
use crate::renderer::managers::*;
use sdl2::rect::{Point, Rect};
use sdl2::render::{Texture, TextureCreator};
use sdl2::ttf::Sdl2TtfContext;
use sdl2::video::WindowContext as WinCtxt;
use std::rc::Rc;
use std::sync::*;

pub struct Renderer<'l> {
    config: ConfigAccess,
    font_manager: FontManager<'l>,
    texture_manager: TextureManager<'l, WinCtxt>,
}

impl<'l> Renderer<'l> {
    pub fn new(
        config: ConfigAccess,
        font_context: &'l Sdl2TtfContext,
        texture_creator: &'l TextureCreator<WinCtxt>,
    ) -> Self {
        Self {
            config,
            font_manager: FontManager::new(&font_context),
            texture_manager: TextureManager::new(&texture_creator),
        }
    }
}

impl<'l> ManagersHolder<'l> for Renderer<'l> {
    fn font_manager(&mut self) -> &mut FontManager<'l> {
        &mut self.font_manager
    }

    fn texture_manager(&mut self) -> &mut TextureManager<'l, WinCtxt> {
        &mut self.texture_manager
    }
}

impl<'l> ConfigHolder for Renderer<'l> {
    fn config(&self) -> &ConfigAccess {
        &self.config
    }
}
