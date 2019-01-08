use sdl2::image::LoadTexture;
use sdl2::pixels::Color;
use sdl2::render::{Texture, TextureCreator};
use sdl2::ttf::{Font, Sdl2TtfContext};
use sdl2::video::WindowContext as WinCtxt;
use std::borrow::Borrow;
use std::collections::HashMap;
#[allow(unused_imports)]
use std::env;
use std::hash::Hash;
use std::rc::Rc;

//noinspection RsWrongLifetimeParametersNumber
pub type RcTex<'l> = Rc<Texture<'l>>;
pub type RcFont<'l> = Rc<Font<'l, 'static>>;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct TextCharacterDetails {
    pub c: char,
    pub font_path: String,
    pub font_size: u16,
}

pub trait ResourceLoader<'l, R> {
    type Args: ?Sized;

    fn load(&'l self, data: &Self::Args) -> Result<R, String>;
}

#[derive(Debug, Hash, Eq, PartialEq, Clone)]
pub struct FontDetails {
    pub path: String,
    pub size: u16,
}

#[derive(Debug, Hash, Eq, PartialEq, Clone)]
pub struct TextDetails {
    pub text: String,
    pub color: Color,
    pub font: FontDetails,
}

impl TextDetails {
    pub fn get_cache_key(&self) -> String {
        format!(
            "text({}) size({}) {:?}",
            self.text, self.font.size, self.color
        )
        .to_string()
    }
}

impl<'a> From<&'a TextDetails> for TextDetails {
    fn from(details: &'a Self) -> Self {
        Self {
            text: details.text.clone(),
            color: details.color.clone(),
            font: details.font.clone(),
        }
    }
}

impl FontDetails {
    pub fn new(path: &str, size: u16) -> FontDetails {
        Self {
            path: path.to_string(),
            size,
        }
    }
}

impl<'a> From<&'a FontDetails> for FontDetails {
    fn from(details: &'a FontDetails) -> Self {
        Self {
            path: details.path.clone(),
            size: details.size,
        }
    }
}

//noinspection RsWrongLifetimeParametersNumber
pub type TextureManager<'l, T> = ResourceManager<'l, String, Texture<'l>, TextureCreator<T>>;
pub type FontManager<'l> = ResourceManager<'l, FontDetails, Font<'l, 'static>, Sdl2TtfContext>;

pub trait ManagersHolder<'l> {
    fn font_manager(&mut self) -> &mut FontManager<'l>;

    fn texture_manager(&mut self) -> &mut TextureManager<'l, WinCtxt>;
}

#[derive(Clone)]
pub struct ResourceManager<'l, K, R, L>
where
    K: Hash + Eq,
    L: 'l + ResourceLoader<'l, R>,
{
    loader: &'l L,
    cache: HashMap<K, Rc<R>>,
}

impl<'l, K, R, L> ResourceManager<'l, K, R, L>
where
    K: Hash + Eq,
    L: ResourceLoader<'l, R>,
{
    pub fn new(loader: &'l L) -> Self {
        Self {
            cache: HashMap::new(),
            loader,
        }
    }

    pub fn load<D>(&mut self, details: &D) -> Result<Rc<R>, String>
    where
        L: ResourceLoader<'l, R, Args = D>,
        D: Eq + Hash + ?Sized,
        K: Borrow<D> + for<'a> From<&'a D>,
    {
        self.cache.get(details).cloned().map_or_else(
            || {
                let resource = Rc::new(self.loader.load(details)?);
                self.cache.insert(details.into(), resource.clone());
                Ok(resource)
            },
            Ok,
        )
    }

    pub fn loader(&self) -> &L {
        self.loader
    }
}

//noinspection RsWrongLifetimeParametersNumber
impl<'l, T> ResourceLoader<'l, Texture<'l>> for TextureCreator<T> {
    type Args = str;

    fn load(&'l self, path: &str) -> Result<Texture, String> {
        println!("Loading {}...", path);
        self.load_texture(path)
    }
}

impl<'l> ResourceLoader<'l, Font<'l, 'static>> for Sdl2TtfContext {
    type Args = FontDetails;

    fn load(&'l self, data: &FontDetails) -> Result<Font<'l, 'static>, String> {
        info!("Loading font {}...", data.path);
        self.load_font(&data.path, data.size)
    }
}

pub trait TextTextureManager<'l> {
    //noinspection RsWrongLifetimeParametersNumber
    fn load_text(
        &mut self,
        details: &mut TextDetails,
        font: &Rc<Font>,
    ) -> Result<Rc<Texture<'l>>, String>;
}

impl<'l, T> TextTextureManager<'l> for TextureManager<'l, T> {
    //noinspection RsWrongLifetimeParametersNumber
    fn load_text(
        &mut self,
        details: &mut TextDetails,
        font: &Rc<Font>,
    ) -> Result<Rc<Texture<'l>>, String> {
        let key = details.get_cache_key();
        self.cache.get(key.as_str()).cloned().map_or_else(
            || {
                let surface = font
                    .render(details.text.as_str())
                    .blended(details.color)
                    .unwrap();
                let texture = self.loader.create_texture_from_surface(&surface).unwrap();
                let resource = Rc::new(texture);
                self.cache.insert(key, resource.clone());
                for c in details.text.chars() {
                    info!("texture for '{:?}' created", c);
                }
                Ok(resource)
            },
            Ok,
        )
    }
}
