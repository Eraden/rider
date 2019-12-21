use crate::app::UpdateResult as UR;
use crate::renderer::renderer::Renderer;
use crate::ui::*;
use crate::ui::{RenderContext as RC, UpdateContext as UC};
use rider_config::ConfigAccess;
use rider_config::ConfigHolder;
use sdl2::pixels::Color;
use sdl2::rect::{Point, Rect};
use std::sync::Arc;

const CONTENT_MARGIN_LEFT: i32 = 16;
const CONTENT_MARGIN_TOP: i32 = 24;
const DEFAULT_ICON_SIZE: u32 = 16;

pub struct OpenFile {
    root_path: String,
    directory_view: DirectoryView,
    vertical_scroll_bar: VerticalScrollBar,
    horizontal_scroll_bar: HorizontalScrollBar,
    dest: Rect,
    full_dest: Rect,
    background_color: Color,
    border_color: Color,
    config: ConfigAccess,
}

impl ScrollView<VerticalScrollBar, HorizontalScrollBar> for OpenFile {
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

impl ConfigHolder for OpenFile {
    fn config(&self) -> &ConfigAccess {
        &self.config
    }
}

impl OpenFile {
    pub fn new(root_path: String, width: u32, height: u32, config: ConfigAccess) -> Self {
        let (window_width, window_height, background_color, border_color) = {
            let c = config.read().unwrap();
            let theme = c.theme();
            (
                c.width(),
                c.height(),
                theme.background().into(),
                theme.border_color().into(),
            )
        };
        Self {
            directory_view: DirectoryView::new(root_path.clone(), Arc::clone(&config)),
            vertical_scroll_bar: VerticalScrollBar::new(Arc::clone(&config)),
            horizontal_scroll_bar: HorizontalScrollBar::new(Arc::clone(&config)),
            root_path,
            dest: Rect::new(
                (window_width / 2) as i32 - (width / 2) as i32,
                (window_height / 2) as i32 - (height / 2) as i32,
                width,
                height,
            ),
            full_dest: Rect::new(0, 0, DEFAULT_ICON_SIZE, DEFAULT_ICON_SIZE),
            background_color,
            border_color,
            config,
        }
    }

    pub fn root_path(&self) -> String {
        self.root_path.clone()
    }

    pub fn open_directory<R>(&mut self, dir_path: String, renderer: &mut R)
    where
        R: Renderer + CharacterSizeManager,
    {
        self.directory_view.open_directory(dir_path, renderer);
        {
            let dest = self.directory_view.dest();
            let full_dest = Rect::new(
                dest.x(),
                dest.y(),
                dest.width() + (2 * CONTENT_MARGIN_LEFT as u32),
                dest.height() + (2 * CONTENT_MARGIN_TOP as u32),
            );
            self.full_dest = full_dest;
        }
    }

    pub fn full_rect(&self) -> &Rect {
        &self.full_dest
    }

    pub fn update(&mut self, ticks: i32, context: &UC) -> UR {
        let (window_width, window_height, color, scroll_width, scroll_margin) = {
            let c = self.config.read().unwrap();
            (
                c.width(),
                c.height(),
                c.theme().background().into(),
                c.scroll().width(),
                c.scroll().margin_right(),
            )
        };

        self.dest
            .set_x((window_width / 2) as i32 - (self.dest.width() / 2) as i32);
        self.dest
            .set_y((window_height / 2) as i32 - (self.dest.height() / 2) as i32);

        self.background_color = color;

        //        Scroll bars
        self.vertical_scroll_bar
            .set_full_size(self.full_dest.height()); // full dest
        self.vertical_scroll_bar.set_viewport(self.dest.height());
        self.vertical_scroll_bar
            .set_location(self.dest.width() as i32 - (scroll_width as i32 + scroll_margin));
        self.vertical_scroll_bar.update(ticks, context);

        self.horizontal_scroll_bar
            .set_full_size(self.full_dest.width()); // full dest
        self.horizontal_scroll_bar.set_viewport(self.dest.width());
        self.horizontal_scroll_bar
            .set_location(self.dest.height() as i32 - (scroll_width as i32 + scroll_margin));
        self.horizontal_scroll_bar.update(ticks, context);

        // End
        UR::NoOp
    }

    pub fn render<C, R>(&self, canvas: &mut C, renderer: &mut R, context: &RC)
    where
        C: CanvasAccess,
        R: Renderer + CharacterSizeManager + ConfigHolder,
    {
        let dest = match context {
            RC::ParentPosition(p) => move_render_point(p.clone(), &self.dest),
            _ => self.dest.clone(),
        };

        // Background
        canvas.set_clipping(dest.clone());
        canvas
            .render_rect(dest, self.background_color)
            .unwrap_or_else(|_| panic!("Failed to render open file modal background!"));
        canvas
            .render_border(dest, self.border_color)
            .unwrap_or_else(|_| panic!("Failed to render open file modal border!"));

        let context = RC::ParentPosition(
            dest.top_left() + Point::new(CONTENT_MARGIN_LEFT, CONTENT_MARGIN_TOP) + self.scroll(),
        );

        // directory tree
        self.directory_view.render(canvas, renderer, &context);

        // Scroll bars
        self.vertical_scroll_bar
            .render(canvas, &RenderContext::ParentPosition(self.dest.top_left()));
        self.horizontal_scroll_bar
            .render(canvas, &RenderContext::ParentPosition(self.dest.top_left()));
    }

    pub fn prepare_ui<R>(&mut self, renderer: &mut R)
    where
        R: Renderer + CharacterSizeManager,
    {
        self.directory_view.prepare_ui(renderer);
    }

    pub fn render_start_point(&self) -> Point {
        self.dest.top_left()
    }

    pub fn dest(&self) -> Rect {
        self.dest.clone()
    }

    pub fn on_left_click(&mut self, point: &Point, context: &UC) -> UR {
        let dest = match context {
            UC::ParentPosition(p) => move_render_point(*p, &self.dest),
            _ => self.dest,
        };
        let context = UC::ParentPosition(
            dest.top_left() + Point::new(CONTENT_MARGIN_LEFT, CONTENT_MARGIN_TOP) + self.scroll(),
        );
        let res = self.directory_view.on_left_click(point, &context);
        {
            let dest = self.directory_view.dest();
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

    pub fn is_left_click_target(&self, point: &Point, context: &UC) -> bool {
        let dest = match context {
            UC::ParentPosition(p) => move_render_point(p.clone(), &self.dest()),
            _ => self.dest().clone(),
        };
        let p =
            dest.top_left() + Point::new(CONTENT_MARGIN_LEFT, CONTENT_MARGIN_TOP) + self.scroll();
        let context = UC::ParentPosition(p);
        if self.directory_view.is_left_click_target(point, &context) {
            true
        } else {
            Rect::new(p.x(), p.y(), dest.width(), dest.height()).contains_point(point.clone())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::support::SimpleRendererMock;
    use crate::tests::support::{build_config, CanvasMock};
    use std::fs;

    //#######################################################################
    // scroll
    //#######################################################################

    #[test]
    fn assert_scroll() {
        let config = build_config();
        let mut widget = OpenFile::new("/tmp".to_owned(), 100, 100, config);
        widget.scroll_by(12, 13);
        assert_eq!(widget.scroll(), Point::new(-360, -390));
    }

    //#######################################################################
    // dest
    //#######################################################################

    #[test]
    fn assert_dest() {
        let config = build_config();
        let widget = OpenFile::new("/tmp".to_owned(), 120, 130, config);
        assert_eq!(widget.dest(), Rect::new(452, 365, 120, 130));
    }

    //#######################################################################
    // full_rect
    //#######################################################################

    #[test]
    fn assert_full_rect() {
        let config = build_config();
        let widget = OpenFile::new("/tmp".to_owned(), 120, 130, config);
        assert_eq!(widget.full_rect(), &Rect::new(0, 0, 16, 16));
    }

    //#######################################################################
    // open_directory
    //#######################################################################

    #[test]
    fn assert_open_directory() {
        let path = "/tmp/rider/test-open-file/open-directory";
        fs::create_dir_all(path).unwrap();
        let config = build_config();
        let mut renderer = SimpleRendererMock::new(config);
        let mut widget = OpenFile::new(path.to_owned(), 120, 130, renderer.config().clone());
        widget.open_directory(path.to_owned(), &mut renderer);
    }

    //#######################################################################
    // update
    //#######################################################################

    #[test]
    fn assert_update() {
        let config = build_config();
        let mut widget = OpenFile::new("/tmp".to_owned(), 100, 100, config);
        widget.update(0, &UpdateContext::Nothing);
    }

    //#######################################################################
    // root_path
    //#######################################################################

    #[test]
    fn assert_root_path() {
        let config = build_config();
        let path = "/tmp/rider/test-open-file/open-directory";
        let mut widget = OpenFile::new(path.to_owned(), 100, 100, config);
        widget.update(0, &UpdateContext::Nothing);
        assert_eq!(widget.root_path(), path.to_owned());
    }

    //#######################################################################
    // render_start_point
    //#######################################################################

    #[test]
    fn assert_render_start_point() {
        let config = build_config();
        let path = "/tmp/rider/test-open-file/open-directory";
        let mut widget = OpenFile::new(path.to_owned(), 100, 100, config);
        widget.update(0, &UpdateContext::Nothing);
        assert_eq!(widget.render_start_point(), Point::new(462, 380));
    }

    //#######################################################################
    // on_left_click
    //#######################################################################

    #[test]
    fn assert_on_left_click_with_nothing() {
        let config = build_config();
        let path = "/tmp/rider/test-open-file/open-directory";
        let mut widget = OpenFile::new(path.to_owned(), 100, 100, config);
        let p = Point::new(100, 100);
        let context = UpdateContext::Nothing;
        widget.on_left_click(&p, &context);
    }

    #[test]
    fn assert_on_left_click_with_parent_position() {
        let config = build_config();
        let path = "/tmp/rider/test-open-file/open-directory";
        let mut widget = OpenFile::new(path.to_owned(), 100, 100, config);
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
        let widget = OpenFile::new(path.to_owned(), 100, 100, config);
        let p = Point::new(100, 100);
        let context = UpdateContext::Nothing;
        assert_eq!(widget.is_left_click_target(&p, &context), false);
    }

    #[test]
    fn assert_is_left_click_target_with_parent_position() {
        let config = build_config();
        let path = "/tmp/rider/test-open-file/open-directory";
        let widget = OpenFile::new(path.to_owned(), 100, 100, config);
        let p = Point::new(100, 100);
        let context = UpdateContext::ParentPosition(Point::new(10, 10));
        assert_eq!(widget.is_left_click_target(&p, &context), false);
    }

    #[test]
    fn assert_is_left_click_target_with_parent_position_in_box() {
        let config = build_config();
        let path = "/tmp/rider/test-open-file/open-directory";
        let widget = OpenFile::new(path.to_owned(), 100, 100, config);
        let p = Point::new(500, 400);
        let context = UpdateContext::ParentPosition(Point::new(10, 10));
        assert_eq!(widget.is_left_click_target(&p, &context), false);
    }

    //#######################################################################
    // render
    //#######################################################################

    #[test]
    fn assert_render() {
        let config = build_config();
        let path = "/tmp/rider/test-open-file/open-directory";
        let mut renderer = SimpleRendererMock::new(config.clone());
        let mut canvas = CanvasMock::new();
        let widget = OpenFile::new(path.to_owned(), 100, 100, config);
        let p = Point::new(100, 100);
        let context = RenderContext::ParentPosition(p);
        widget.render(&mut canvas, &mut renderer, &context);
    }

    //#######################################################################
    // prepare_ui
    //#######################################################################

    #[test]
    fn assert_prepare_ui() {
        let config = build_config();
        let path = "/tmp/rider/test-open-file/open-directory";
        let mut renderer = SimpleRendererMock::new(config.clone());
        let mut widget = OpenFile::new(path.to_owned(), 100, 100, config);
        widget.prepare_ui(&mut renderer);
    }
}
