use crate::app::UpdateResult as UR;
use crate::renderer::renderer::Renderer;
use crate::ui::RenderContext as RC;
use crate::ui::*;
use rider_config::ConfigAccess;
use rider_config::ConfigHolder;
use sdl2::pixels::Color;
use sdl2::rect::{Point, Rect};
use std::sync::Arc;

const CONTENT_MARGIN_LEFT: i32 = 16;
const CONTENT_MARGIN_TOP: i32 = 24;
const DEFAULT_ICON_SIZE: u32 = 16;
const LABEL_WIDTH: i32 = CONTENT_MARGIN_LEFT * 20;

pub struct Settings {
    vertical_scroll_bar: VerticalScrollBar,
    horizontal_scroll_bar: HorizontalScrollBar,
    dest: Rect,
    full_dest: Rect,
    background_color: Color,
    border_color: Color,
    config: ConfigAccess,
    font_label: Label,
    font_value: Label,
    character_size_label: Label,
    character_size_value: Label,
}

impl Widget for Settings {
    fn texture_path(&self) -> Option<String> {
        None
    }

    fn dest(&self) -> &Rect {
        &self.dest
    }

    fn set_dest(&mut self, _rect: &Rect) {}

    fn source(&self) -> &Rect {
        &self.dest
    }

    fn set_source(&mut self, _rect: &Rect) {}

    fn update(&mut self, ticks: i32, context: &UpdateContext) -> UR {
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

        self.dest.set_x(CONTENT_MARGIN_LEFT);
        self.dest
            .set_width(window_width - (CONTENT_MARGIN_LEFT * 2) as u32);
        self.dest.set_y(CONTENT_MARGIN_TOP);
        self.dest
            .set_height(window_height - (CONTENT_MARGIN_TOP * 2) as u32);

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

    #[inline]
    fn on_left_click(&mut self, _point: &Point, _context: &UpdateContext) -> UR {
        UR::NoOp
    }

    #[inline]
    fn is_left_click_target(&self, _point: &Point, _context: &UpdateContext) -> bool {
        false
    }

    fn render<C, R>(&self, canvas: &mut C, renderer: &mut R, context: &RC)
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

        // font path
        self.font_label.render(
            canvas,
            renderer,
            &RC::ParentPosition(
                self.render_start_point()
                    + Point::new(CONTENT_MARGIN_LEFT, CONTENT_MARGIN_TOP)
                    + self.scroll(),
            ),
        );
        self.font_value.render(
            canvas,
            renderer,
            &RC::ParentPosition(
                self.render_start_point()
                    + Point::new(CONTENT_MARGIN_LEFT + LABEL_WIDTH, CONTENT_MARGIN_TOP)
                    + self.scroll(),
            ),
        );

        // character size
        self.character_size_label.render(
            canvas,
            renderer,
            &RC::ParentPosition(
                self.render_start_point()
                    + Point::new(
                        CONTENT_MARGIN_LEFT,
                        CONTENT_MARGIN_TOP + self.font_label.dest().height() as i32,
                    )
                    + self.scroll(),
            ),
        );
        self.character_size_value.render(
            canvas,
            renderer,
            &RC::ParentPosition(
                self.render_start_point()
                    + Point::new(
                        CONTENT_MARGIN_LEFT + LABEL_WIDTH,
                        CONTENT_MARGIN_TOP + self.font_label.dest().height() as i32,
                    )
                    + self.scroll(),
            ),
        );

        // Scroll bars
        self.vertical_scroll_bar
            .render(canvas, &RenderContext::ParentPosition(self.dest.top_left()));
        self.horizontal_scroll_bar
            .render(canvas, &RenderContext::ParentPosition(self.dest.top_left()));
    }

    fn prepare_ui<R>(&mut self, renderer: &mut R)
    where
        R: Renderer + CharacterSizeManager + ConfigHolder,
    {
        self.font_label.prepare_ui(renderer);
        self.font_value.prepare_ui(renderer);
        self.character_size_label.prepare_ui(renderer);
        self.character_size_value.prepare_ui(renderer);
    }
}

impl ScrollView<VerticalScrollBar, HorizontalScrollBar> for Settings {
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

impl Settings {
    pub fn new(config: ConfigAccess) -> Self {
        let c = config
            .read()
            .unwrap_or_else(|_| panic!("Failed to read config"));
        let theme = c.theme();
        let window_width = c.width();
        let window_height = c.height();
        let background_color = theme.background().into();
        let border_color = theme.border_color().into();
        Self {
            vertical_scroll_bar: VerticalScrollBar::new(Arc::clone(&config)),
            horizontal_scroll_bar: HorizontalScrollBar::new(Arc::clone(&config)),
            dest: Rect::new(
                CONTENT_MARGIN_LEFT,
                CONTENT_MARGIN_TOP,
                window_width - (CONTENT_MARGIN_LEFT * 2) as u32,
                window_height - (CONTENT_MARGIN_TOP * 2) as u32,
            ),
            full_dest: Rect::new(0, 0, DEFAULT_ICON_SIZE, DEFAULT_ICON_SIZE),
            background_color,
            border_color,
            font_label: Label::new("Font path".into(), config.clone()),
            font_value: Label::new(c.editor_config().font_path().to_string(), config.clone()),
            character_size_label: Label::new("Character size".into(), config.clone()),
            character_size_value: Label::new(
                format!("{}", c.editor_config().character_size()).to_owned(),
                config.clone(),
            ),
            config: config.clone(),
        }
    }

    pub fn full_rect(&self) -> &Rect {
        &self.full_dest
    }
}

impl ConfigHolder for Settings {
    fn config(&self) -> &ConfigAccess {
        &self.config
    }
}

#[cfg(test)]
mod tests {
    use crate::app::UpdateResult;
    use crate::tests::*;
    use crate::ui::{RenderContext, ScrollView, Settings, UpdateContext, Widget};
    use rider_derive::*;
    use sdl2::rect::{Point, Rect};

    #[test]
    fn must_have_vertical_scrollbar() {
        let config = build_config();
        let mut widget = Settings::new(config);
        assert_eq!(widget.mut_vertical_scroll_handler().is_some(), true);
        assert_eq!(widget.vertical_scroll_handler().is_some(), true);
    }

    #[test]
    fn must_have_horizontal_scrollbar() {
        let config = build_config();
        let mut widget = Settings::new(config);
        assert_eq!(widget.mut_horizontal_scroll_handler().is_some(), true);
        assert_eq!(widget.horizontal_scroll_handler().is_some(), true);
    }

    // Widget

    #[test]
    fn assert_texture_path() {
        let config = build_config();
        let widget = Settings::new(config);
        let result = widget.texture_path();
        assert!(result.is_none());
    }

    #[test]
    fn assert_dest() {
        let config = build_config();
        let widget = Settings::new(config);
        let result = widget.dest().clone();
        let expected = Rect::new(16, 24, 992, 812);
        assert_eq!(result, expected);
    }

    #[test]
    fn assert_set_dest() {
        let config = build_config();
        let mut widget = Settings::new(config);
        widget.set_dest(&Rect::new(100, 200, 300, 400));
        assert_ne!(widget.dest(), &Rect::new(100, 200, 300, 400));
    }
    #[test]
    fn assert_source() {
        let config = build_config();
        let widget = Settings::new(config);
        let result = widget.source();
        assert_eq!(result, &Rect::new(16, 24, 992, 812));
    }
    #[test]
    fn assert_set_source() {
        let config = build_config();
        let mut widget = Settings::new(config);
        widget.set_source(&Rect::new(1, 2, 3, 4));
        assert_ne!(widget.source(), &Rect::new(1, 2, 3, 4));
    }

    #[test]
    fn assert_update() {
        let config = build_config();
        let mut widget = Settings::new(config);
        let result = widget.update(0, &UpdateContext::Nothing);
        assert_eq!(result, UpdateResult::NoOp);
    }

    #[test]
    fn assert_on_left_click() {
        let config = build_config();
        let mut widget = Settings::new(config);
        let result = widget.on_left_click(&Point::new(600, 800), &UpdateContext::Nothing);
        assert_eq!(result, UpdateResult::NoOp);
    }

    #[test]
    fn assert_is_left_click_target() {
        let config = build_config();
        let widget = Settings::new(config);
        let result = widget.is_left_click_target(&Point::new(600, 800), &UpdateContext::Nothing);
        assert_eq!(result, false);
    }

    #[test]
    fn assert_use_clipping() {
        let config = build_config();
        let widget = Settings::new(config);
        let result = widget.use_clipping();
        assert_eq!(result, false);
    }

    #[test]
    fn assert_render() {
        build_test_renderer!(renderer);
        let mut canvas = CanvasMock::new();
        let widget = Settings::new(config);
        let result = widget.render(&mut canvas, &mut renderer, &RenderContext::Nothing);
        assert_eq!(result, ());
    }
    #[test]
    fn assert_prepare_ui() {
        build_test_renderer!(renderer);
        let mut widget = Settings::new(config);
        let result = widget.prepare_ui(&mut renderer);
        assert_eq!(result, ());
    }
}
