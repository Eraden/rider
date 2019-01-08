use crate::renderer::managers::*;
use crate::ui::get_text_character_rect;
use rider_config::{ConfigAccess, ConfigHolder};
use sdl2::rect::Rect;
use sdl2::render::TextureCreator;
use sdl2::ttf::Sdl2TtfContext;
use sdl2::video::WindowContext as WinCtxt;
use std::collections::HashMap;

pub struct Renderer<'l> {
    config: ConfigAccess,
    font_manager: FontManager<'l>,
    texture_manager: TextureManager<'l, WinCtxt>,
    character_sizes: HashMap<TextCharacterDetails, Rect>,
}

impl<'l> Renderer<'l> {
    pub fn new(
        config: ConfigAccess,
        font_context: &'l Sdl2TtfContext,
        texture_creator: &'l TextureCreator<WinCtxt>,
    ) -> Self {
        let texture_manager = TextureManager::new(&texture_creator);
        let font_manager = FontManager::new(&font_context);
        Self {
            config,
            font_manager,
            texture_manager,
            character_sizes: HashMap::new(),
        }
    }

    pub fn character_sizes_mut(&mut self) -> &mut HashMap<TextCharacterDetails, Rect> {
        &mut self.character_sizes
    }

    pub fn load_character_size(&mut self, c: char) -> Rect {
        let (font_path, font_size) = {
            let config = self.config().read().unwrap();
            (
                config.editor_config().font_path().clone(),
                config.editor_config().character_size().clone(),
            )
        };
        let details = TextCharacterDetails {
            c: c.clone(),
            font_path,
            font_size,
        };
        self.character_sizes
            .get(&details)
            .cloned()
            .or_else(|| {
                let size = get_text_character_rect(c, self).unwrap();
                self.character_sizes.insert(details.clone(), size.clone());
                Some(size)
            })
            .unwrap()
            .clone()
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
