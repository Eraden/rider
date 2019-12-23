#[cfg_attr(tarpaulin, skip)]
#[cfg(test)]
pub mod support {
    use crate::renderer::managers::FontDetails;
    use crate::renderer::managers::TextDetails;
    use crate::renderer::renderer::Renderer;
    use crate::renderer::{
        FontManager, ManagersHolder, ResourceLoader, ResourceManager, TextureManager,
    };
    use crate::ui::text_character::CharacterSizeManager;
    use crate::ui::CanvasAccess;
    use rider_config::Config;
    use rider_config::ConfigAccess;
    use rider_config::ConfigHolder;
    use sdl2::pixels::Color;
    use sdl2::rect::Point;
    use sdl2::rect::Rect;
    use sdl2::render::{Texture, TextureCreator};
    use sdl2::ttf::{Font, Sdl2TtfContext};
    use sdl2::video::WindowContext;
    use std::collections::HashMap;
    use std::fmt::Debug;
    use std::fmt::Error;
    use std::fmt::Formatter;
    use std::rc::Rc;
    use std::sync::*;

    pub fn build_path(path: String) {
        use std::fs;

        fs::create_dir_all(path.as_str()).unwrap();
        fs::write((path.clone() + &"/file1".to_owned()).as_str(), "foo").unwrap();
        fs::write((path.clone() + &"/file2".to_owned()).as_str(), "bar").unwrap();
        fs::create_dir_all((path.clone() + &"/dir1".to_owned()).as_str()).unwrap();
        fs::create_dir_all((path.clone() + &"/dir2".to_owned()).as_str()).unwrap();
    }

    pub fn build_config() -> Arc<RwLock<Config>> {
        let mut config = Config::new();
        config.set_theme(config.editor_config().current_theme().clone());
        Arc::new(RwLock::new(config))
    }

    #[derive(Debug, PartialEq)]
    pub enum CanvasShape {
        Line,
        Border,
        Rectangle,
        Image(Rect, Rect, String),
    }

    #[derive(Debug, PartialEq)]
    pub struct RendererRect {
        pub rect: Rect,
        pub color: Color,
        pub shape: CanvasShape,
    }

    impl RendererRect {
        pub fn new(rect: Rect, color: Color, shape: CanvasShape) -> Self {
            Self { rect, color, shape }
        }
    }

    pub struct CanvasMock {
        pub rects: Vec<RendererRect>,
        pub borders: Vec<RendererRect>,
        pub lines: Vec<RendererRect>,
        pub clippings: Vec<Option<Rect>>,
        pub character_sizes: HashMap<char, sdl2::rect::Rect>,
    }

    impl Debug for CanvasMock {
        fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
            write!(
                f,
                "CanvasMock {{ {:?} {:?} {:?} }}",
                self.rects, self.lines, self.clippings
            )
        }
    }

    impl PartialEq for CanvasMock {
        fn eq(&self, other: &CanvasMock) -> bool {
            self.rects == other.rects
                && self.borders == other.borders
                && self.clippings == other.clippings
                && self.lines == other.lines
        }
    }

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

    impl CanvasMock {
        pub fn set_character_rect(&mut self, c: char, rect: Rect) {
            self.character_sizes.insert(c, rect);
        }

        pub fn find_pixel_with_color(
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

        pub fn find_rect_with_color(
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

        pub fn find_line_with_color(
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

        pub fn find_border_with_color(
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

    //    struct TextureLoaderMock {}

    //    impl<'l> ResourceLoader<'l> for  TextureLoaderMock {}

    pub struct SimpleRendererMock<'l> {
        config: ConfigAccess,
        //        font_manager: FontManager<'l>,
        //        texture_manager: TextureManager<'l>,
    }

    impl SimpleRendererMock {
        pub fn new(config: ConfigAccess) -> Self {
            Self { config }
        }
    }

    impl<'l> Renderer for SimpleRendererMock<'l> {
        fn load_font(&mut self, _details: FontDetails) -> Rc<Font> {
            unimplemented!()
        }

        fn load_text_tex(
            &mut self,
            _details: &mut TextDetails,
            _font_details: FontDetails,
        ) -> Result<Rc<Texture>, String> {
            Err("skip text texture".to_owned())
        }

        fn load_image(&mut self, _path: String) -> Result<Rc<Texture>, String> {
            Err("skip img texture".to_owned())
        }
    }

    impl<'l> CharacterSizeManager for SimpleRendererMock<'l> {
        fn load_character_size(&mut self, _c: char) -> Rect {
            Rect::new(0, 0, 13, 14)
        }
    }

    impl<'l> ConfigHolder for SimpleRendererMock<'l> {
        fn config(&self) -> &Arc<RwLock<Config>> {
            &self.config
        }
    }

    impl<'l> ManagersHolder<'l> for SimpleRendererMock<'l> {
        fn font_manager(&mut self) -> &mut FontManager {
            unimplemented!()
        }

        fn texture_manager(&mut self) -> &mut TextureManager<'l> {
            unimplemented!()
        }
    }
}
