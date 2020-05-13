use crate::renderer::managers::FontDetails;
use crate::renderer::managers::TextDetails;
use crate::renderer::renderer::Renderer;
use crate::renderer::TextureManager;
use crate::ui::text_character::CharacterSizeManager;
use crate::ui::CanvasAccess;
use rider_config::Config;
use rider_config::ConfigAccess;
use rider_config::ConfigHolder;
use sdl2::pixels::{Color, PixelFormatEnum};
use sdl2::rect::Point;
use sdl2::rect::Rect;
use sdl2::render::{Texture, TextureCreator};
use sdl2::ttf::{Font, Sdl2TtfContext};
use std::collections::HashMap;
use std::fmt::Debug;
use std::fmt::Error;
use std::fmt::Formatter;
use std::rc::Rc;
use std::sync::*;

#[cfg_attr(tarpaulin, skip)]
pub fn build_path(path: String) {
    use std::fs;

    fs::create_dir_all(path.as_str()).unwrap();
    fs::write((path.clone() + &"/file1".to_owned()).as_str(), "foo").unwrap();
    fs::write((path.clone() + &"/file2".to_owned()).as_str(), "bar").unwrap();
    fs::create_dir_all((path.clone() + &"/dir1".to_owned()).as_str()).unwrap();
    fs::create_dir_all((path.clone() + &"/dir2".to_owned()).as_str()).unwrap();
}

#[cfg_attr(tarpaulin, skip)]
pub fn build_config() -> Arc<RwLock<Config>> {
    let mut config = Config::new();
    config.set_theme(config.editor_config().current_theme().clone());
    Arc::new(RwLock::new(config))
}

#[cfg_attr(tarpaulin, skip)]
#[derive(Debug, PartialEq)]
pub enum CanvasShape {
    Line,
    Border,
    Rectangle,
    Image(Rect, Rect, String),
}

#[cfg_attr(tarpaulin, skip)]
#[derive(Debug, PartialEq)]
pub struct RendererRect {
    pub rect: Rect,
    pub color: Color,
    pub shape: CanvasShape,
}

#[cfg_attr(tarpaulin, skip)]
impl RendererRect {
    pub fn new(rect: Rect, color: Color, shape: CanvasShape) -> Self {
        Self { rect, color, shape }
    }
}

#[cfg_attr(tarpaulin, skip)]
pub struct CanvasMock {
    pub rects: Vec<RendererRect>,
    pub borders: Vec<RendererRect>,
    pub lines: Vec<RendererRect>,
    pub clippings: Vec<Option<Rect>>,
    pub character_sizes: HashMap<char, sdl2::rect::Rect>,
}

#[cfg_attr(tarpaulin, skip)]
impl Debug for CanvasMock {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        write!(
            f,
            "CanvasMock {{ {:?} {:?} {:?} }}",
            self.rects, self.lines, self.clippings
        )
    }
}

#[cfg_attr(tarpaulin, skip)]
impl PartialEq for CanvasMock {
    fn eq(&self, other: &CanvasMock) -> bool {
        self.rects == other.rects
            && self.borders == other.borders
            && self.clippings == other.clippings
            && self.lines == other.lines
    }
}

#[cfg_attr(tarpaulin, skip)]
impl CanvasMock {
    pub fn new() -> Self {
        Self {
            rects: vec![],
            borders: vec![],
            lines: vec![],
            clippings: vec![],
            character_sizes: HashMap::new(),
        }
    }
}

#[cfg_attr(tarpaulin, skip)]
impl CanvasAccess for CanvasMock {
    fn render_rect(&mut self, rect: Rect, color: Color) -> Result<(), String> {
        self.rects.push(RendererRect {
            rect,
            color,
            shape: CanvasShape::Rectangle,
        });
        Ok(())
    }

    fn render_border(&mut self, rect: Rect, color: Color) -> Result<(), String> {
        self.borders.push(RendererRect {
            rect,
            color,
            shape: CanvasShape::Border,
        });
        Ok(())
    }

    fn render_image(&mut self, _tex: Rc<Texture>, src: Rect, dest: Rect) -> Result<(), String> {
        self.rects.push(RendererRect::new(
            dest.clone(),
            Color::RGBA(0, 0, 0, 255),
            CanvasShape::Image(src.clone(), dest.clone(), format!("_tex: Rc<Texture>")),
        ));
        Ok(())
    }

    fn render_line(&mut self, start: Point, end: Point, color: Color) -> Result<(), String> {
        self.lines.push(RendererRect {
            rect: Rect::new(start.x(), start.y(), end.x() as u32, end.y() as u32),
            color,
            shape: CanvasShape::Line,
        });
        Ok(())
    }

    fn set_clipping(&mut self, rect: Rect) {
        self.clippings.push(Some(rect));
    }

    fn set_clip_rect(&mut self, rect: Option<Rect>) {
        self.clippings.push(rect);
    }

    fn clip_rect(&self) -> Option<Rect> {
        self.clippings.last().cloned().unwrap_or_else(|| None)
    }
}

#[cfg_attr(tarpaulin, skip)]
impl CharacterSizeManager for CanvasMock {
    fn load_character_size(&mut self, c: char) -> Rect {
        match self.character_sizes.get(&c) {
            Some(r) => r.clone(),
            None => {
                self.character_sizes.insert(c, Rect::new(0, 0, 1, 1));
                self.character_sizes.get(&c).cloned().unwrap()
            }
        }
    }
}

#[cfg_attr(tarpaulin, skip)]
pub trait CanvasTester {
    fn set_character_rect(&mut self, c: char, rect: Rect);

    fn find_pixel_with_color(
        &self,
        point: sdl2::rect::Point,
        color: sdl2::pixels::Color,
    ) -> Option<&RendererRect>;

    fn find_rect_with_color(
        &self,
        subject: sdl2::rect::Rect,
        color: sdl2::pixels::Color,
    ) -> Option<&RendererRect>;

    fn find_line_with_color(
        &self,
        subject: sdl2::rect::Rect,
        color: sdl2::pixels::Color,
    ) -> Option<&RendererRect>;

    fn find_border_with_color(
        &self,
        subject: sdl2::rect::Rect,
        color: sdl2::pixels::Color,
    ) -> Option<&RendererRect>;
}

#[cfg_attr(tarpaulin, skip)]
impl CanvasTester for CanvasMock {
    fn set_character_rect(&mut self, c: char, rect: Rect) {
        self.character_sizes.insert(c, rect);
    }

    fn find_pixel_with_color(
        &self,
        point: sdl2::rect::Point,
        color: sdl2::pixels::Color,
    ) -> Option<&RendererRect> {
        for rect in self.rects.iter() {
            if rect.rect.contains_point(point.clone()) && rect.color == color {
                return Some(rect.clone());
            }
        }
        for rect in self.borders.iter() {
            if rect.rect.contains_point(point.clone()) && rect.color == color {
                return Some(rect.clone());
            }
        }
        for rect in self.lines.iter() {
            if rect.rect.contains_point(point.clone()) && rect.color == color {
                return Some(rect.clone());
            }
        }
        None
    }

    fn find_rect_with_color(
        &self,
        subject: sdl2::rect::Rect,
        color: sdl2::pixels::Color,
    ) -> Option<&RendererRect> {
        for rect in self.rects.iter() {
            if rect.rect == subject && rect.color == color {
                return Some(rect.clone());
            }
        }
        None
    }

    fn find_line_with_color(
        &self,
        subject: sdl2::rect::Rect,
        color: sdl2::pixels::Color,
    ) -> Option<&RendererRect> {
        for rect in self.lines.iter() {
            if rect.rect == subject && rect.color == color {
                return Some(rect.clone());
            }
        }
        None
    }

    fn find_border_with_color(
        &self,
        subject: sdl2::rect::Rect,
        color: sdl2::pixels::Color,
    ) -> Option<&RendererRect> {
        for rect in self.borders.iter() {
            if rect.rect == subject && rect.color == color {
                return Some(rect.clone());
            }
        }
        None
    }
}

#[cfg_attr(tarpaulin, skip)]
pub struct SimpleRendererMock<'l> {
    pub config: ConfigAccess,
    pub ttf: Sdl2TtfContext,
    pub character_sizes: HashMap<char, Rect>,
    pub texture_manager: TextureManager<'l, sdl2::surface::SurfaceContext<'l>>,
}

#[cfg_attr(tarpaulin, skip)]
impl<'l> SimpleRendererMock<'l> {
    pub fn set_character_rect(&mut self, c: char, rect: Rect) {
        self.character_sizes.insert(c, rect);
    }

    pub fn texture_creator(&self) -> &TextureCreator<sdl2::surface::SurfaceContext<'l>> {
        self.texture_manager.loader()
    }
}

#[cfg_attr(tarpaulin, skip)]
impl<'l> Renderer for SimpleRendererMock<'l> {
    fn load_font(&mut self, details: FontDetails) -> Rc<Font> {
        Rc::new(
            self.ttf
                .load_font(details.path, details.size)
                .unwrap_or_else(|e| panic!("{:?}", e)),
        )
    }

    fn load_text_tex(
        &mut self,
        _details: &mut TextDetails,
        _font_details: FontDetails,
    ) -> Result<Rc<Texture>, String> {
        self.texture_creator()
            .create_texture(
                PixelFormatEnum::RGB24,
                sdl2::render::TextureAccess::Target,
                24,
                24,
            )
            .map_err(|e| format!("{:?}", e))
            .map(|t| Rc::new(t))
    }

    fn load_image(&mut self, path: String) -> Result<Rc<Texture>, String> {
        self.texture_manager.load(path.as_str())
    }
}

#[cfg_attr(tarpaulin, skip)]
impl<'l> CharacterSizeManager for SimpleRendererMock<'l> {
    fn load_character_size(&mut self, c: char) -> Rect {
        match self.character_sizes.get(&c) {
            Some(r) => r.clone(),
            _ => {
                let rect = Rect::new(0, 0, 13, 14);
                self.set_character_rect(c.clone(), rect.clone());
                rect
            }
        }
    }
}

#[cfg_attr(tarpaulin, skip)]
impl<'l> ConfigHolder for SimpleRendererMock<'l> {
    fn config(&self) -> &Arc<RwLock<Config>> {
        &self.config
    }
}

#[cfg_attr(tarpaulin, skip)]
pub type TestCanvas<'r> = sdl2::render::Canvas<sdl2::surface::Surface<'r>>;

#[cfg_attr(tarpaulin, skip)]
impl<'r> CanvasAccess for TestCanvas<'r> {
    fn render_rect(&mut self, rect: Rect, color: sdl2::pixels::Color) -> Result<(), String> {
        self.set_draw_color(color);
        self.fill_rect(rect)
    }

    fn render_border(&mut self, rect: Rect, color: sdl2::pixels::Color) -> Result<(), String> {
        self.set_draw_color(color);
        self.draw_rect(rect)
    }

    fn render_image(&mut self, tex: Rc<Texture>, src: Rect, dest: Rect) -> Result<(), String> {
        self.copy_ex(&tex, Some(src), Some(dest), 0.0, None, false, false)
    }

    fn render_line(
        &mut self,
        start: Point,
        end: Point,
        color: sdl2::pixels::Color,
    ) -> Result<(), String> {
        self.set_draw_color(color);
        self.draw_line(start, end)
    }

    fn set_clipping(&mut self, rect: Rect) {
        self.set_clip_rect(rect);
    }

    fn set_clip_rect(&mut self, rect: Option<Rect>) {
        self.set_clip_rect(rect);
    }

    fn clip_rect(&self) -> Option<Rect> {
        self.clip_rect()
    }
}

#[cfg_attr(tarpaulin, skip)]
pub trait DumpImage {
    fn dump_ui<S>(&self, path: S)
    where
        S: Into<String>;
}

#[cfg_attr(tarpaulin, skip)]
impl<'r> DumpImage for TestCanvas<'r> {
    fn dump_ui<S>(&self, path: S)
    where
        S: Into<String>,
    {
        let p = std::path::PathBuf::from(path.into());
        std::fs::create_dir_all(p.parent().unwrap()).unwrap();
        self.surface()
            .save_bmp(p)
            .expect("Failed to save canvas as BMP file");
    }
}

//    impl<'l> ManagersHolder<'l> for SimpleRendererMock {
//        fn font_manager(&mut self) -> &mut FontManager {
//            &mut self.font_manager
//        }
//
//        fn texture_manager(&mut self) -> &mut TextureManager<'l> {
//            &mut self.texture_manager
//        }
//    }
