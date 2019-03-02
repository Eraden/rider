use crate::app::UpdateResult as UR;
use crate::renderer::*;
use crate::ui::*;
use rider_config::Config;
use rider_config::ConfigHolder;
use rider_lexers::TokenType;
use sdl2::pixels::Color;
use sdl2::rect::{Point, Rect};
use std::sync::*;

#[derive(Clone, Debug)]
pub struct EditorFileToken {
    last_in_line: bool,
    characters: Vec<TextCharacter>,
    token_type: TokenType,
    config: Arc<RwLock<Config>>,
}

impl EditorFileToken {
    pub fn new(token_type: &TokenType, last_in_line: bool, config: Arc<RwLock<Config>>) -> Self {
        Self {
            last_in_line,
            characters: vec![],
            token_type: token_type.clone(),
            config,
        }
    }

    pub fn is_last_in_line(&self) -> bool {
        self.last_in_line
    }

    pub fn is_new_line(&self) -> bool {
        self.token_type.is_new_line()
    }

    pub fn update_position(&mut self, current: &mut Rect) {
        for text_character in self.characters.iter_mut() {
            text_character.update_position(current);
        }
    }

    fn token_to_color(&self, config: &Arc<RwLock<Config>>) -> Color {
        let config = config.read().unwrap();
        let ch = config.theme().code_highlighting();
        match &self.token_type {
            &TokenType::Whitespace { .. } => ch.whitespace().color().into(),
            &TokenType::Keyword { .. } => ch.keyword().color().into(),
            &TokenType::String { .. } => ch.string().color().into(),
            &TokenType::Identifier { .. } => ch.identifier().color().into(),
            &TokenType::Literal { .. } => ch.literal().color().into(),
            &TokenType::Comment { .. } => ch.comment().color().into(),
            &TokenType::Operator { .. } => ch.operator().color().into(),
            &TokenType::Separator { .. } => ch.separator().color().into(),
        }
    }
}

impl TextWidget for EditorFileToken {
    fn full_rect(&self) -> Rect {
        let mut rect = Rect::new(0, 0, 0, 0);
        match self.characters.first() {
            Some(c) => {
                rect.set_x(c.dest().x());
                rect.set_y(c.dest().y());
                rect.set_width(c.dest().width());
                rect.set_height(c.dest().height());
            }
            _ => return rect,
        };
        rect
    }
}

impl TextCollection for EditorFileToken {
    fn get_character_at(&self, index: usize) -> Option<TextCharacter> {
        for character in self.characters.iter() {
            if character.position() == index {
                return Some(character.clone());
            }
        }
        None
    }

    fn get_line(&self, line: &usize) -> Option<Vec<&TextCharacter>> {
        let mut vec: Vec<&TextCharacter> = vec![];
        for c in self.characters.iter() {
            match (
                line.clone(),
                c.line().clone(),
                self.token_type.is_new_line(),
            ) {
                (0, 0, true) => {
                    vec.push(c);
                }
                (a, b, true) if (a + 1) == b => {
                    vec.push(c);
                }
                (a, b, true) if a != (b + 1) => (),
                (a, b, false) if a == b => {
                    vec.push(c);
                }
                _t => (),
            }
        }
        if vec.is_empty() {
            None
        } else {
            Some(vec)
        }
    }

    fn get_last_at_line(&self, line: usize) -> Option<TextCharacter> {
        let mut current: Option<&TextCharacter> = None;
        for text_character in self.characters.iter() {
            if !text_character.is_last_in_line() {
                continue;
            }
            if text_character.line() == line {
                current = Some(text_character);
            }
        }
        current.map(|c| c.clone())
    }
}

impl EditorFileToken {
    /**
     * Must first create targets so even if new line appear renderer will know
     * where move render starting point
     */
    pub fn render<R, C>(&self, canvas: &mut C, renderer: &mut R, context: &RenderContext)
    where
        R: Renderer + ConfigHolder,
        C: CanvasAccess,
    {
        if self.token_type.is_new_line() {
            return;
        }
        for text_character in self.characters.iter() {
            text_character.render(canvas, renderer, context);
        }
    }

    pub fn prepare_ui<R>(&mut self, renderer: &mut R)
    where
        R: ConfigHolder + CharacterSizeManager + Renderer,
    {
        if !self.characters.is_empty() {
            return;
        }
        let color: Color = self.token_to_color(&renderer.config());
        let chars: Vec<char> = self.token_type.text().chars().collect();
        for (index, c) in chars.iter().enumerate() {
            let last_in_line = self.last_in_line && index + 1 == chars.len();
            let mut text_character: TextCharacter = TextCharacter::new(
                c.clone(),
                self.token_type.start() + index,
                self.token_type.line(),
                last_in_line,
                color,
                self.config.clone(),
            );
            text_character.prepare_ui(renderer);
            self.characters.push(text_character);
        }
    }
}

impl Update for EditorFileToken {
    fn update(&mut self, ticks: i32, context: &UpdateContext) -> UR {
        for text_character in self.characters.iter_mut() {
            text_character.update(ticks, context);
        }
        UR::NoOp
    }
}

impl ClickHandler for EditorFileToken {
    fn on_left_click(&mut self, point: &Point, context: &UpdateContext) -> UR {
        for text_character in self.characters.iter_mut() {
            if text_character.is_left_click_target(point, context) {
                return text_character.on_left_click(point, context);
            }
        }
        UR::NoOp
    }

    fn is_left_click_target(&self, point: &Point, context: &UpdateContext) -> bool {
        for text_character in self.characters.iter() {
            if text_character.is_left_click_target(point, context) {
                return true;
            }
        }
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::support::build_config;
    use rider_lexers::Token;
    use sdl2::pixels::PixelFormatEnum;
    use sdl2::render::Texture;
    use sdl2::render::TextureCreator;
    use sdl2::surface::Surface;
    use sdl2::surface::SurfaceContext;
    use sdl2::ttf::Font;
    use std::fmt::Debug;
    use std::fmt::Error;
    use std::fmt::Formatter;
    use std::rc::Rc;
    use std::sync::{Arc, RwLock};

    //##################################################
    // models
    //##################################################

    #[derive(Debug, PartialEq)]
    struct RendererRect {
        pub rect: Rect,
        pub color: Color,
    }

    #[cfg_attr(tarpaulin, skip)]
    struct CanvasMock {
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
    struct RendererMock<'l> {
        pub config: Arc<RwLock<Config>>,
        pub canvas: sdl2::render::Canvas<Surface<'l>>,
        pub map: Vec<Rc<Texture<'l>>>,
        pub creator: TextureCreator<SurfaceContext<'l>>,
    }

    #[cfg_attr(tarpaulin, skip)]
    impl<'l> RendererMock<'l> {
        pub fn new(config: Arc<RwLock<Config>>, surface: Surface<'l>) -> Self {
            let canvas = sdl2::render::Canvas::from_surface(surface).unwrap();
            Self {
                config,
                creator: canvas.texture_creator(),
                canvas,
                map: vec![],
            }
        }
    }

    #[cfg_attr(tarpaulin, skip)]
    impl<'l> Renderer for RendererMock<'l> {
        fn load_font(&mut self, _details: FontDetails) -> Rc<Font> {
            unimplemented!("load_font")
        }

        fn load_text_tex(
            &mut self,
            _details: &mut TextDetails,
            _font_details: FontDetails,
        ) -> Result<Rc<Texture>, String> {
            //            self.map.get(0).cloned().map_or_else(|| {
            //                let surface = font
            //                    .render(details.text.as_str())
            //                    .blended(details.color)
            //                    .unwrap();
            //                let texture = self.loader.create_texture_from_surface(&surface).unwrap();
            //                let resource = Rc::new(texture);
            //                self.map.push(resource.clone());
            //                Ok(resource)
            //            }, Ok)
            Err("".to_owned())
        }
    }

    impl<'l> CharacterSizeManager for RendererMock<'l> {
        fn load_character_size(&mut self, _c: char) -> Rect {
            Rect::new(0, 0, 13, 14)
        }
    }

    impl<'l> ConfigHolder for RendererMock<'l> {
        fn config(&self) -> &Arc<RwLock<Config>> {
            &self.config
        }
    }

    //##################################################
    // token_to_color
    //##################################################

    #[test]
    fn assert_whitespace_to_color() {
        let config = build_config();
        let surface = Surface::new(1024, 800, PixelFormatEnum::RGBA8888).unwrap();
        let mut renderer = RendererMock::new(config.clone(), surface);
        let token_type = TokenType::Whitespace {
            token: Token::new("".to_owned(), 0, 0, 0, 0),
        };
        let mut token = EditorFileToken::new(&token_type, false, config.clone());
        token.prepare_ui(&mut renderer);
    }
    #[test]
    fn assert_keyword_to_color() {
        let config = build_config();
        let surface = Surface::new(1024, 800, PixelFormatEnum::RGBA8888).unwrap();
        let mut renderer = RendererMock::new(config.clone(), surface);
        let token_type = TokenType::Keyword {
            token: Token::new("".to_owned(), 0, 0, 0, 0),
        };
        let mut token = EditorFileToken::new(&token_type, false, config.clone());
        token.prepare_ui(&mut renderer);
    }
    #[test]
    fn assert_string_to_color() {
        let config = build_config();
        let surface = Surface::new(1024, 800, PixelFormatEnum::RGBA8888).unwrap();
        let mut renderer = RendererMock::new(config.clone(), surface);
        let token_type = TokenType::String {
            token: Token::new("".to_owned(), 0, 0, 0, 0),
        };
        let mut token = EditorFileToken::new(&token_type, false, config.clone());
        token.prepare_ui(&mut renderer);
    }
    #[test]
    fn assert_identifier_to_color() {
        let config = build_config();
        let surface = Surface::new(1024, 800, PixelFormatEnum::RGBA8888).unwrap();
        let mut renderer = RendererMock::new(config.clone(), surface);
        let token_type = TokenType::Identifier {
            token: Token::new("".to_owned(), 0, 0, 0, 0),
        };
        let mut token = EditorFileToken::new(&token_type, false, config.clone());
        token.prepare_ui(&mut renderer);
    }
    #[test]
    fn assert_literal_to_color() {
        let config = build_config();
        let surface = Surface::new(1024, 800, PixelFormatEnum::RGBA8888).unwrap();
        let mut renderer = RendererMock::new(config.clone(), surface);
        let token_type = TokenType::Literal {
            token: Token::new("".to_owned(), 0, 0, 0, 0),
        };
        let mut token = EditorFileToken::new(&token_type, false, config.clone());
        token.prepare_ui(&mut renderer);
    }
    #[test]
    fn assert_comment_to_color() {
        let config = build_config();
        let surface = Surface::new(1024, 800, PixelFormatEnum::RGBA8888).unwrap();
        let mut renderer = RendererMock::new(config.clone(), surface);
        let token_type = TokenType::Comment {
            token: Token::new("".to_owned(), 0, 0, 0, 0),
        };
        let mut token = EditorFileToken::new(&token_type, false, config.clone());
        token.prepare_ui(&mut renderer);
    }
    #[test]
    fn assert_operator_to_color() {
        let config = build_config();
        let surface = Surface::new(1024, 800, PixelFormatEnum::RGBA8888).unwrap();
        let mut renderer = RendererMock::new(config.clone(), surface);
        let token_type = TokenType::Operator {
            token: Token::new("".to_owned(), 0, 0, 0, 0),
        };
        let mut token = EditorFileToken::new(&token_type, false, config.clone());
        token.prepare_ui(&mut renderer);
    }
    #[test]
    fn assert_separator_to_color() {
        let config = build_config();
        let surface = Surface::new(1024, 800, PixelFormatEnum::RGBA8888).unwrap();
        let mut renderer = RendererMock::new(config.clone(), surface);
        let token_type = TokenType::Separator {
            token: Token::new("".to_owned(), 0, 0, 0, 0),
        };
        let mut token = EditorFileToken::new(&token_type, false, config.clone());
        token.prepare_ui(&mut renderer);
    }

    //##################################################
    // render
    //##################################################

    #[test]
    fn assert_is_last_in_line() {
        let config = build_config();
        let token = TokenType::String {
            token: Token::new("".to_string(), 0, 0, 0, 0),
        };
        let widget = EditorFileToken::new(&token, true, config);
        assert_eq!(widget.is_last_in_line(), true);
    }

    #[test]
    fn assert_is_not_last_in_line() {
        let config = build_config();
        let token = TokenType::String {
            token: Token::new("".to_string(), 0, 0, 0, 0),
        };
        let widget = EditorFileToken::new(&token, false, config);
        assert_eq!(widget.is_last_in_line(), false);
    }

    #[test]
    fn assert_is_new_line() {
        let config = build_config();
        let token = TokenType::Whitespace {
            token: Token::new("\n".to_string(), 0, 0, 0, 0),
        };
        let widget = EditorFileToken::new(&token, true, config);
        assert_eq!(widget.is_new_line(), true);
    }

    #[test]
    fn assert_is_not_new_line() {
        let config = build_config();
        let token = TokenType::String {
            token: Token::new("".to_string(), 0, 0, 0, 0),
        };
        let widget = EditorFileToken::new(&token, false, config);
        assert_eq!(widget.is_new_line(), false);
    }

    #[test]
    fn assert_empty_characters_update_position() {
        let config = build_config();
        let surface = Surface::new(1024, 1024, PixelFormatEnum::RGBA8888).unwrap();
        //        let mut canvas = CanvasMock::new();
        let mut renderer = RendererMock::new(config.clone(), surface);
        let token = TokenType::String {
            token: Token::new("".to_string(), 0, 0, 0, 0),
        };
        let mut widget = EditorFileToken::new(&token, false, config);
        let mut rect = Rect::new(1, 2, 3, 4);
        widget.prepare_ui(&mut renderer);
        widget.update_position(&mut rect);
        assert_eq!(widget.is_new_line(), false);
    }

    #[test]
    fn assert_some_characters_update_position() {
        let config = build_config();
        let surface = Surface::new(1024, 1024, PixelFormatEnum::RGBA8888).unwrap();
        //        let mut canvas = CanvasMock::new();
        let mut renderer = RendererMock::new(config.clone(), surface);
        let token = TokenType::String {
            token: Token::new("foo bar".to_string(), 0, 0, 0, 0),
        };
        let mut widget = EditorFileToken::new(&token, false, config);
        let mut rect = Rect::new(1, 2, 3, 4);
        widget.prepare_ui(&mut renderer);
        widget.update_position(&mut rect);
        assert_eq!(widget.is_new_line(), false);
    }

    #[test]
    fn assert_prepare_ui_non_empty() {
        let config = build_config();
        let surface = Surface::new(1024, 1024, PixelFormatEnum::RGBA8888).unwrap();
        //        let mut canvas = CanvasMock::new();
        let mut renderer = RendererMock::new(config.clone(), surface);
        let token = TokenType::String {
            token: Token::new("foo bar".to_string(), 0, 0, 0, 0),
        };
        let mut widget = EditorFileToken::new(&token, false, config);
        widget.prepare_ui(&mut renderer);
        widget.prepare_ui(&mut renderer);
        assert_eq!(widget.is_new_line(), false);
    }

    #[test]
    fn assert_update_empty() {
        let config = build_config();
        let token = TokenType::String {
            token: Token::new("".to_string(), 0, 0, 0, 0),
        };
        let mut widget = EditorFileToken::new(&token, false, config);
        widget.update(0, &UpdateContext::Nothing);
        assert_eq!(widget.is_new_line(), false);
    }

    #[test]
    fn assert_update_non_empty() {
        let config = build_config();
        let surface = Surface::new(1024, 1024, PixelFormatEnum::RGBA8888).unwrap();
        //        let mut canvas = CanvasMock::new();
        let mut renderer = RendererMock::new(config.clone(), surface);
        let token = TokenType::String {
            token: Token::new("foo bar".to_string(), 0, 0, 0, 0),
        };
        let mut widget = EditorFileToken::new(&token, false, config);
        widget.prepare_ui(&mut renderer);
        widget.update(0, &UpdateContext::Nothing);
        assert_eq!(widget.is_new_line(), false);
    }

    #[test]
    fn assert_get_character_on_empty() {
        let config = build_config();
        let surface = Surface::new(1024, 1024, PixelFormatEnum::RGBA8888).unwrap();
        //        let mut canvas = CanvasMock::new();
        let mut renderer = RendererMock::new(config.clone(), surface);
        let token = TokenType::String {
            token: Token::new("".to_string(), 0, 0, 0, 0),
        };
        let mut widget = EditorFileToken::new(&token, false, config);
        widget.prepare_ui(&mut renderer);
        let res = widget.get_character_at(0);
        assert_eq!(res.is_none(), true);
    }

    #[test]
    fn assert_get_character_non_empty() {
        let config = build_config();
        let surface = Surface::new(1024, 1024, PixelFormatEnum::RGBA8888).unwrap();
        //        let mut canvas = CanvasMock::new();
        let mut renderer = RendererMock::new(config.clone(), surface);
        let token = TokenType::String {
            token: Token::new("foo bar".to_string(), 0, 0, 0, 0),
        };
        let mut widget = EditorFileToken::new(&token, false, config.clone());
        widget.prepare_ui(&mut renderer);
        let res = widget.get_character_at(0);
        assert_eq!(res.is_none(), false);
        let mut expected =
            TextCharacter::new('f', 0, 0, false, Color::RGBA(135, 175, 95, 0), config);
        expected.prepare_ui(&mut renderer);
        assert_eq!(res, Some(expected));
    }

    #[test]
    fn assert_get_character_at_5_pos_with_non_empty() {
        let config = build_config();
        let surface = Surface::new(1024, 1024, PixelFormatEnum::RGBA8888).unwrap();
        //        let mut canvas = CanvasMock::new();
        let mut renderer = RendererMock::new(config.clone(), surface);
        let token = TokenType::String {
            token: Token::new("foo bar".to_string(), 0, 0, 0, 0),
        };
        let mut widget = EditorFileToken::new(&token, false, config.clone());
        widget.prepare_ui(&mut renderer);
        let res = widget.get_character_at(5);
        assert_eq!(res.is_none(), false);
        let mut expected =
            TextCharacter::new('a', 5, 0, false, Color::RGBA(135, 175, 95, 0), config);
        expected.prepare_ui(&mut renderer);
        assert_eq!(res, Some(expected));
    }

    #[test]
    fn assert_get_full_rect_on_empty() {
        let config = build_config();
        let surface = Surface::new(1024, 1024, PixelFormatEnum::RGBA8888).unwrap();
        //        let mut canvas = CanvasMock::new();
        let mut renderer = RendererMock::new(config.clone(), surface);
        let token = TokenType::String {
            token: Token::new("".to_string(), 0, 0, 0, 0),
        };
        let mut widget = EditorFileToken::new(&token, false, config);
        widget.prepare_ui(&mut renderer);
        let res = widget.full_rect();
        let expected = Rect::new(0, 0, 1, 1);
        assert_eq!(res, expected);
    }

    #[test]
    fn assert_get_full_rect_non_empty() {
        let config = build_config();
        let surface = Surface::new(1024, 1024, PixelFormatEnum::RGBA8888).unwrap();
        //        let mut canvas = CanvasMock::new();
        let mut renderer = RendererMock::new(config.clone(), surface);
        let token = TokenType::String {
            token: Token::new("foo bar".to_string(), 0, 0, 0, 0),
        };
        let mut widget = EditorFileToken::new(&token, false, config.clone());
        widget.prepare_ui(&mut renderer);
        let res = widget.full_rect();
        let expected = Rect::new(0, 0, 13, 14);
        assert_eq!(res, expected);
    }

    #[test]
    fn assert_render_on_empty() {
        let config = build_config();

        let surface = Surface::new(1024, 1024, PixelFormatEnum::RGBA8888).unwrap();
        let mut canvas = CanvasMock::new();
        let mut renderer = RendererMock::new(config.clone(), surface);
        let token = TokenType::String {
            token: Token::new("".to_string(), 0, 0, 0, 0),
        };
        let mut widget = EditorFileToken::new(&token, false, config);
        widget.prepare_ui(&mut renderer);
        widget.render(&mut canvas, &mut renderer, &RenderContext::Nothing);
        //        let surface = Surface::new(1024, 1024, PixelFormatEnum::RGBA8888).unwrap();
        let expected = CanvasMock::new();
        assert_eq!(canvas, expected);
    }

    #[test]
    fn assert_render_non_empty() {
        let config = build_config();
        let surface = Surface::new(1024, 1024, PixelFormatEnum::RGBA8888).unwrap();
        let mut canvas = CanvasMock::new();
        let mut renderer = RendererMock::new(config.clone(), surface);
        let token = TokenType::String {
            token: Token::new("foo bar".to_string(), 0, 0, 0, 0),
        };
        let mut widget = EditorFileToken::new(&token, false, config.clone());
        widget.prepare_ui(&mut renderer);
        widget.render(&mut canvas, &mut renderer, &RenderContext::Nothing);
        let expected = CanvasMock::new();
        assert_eq!(canvas, expected);
    }

    #[test]
    fn assert_render_new_line() {
        let config = build_config();
        let surface = Surface::new(1024, 1024, PixelFormatEnum::RGBA8888).unwrap();
        let mut canvas = CanvasMock::new();
        let mut renderer = RendererMock::new(config.clone(), surface);
        let token = TokenType::Whitespace {
            token: Token::new("\n".to_string(), 0, 0, 0, 0),
        };
        let mut widget = EditorFileToken::new(&token, false, config.clone());
        widget.prepare_ui(&mut renderer);
        widget.render(&mut canvas, &mut renderer, &RenderContext::Nothing);
        let expected = CanvasMock::new();
        assert_eq!(canvas, expected);
    }
}
