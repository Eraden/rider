use crate::renderer::managers::*;
use crate::ui::get_text_character_rect;
use crate::ui::text_character::CharacterSizeManager;
use rider_config::{ConfigAccess, ConfigHolder};
use sdl2::rect::Rect;
use sdl2::render::Texture;
use sdl2::render::TextureCreator;
use sdl2::ttf::Font;
use sdl2::ttf::Sdl2TtfContext;
use std::collections::HashMap;
use std::rc::Rc;

pub trait Renderer {
    fn load_font(&mut self, details: FontDetails) -> Rc<Font>;

    fn load_text_tex(
        &mut self,
        details: &mut TextDetails,
        font_details: FontDetails,
    ) -> Result<Rc<Texture>, String>;

    fn load_image(&mut self, path: String) -> Result<Rc<Texture>, String>;
}

#[cfg_attr(tarpaulin, skip)]
pub struct CanvasRenderer<'l> {
    config: ConfigAccess,
    font_manager: FontManager<'l>,
    texture_manager: TextureManager<'l, sdl2::video::WindowContext>,
    character_sizes: HashMap<TextCharacterDetails, Rect>,
}

#[cfg_attr(tarpaulin, skip)]
impl<'l> CanvasRenderer<'l> {
    pub fn new(
        config: ConfigAccess,
        font_context: &'l Sdl2TtfContext,
        texture_creator: &'l TextureCreator<sdl2::video::WindowContext>,
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
}

#[cfg_attr(tarpaulin, skip)]
impl<'l> CharacterSizeManager for CanvasRenderer<'l> {
    fn load_character_size(&mut self, c: char) -> Rect {
        let (font_path, font_size) = {
            let config = self.config().read().unwrap();
            (
                config.editor_config().font_path().to_string(),
                config.editor_config().character_size().clone(),
            )
        };
        let details = TextCharacterDetails {
            c: c.clone(),
            font_path: font_path.to_string(),
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

#[cfg_attr(tarpaulin, skip)]
impl<'l> ConfigHolder for CanvasRenderer<'l> {
    fn config(&self) -> &ConfigAccess {
        &self.config
    }
}

#[cfg_attr(tarpaulin, skip)]
impl<'l> Renderer for CanvasRenderer<'l> {
    fn load_font(&mut self, details: FontDetails) -> Rc<Font> {
        self.font_manager
            .load(&details)
            .unwrap_or_else(|_| panic!("Font not found {:?}", details))
    }

    fn load_text_tex(
        &mut self,
        details: &mut TextDetails,
        font_details: FontDetails,
    ) -> Result<Rc<Texture>, String> {
        use crate::renderer::managers::*;
        let font = self
            .font_manager
            .load(&font_details)
            .unwrap_or_else(|_| panic!("Font not found {:?}", details));
        self.texture_manager.load_text(details, font)
    }

    fn load_image(&mut self, path: String) -> Result<Rc<Texture>, String> {
        self.texture_manager.load(path.as_str())
    }
}
