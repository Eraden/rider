use crate::renderer::renderer::Renderer;
use crate::ui::filesystem::directory::DirectoryView;
use crate::ui::text_character::CharacterSizeManager;
use crate::ui::CanvasAccess;
use crate::ui::RenderContext;
use crate::ui::UpdateContext;
use rider_config::config::Config;
use sdl2::pixels::Color;
use sdl2::rect::Point;
use sdl2::rect::Rect;
use std::sync::Arc;
use std::sync::RwLock;

pub struct ProjectTreeSidebar {
    dest: Rect,
    config: Arc<RwLock<Config>>,
    root: String,
    border_color: Color,
    background_color: Color,
    dir_view: DirectoryView,
}

impl ProjectTreeSidebar {
    pub fn new(root: String, config: Arc<RwLock<Config>>) -> Self {
        let (background_color, border_color, h): (Color, Color, u32) = {
            let c = config.read().unwrap();
            (
                c.theme().background().into(),
                c.theme().border_color().into(),
                c.height(),
            )
        };

        Self {
            dest: Rect::new(0, 0, 100, h),
            dir_view: DirectoryView::new(root.clone(), config.clone()),
            config,
            root,
            background_color,
            border_color,
        }
    }

    pub fn update(&mut self, ticks: i32) {
        let config = self.config.read().unwrap();
        let height = config.height();
        //        let left_margin = config.editor_left_margin();
        let top_margin = config.menu_height() as i32;
        //        self.dest.set_x(left_margin);
        self.dest.set_y(top_margin);
        self.dest.set_height(height - top_margin as u32);
        self.dir_view.update(ticks, &UpdateContext::Nothing);
    }

    pub fn prepare_ui<R>(&mut self, renderer: &mut R)
    where
        R: Renderer + CharacterSizeManager,
    {
        let config = self.config.read().unwrap();
        let height = config.height();
        let left_margin = config.editor_left_margin();
        let top_margin = config.menu_height() as i32;
        self.dest.set_x(left_margin);
        self.dest.set_y(top_margin);
        self.dest.set_height(height);
        self.dir_view.prepare_ui(renderer);
        self.dir_view.open_directory(self.root.clone(), renderer);
    }

    pub fn render<C, R>(&self, canvas: &mut C, renderer: &mut R)
    where
        R: Renderer + CharacterSizeManager,
        C: CanvasAccess,
    {
        canvas.set_clipping(self.dest.clone());
        canvas
            .render_rect(self.dest.clone(), self.background_color.clone())
            .unwrap();
        canvas
            .render_border(self.dest.clone(), self.border_color.clone())
            .unwrap();

        // dir view
        let context = RenderContext::RelativePosition(self.dest.top_left() + Point::new(10, 10));
        self.dir_view.render(canvas, renderer, &context);
    }

    pub fn full_rect(&self) -> Rect {
        self.dest.clone()
    }

    pub fn root(&self) -> String {
        self.root.clone()
    }
}

#[cfg(test)]
mod tests {
    use crate::renderer::managers::FontDetails;
    use crate::renderer::managers::TextDetails;
    use crate::renderer::renderer::Renderer;
    use crate::tests::support::build_config;
    use crate::tests::support::CanvasMock;
    use crate::ui::project_tree::ProjectTreeSidebar;
    use crate::ui::text_character::CharacterSizeManager;
    use rider_config::ConfigAccess;
    use rider_config::ConfigHolder;
    use sdl2::rect::Rect;
    use sdl2::render::Texture;
    use sdl2::ttf::Font;
    use std::rc::Rc;

    #[cfg_attr(tarpaulin, skip)]
    struct RendererMock {
        config: ConfigAccess,
    }

    #[cfg_attr(tarpaulin, skip)]
    impl RendererMock {
        pub fn new(config: ConfigAccess) -> Self {
            Self { config }
        }
    }

    #[cfg_attr(tarpaulin, skip)]
    impl Renderer for RendererMock {
        fn load_font(&mut self, _details: FontDetails) -> Rc<Font> {
            unimplemented!()
        }

        fn load_text_tex(
            &mut self,
            _details: &mut TextDetails,
            _font_details: FontDetails,
        ) -> Result<Rc<Texture>, String> {
            Err("Skip load text texture".to_owned())
        }

        fn load_image(&mut self, _path: String) -> Result<Rc<Texture>, String> {
            Err("Skip render".to_owned())
        }
    }

    #[cfg_attr(tarpaulin, skip)]
    impl ConfigHolder for RendererMock {
        fn config(&self) -> &ConfigAccess {
            &self.config
        }
    }

    #[cfg_attr(tarpaulin, skip)]
    impl CharacterSizeManager for RendererMock {
        fn load_character_size(&mut self, _c: char) -> Rect {
            Rect::new(0, 0, 13, 14)
        }
    }

    #[test]
    fn assert_full_rect() {
        let config = build_config();
        let mut renderer = RendererMock::new(config.clone());
        let mut widget = ProjectTreeSidebar::new("/tmp".to_owned(), config);
        widget.prepare_ui(&mut renderer);
        assert_eq!(widget.full_rect(), Rect::new(10, 60, 100, 860));
    }

    #[test]
    fn assert_update() {
        let config = build_config();
        let mut widget = ProjectTreeSidebar::new("/tmp".to_owned(), config);
        widget.update(0);
        assert_eq!(widget.full_rect(), Rect::new(0, 60, 100, 800));
    }

    #[test]
    fn assert_prepare_ui() {
        let config = build_config();
        let mut renderer = RendererMock::new(config.clone());
        let mut widget = ProjectTreeSidebar::new("/tmp".to_owned(), config);
        widget.prepare_ui(&mut renderer);
        assert_eq!(widget.full_rect(), Rect::new(10, 60, 100, 860));
    }

    #[test]
    fn assert_render() {
        let config = build_config();
        let mut renderer = RendererMock::new(config.clone());
        let mut canvas = CanvasMock::new();
        let widget = ProjectTreeSidebar::new("/tmp".to_owned(), config);
        //        widget.prepare_ui(&mut renderer); // skip load directory
        widget.render(&mut canvas, &mut renderer);
    }
    /*let pwd = env::current_dir().unwrap().to_str().unwrap().to_string();*/
}
