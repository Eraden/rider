use crate::app::application::UpdateResult;
use crate::renderer::renderer::Renderer;
use crate::ui::filesystem::directory::DirectoryView;
use crate::ui::horizontal_scroll_bar::HorizontalScrollBar;
use crate::ui::text_character::CharacterSizeManager;
use crate::ui::vertical_scroll_bar::VerticalScrollBar;
use crate::ui::CanvasAccess;
use crate::ui::ClickHandler;
use crate::ui::RenderContext;
use crate::ui::UpdateContext;
use crate::ui::{move_render_point, ScrollView};
use rider_config::config::Config;
use rider_config::ConfigHolder;
use sdl2::pixels::Color;
use sdl2::rect::Point;
use sdl2::rect::Rect;
use std::sync::Arc;
use std::sync::RwLock;

const CONTENT_MARGIN_LEFT: i32 = 16;
const CONTENT_MARGIN_TOP: i32 = 24;
const DEFAULT_ICON_SIZE: u32 = 16;

pub struct ProjectTreeSidebar {
    dest: Rect,
    full_dest: Rect,
    config: Arc<RwLock<Config>>,
    root: String,
    border_color: Color,
    background_color: Color,
    dir_view: DirectoryView,
    vertical_scroll_bar: VerticalScrollBar,
    horizontal_scroll_bar: HorizontalScrollBar,
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
            dest: Rect::new(0, 0, 200, h),
            full_dest: Rect::new(0, 0, DEFAULT_ICON_SIZE, DEFAULT_ICON_SIZE),
            dir_view: DirectoryView::new(root.clone(), config.clone()),
            vertical_scroll_bar: VerticalScrollBar::new(Arc::clone(&config)),
            horizontal_scroll_bar: HorizontalScrollBar::new(Arc::clone(&config)),
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
        let left_margin = 0;
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
        let context = RenderContext::ParentPosition(
            self.dest.top_left() + Point::new(CONTENT_MARGIN_LEFT, CONTENT_MARGIN_TOP),
        );
        self.dir_view.render(canvas, renderer, &context);
    }

    pub fn full_rect(&self) -> Rect {
        self.dest.clone()
    }

    pub fn root(&self) -> String {
        self.root.clone()
    }

    pub fn open_directory<R>(&mut self, dir_path: String, renderer: &mut R)
    where
        R: Renderer + CharacterSizeManager,
    {
        self.dir_view.open_directory(dir_path, renderer);
        {
            let dest = self.dir_view.dest();
            let full_dest = Rect::new(
                dest.x(),
                dest.y(),
                dest.width() + (2 * CONTENT_MARGIN_LEFT as u32),
                dest.height() + (2 * CONTENT_MARGIN_TOP as u32),
            );
            self.full_dest = full_dest;
        }
    }
}

impl ClickHandler for ProjectTreeSidebar {
    fn on_left_click(&mut self, point: &Point, context: &UpdateContext) -> UpdateResult {
        let dest = match context {
            UpdateContext::ParentPosition(p) => move_render_point(*p, &self.dest),
            _ => self.dest,
        };
        let context = UpdateContext::ParentPosition(
            dest.top_left() + Point::new(CONTENT_MARGIN_LEFT, CONTENT_MARGIN_TOP) + self.scroll(),
        );
        let res = self.dir_view.on_left_click(point, &context);
        {
            let dest = self.dir_view.dest();
            let full_dest = Rect::new(
                dest.x(),
                dest.y(),
                dest.width() + (2 * CONTENT_MARGIN_LEFT as u32),
                dest.height() + (2 * CONTENT_MARGIN_TOP as u32),
            );
            self.full_dest = full_dest;
        }
        res
    }

    fn is_left_click_target(&self, point: &Point, context: &UpdateContext) -> bool {
        let dest = match context {
            UpdateContext::ParentPosition(p) => move_render_point(p.clone(), &self.dest),
            _ => self.dest.clone(),
        };
        let p =
            dest.top_left() + Point::new(CONTENT_MARGIN_LEFT, CONTENT_MARGIN_TOP) + self.scroll();
        let context = UpdateContext::ParentPosition(p);
        if self.dir_view.is_left_click_target(point, &context) {
            true
        } else {
            Rect::new(p.x(), p.y(), dest.width(), dest.height()).contains_point(point.clone())
        }
    }
}

impl ConfigHolder for ProjectTreeSidebar {
    fn config(&self) -> &Arc<RwLock<Config>> {
        &self.config
    }
}

impl ScrollView<VerticalScrollBar, HorizontalScrollBar> for ProjectTreeSidebar {
    fn mut_horizontal_scroll_handler(&mut self) -> Option<&mut HorizontalScrollBar> {
        Some(&mut self.horizontal_scroll_bar)
    }

    fn horizontal_scroll_handler(&self) -> Option<&HorizontalScrollBar> {
        Some(&self.horizontal_scroll_bar)
    }

    fn mut_vertical_scroll_handler(&mut self) -> Option<&mut VerticalScrollBar> {
        Some(&mut self.vertical_scroll_bar)
    }

    fn vertical_scroll_handler(&self) -> Option<&VerticalScrollBar> {
        Some(&self.vertical_scroll_bar)
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
    use crate::ui::scroll_bar::ScrollView;
    use crate::ui::text_character::CharacterSizeManager;
    use crate::ui::ClickHandler;
    use crate::ui::UpdateContext;
    use rider_config::ConfigAccess;
    use rider_config::ConfigHolder;
    use sdl2::rect::Point;
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
        assert_eq!(widget.full_rect(), Rect::new(0, 40, 200, 860));
    }

    #[test]
    fn assert_update() {
        let config = build_config();
        let mut widget = ProjectTreeSidebar::new("/tmp".to_owned(), config);
        widget.update(0);
        assert_eq!(widget.full_rect(), Rect::new(0, 40, 200, 820));
    }

    #[test]
    fn assert_prepare_ui() {
        let config = build_config();
        let mut renderer = RendererMock::new(config.clone());
        let mut widget = ProjectTreeSidebar::new("/tmp".to_owned(), config);
        widget.prepare_ui(&mut renderer);
        assert_eq!(widget.full_rect(), Rect::new(0, 40, 200, 860));
    }

    #[test]
    fn assert_render() {
        let config = build_config();
        let mut renderer = RendererMock::new(config.clone());
        let mut canvas = CanvasMock::new();
        let widget = ProjectTreeSidebar::new("/tmp".to_owned(), config);
        widget.render(&mut canvas, &mut renderer);
    }

    //#######################################################################
    // scroll
    //#######################################################################

    #[test]
    fn assert_scroll() {
        let config = build_config();
        let widget = ProjectTreeSidebar::new("/tmp".to_owned(), config);
        let res = widget.scroll();
        let expected = Point::new(0, 0);
        assert_eq!(res, expected);
    }

    #[test]
    fn assert_scroll_by() {
        let config = build_config();
        let mut widget = ProjectTreeSidebar::new("/tmp".to_owned(), config);
        widget.scroll_by(10, 10);
        let res = widget.scroll();
        let expected = Point::new(0, -300);
        assert_eq!(res, expected);
    }

    //#######################################################################
    // on_left_click
    //#######################################################################

    #[test]
    fn assert_on_left_click_with_nothing() {
        let config = build_config();
        let path = "/tmp/rider/test-open-file/open-directory";
        let mut widget = ProjectTreeSidebar::new(path.to_owned(), config);
        let p = Point::new(100, 100);
        let context = UpdateContext::Nothing;
        widget.on_left_click(&p, &context);
    }

    #[test]
    fn assert_on_left_click_with_parent_position() {
        let config = build_config();
        let path = "/tmp/rider/test-open-file/open-directory";
        let mut widget = ProjectTreeSidebar::new(path.to_owned(), config);
        let p = Point::new(100, 100);
        let context = UpdateContext::ParentPosition(Point::new(10, 10));
        widget.on_left_click(&p, &context);
    }

    //#######################################################################
    // is_left_click_target
    //#######################################################################

    #[test]
    fn assert_is_left_click_target_with_nothing() {
        let config = build_config();
        let path = "/tmp/rider/test-open-file/open-directory";
        let widget = ProjectTreeSidebar::new(path.to_owned(), config);
        let p = Point::new(400, 400);
        let context = UpdateContext::Nothing;
        assert_eq!(widget.is_left_click_target(&p, &context), false);
    }

    #[test]
    fn assert_is_left_click_target_with_parent_position() {
        let config = build_config();
        let path = "/tmp/rider/test-open-file/open-directory";
        let widget = ProjectTreeSidebar::new(path.to_owned(), config);
        let p = Point::new(800, 800);
        let context = UpdateContext::ParentPosition(Point::new(10, 10));
        assert_eq!(widget.is_left_click_target(&p, &context), false);
    }

    #[test]
    fn assert_is_left_click_target_with_parent_position_in_box() {
        let config = build_config();
        let path = "/tmp/rider/test-open-file/open-directory";
        let widget = ProjectTreeSidebar::new(path.to_owned(), config);
        let p = Point::new(500, 400);
        let context = UpdateContext::ParentPosition(Point::new(10, 10));
        assert_eq!(widget.is_left_click_target(&p, &context), false);
    }
}
