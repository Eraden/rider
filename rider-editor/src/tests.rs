#[cfg(test)]
pub mod support {
    use crate::renderer::managers::FontDetails;
    use crate::renderer::managers::TextDetails;
    use crate::renderer::renderer::Renderer;
    use crate::ui::text_character::CharacterSizeManager;
    use crate::ui::CanvasAccess;
    use rider_config::Config;
    use rider_config::ConfigAccess;
    use rider_config::ConfigHolder;
    use sdl2::pixels::Color;
    use sdl2::rect::Point;
    use sdl2::rect::Rect;
    use sdl2::render::Texture;
    use sdl2::ttf::Font;
    use std::fmt::Debug;
    use std::fmt::Error;
    use std::fmt::Formatter;
    use std::rc::Rc;
    use std::sync::*;

    pub fn build_config() -> Arc<RwLock<Config>> {
        let mut config = Config::new();
        config.set_theme(config.editor_config().current_theme().clone());
        Arc::new(RwLock::new(config))
    }

    #[derive(Debug, PartialEq)]
    pub struct RendererRect {
        pub rect: Rect,
        pub color: Color,
    }

    #[cfg_attr(tarpaulin, skip)]
    pub struct CanvasMock {
        pub rects: Vec<RendererRect>,
        pub borders: Vec<RendererRect>,
        pub lines: Vec<RendererRect>,
        pub clippings: Vec<Rect>,
    }

    #[cfg_attr(tarpaulin, skip)]
    impl Debug for CanvasMock {
        fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
            write!(f, "CanvasMock {{}}")
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
            }
        }
    }

    #[cfg_attr(tarpaulin, skip)]
    impl CanvasAccess for CanvasMock {
        fn render_rect(&mut self, rect: Rect, color: Color) -> Result<(), String> {
            self.rects.push(RendererRect { rect, color });
            Ok(())
        }

        fn render_border(&mut self, rect: Rect, color: Color) -> Result<(), String> {
            self.borders.push(RendererRect { rect, color });
            Ok(())
        }

        fn render_image(
            &mut self,
            _tex: Rc<Texture>,
            _src: Rect,
            _dest: Rect,
        ) -> Result<(), String> {
            unimplemented!()
        }

        fn render_line(&mut self, start: Point, end: Point, color: Color) -> Result<(), String> {
            self.lines.push(RendererRect {
                rect: Rect::new(start.x(), start.y(), end.x() as u32, end.y() as u32),
                color,
            });
            Ok(())
        }

        fn set_clipping(&mut self, rect: Rect) {
            self.clippings.push(rect);
        }
    }

    #[cfg_attr(tarpaulin, skip)]
    pub struct SimpleRendererMock {
        config: ConfigAccess,
    }

    #[cfg_attr(tarpaulin, skip)]
    impl SimpleRendererMock {
        pub fn new(config: ConfigAccess) -> Self {
            Self { config }
        }
    }

    #[cfg_attr(tarpaulin, skip)]
    impl Renderer for SimpleRendererMock {
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

    #[cfg_attr(tarpaulin, skip)]
    impl CharacterSizeManager for SimpleRendererMock {
        fn load_character_size(&mut self, _c: char) -> Rect {
            Rect::new(0, 0, 13, 14)
        }
    }

    #[cfg_attr(tarpaulin, skip)]
    impl ConfigHolder for SimpleRendererMock {
        fn config(&self) -> &Arc<RwLock<Config>> {
            &self.config
        }
    }
}
