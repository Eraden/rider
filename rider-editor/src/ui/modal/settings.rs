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
}

impl Settings {
    pub fn new(config: ConfigAccess) -> Self {
        //        let (window_width, window_height, background_color, border_color) = {
        let c = config
            .read()
            .unwrap_or_else(|_| panic!("Failed to read config"));
        let theme = c.theme();
        let window_width = c.width();
        let window_height = c.height();
        let background_color = theme.background().into();
        let border_color = theme.border_color().into();
        //        };
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
            font_value: Label::new(c.editor_config().font_path().clone(), config.clone()),
            config: config.clone(),
        }
    }

    pub fn full_rect(&self) -> &Rect {
        &self.full_dest
    }

    pub fn scroll_by(&mut self, x: i32, y: i32) {
        let read_config = self.config.read().unwrap();

        let value_x = read_config.scroll().speed() * x;
        let value_y = read_config.scroll().speed() * y;
        let old_x = self.horizontal_scroll_bar.scroll_value();
        let old_y = self.vertical_scroll_bar.scroll_value();

        if value_x + old_x >= 0 {
            self.horizontal_scroll_bar.scroll_to(value_x + old_x);
            if self.horizontal_scroll_bar.scrolled_part() > 1.0 {
                self.horizontal_scroll_bar.scroll_to(old_x);
            }
        }
        if value_y + old_y >= 0 {
            self.vertical_scroll_bar.scroll_to(value_y + old_y);
            if self.vertical_scroll_bar.scrolled_part() > 1.0 {
                self.vertical_scroll_bar.scroll_to(old_y);
            }
        }
    }

    pub fn scroll(&self) -> Point {
        Point::new(
            -self.horizontal_scroll_bar.scroll_value(),
            -self.vertical_scroll_bar.scroll_value(),
        )
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
                    + Point::new(
                        (CONTENT_MARGIN_LEFT * 2) + self.font_label.name_width() as i32,
                        CONTENT_MARGIN_TOP,
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

    pub fn prepare_ui<R>(&mut self, renderer: &mut R)
    where
        R: Renderer + CharacterSizeManager,
    {
        self.font_label.prepare_ui(renderer);
        self.font_value.prepare_ui(renderer);
    }
}

impl ClickHandler for Settings {
    fn on_left_click(&mut self, _point: &Point, context: &UpdateContext) -> UR {
        let dest = match context {
            UC::ParentPosition(p) => move_render_point(*p, &self.dest),
            _ => self.dest,
        };
        let _context = UC::ParentPosition(
            dest.top_left() + Point::new(CONTENT_MARGIN_LEFT, CONTENT_MARGIN_TOP) + self.scroll(),
        );
        //        let res = self.directory_view.on_left_click(point, &context);
        //        {
        //            let dest = self.directory_view.dest();
        //            let full_dest = Rect::new(
        //                dest.x(),
        //                dest.y(),
        //                dest.width() + (2 * CONTENT_MARGIN_LEFT as u32),
        //                dest.height() + (2 * CONTENT_MARGIN_TOP as u32),
        //            );
        //            self.full_dest = full_dest;
        //        }
        //        res
        UR::NoOp
    }

    fn is_left_click_target(&self, _point: &Point, context: &UpdateContext) -> bool {
        let dest = match *context {
            UC::ParentPosition(p) => move_render_point(p.clone(), &self.dest),
            _ => self.dest.clone(),
        };
        let p =
            dest.top_left() + Point::new(CONTENT_MARGIN_LEFT, CONTENT_MARGIN_TOP) + self.scroll();
        let _context = UC::ParentPosition(p);
        //        if self.directory_view.is_left_click_target(point, &context) {
        //            true
        //        } else {
        //            Rect::new(p.x(), p.y(), dest.width(), dest.height()).contains_point(point.clone())
        //        }
        false
    }
}

impl RenderBox for Settings {
    fn render_start_point(&self) -> Point {
        self.dest.top_left()
    }

    fn dest(&self) -> Rect {
        self.dest.clone()
    }
}

impl Update for Settings {
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
}
