use crate::app::UpdateResult as UR;
use crate::ui::*;
use rider_config::ConfigAccess;
use sdl2::pixels::Color;
use sdl2::rect::{Point, Rect};

pub struct MenuBar {
    border_color: Color,
    background_color: Color,
    dest: Rect,
    config: ConfigAccess,
}

impl MenuBar {
    pub fn new(config: ConfigAccess) -> Self {
        let (background_color, border_color, w, h): (Color, Color, u32, u16) = {
            let c = config.read().unwrap();
            (
                c.theme().background().into(),
                c.theme().border_color().into(),
                c.width(),
                c.menu_height(),
            )
        };
        Self {
            border_color,
            background_color,
            dest: Rect::new(0, 0, w as u32, h as u32),
            config,
        }
    }

    pub fn background_color(&self) -> &Color {
        &self.background_color
    }

    pub fn render<C>(&self, canvas: &mut C, context: &RenderContext)
    where
        C: CanvasAccess,
    {
        use std::borrow::*;

        canvas.set_clipping(self.dest.clone());
        canvas
            .render_rect(
                match context.borrow() {
                    RenderContext::RelativePosition(p) => move_render_point(p.clone(), &self.dest),
                    _ => self.dest(),
                },
                self.background_color.clone(),
            )
            .unwrap_or_else(|_| panic!("Failed to draw main menu background"));
        canvas
            .render_border(
                match context.borrow() {
                    RenderContext::RelativePosition(p) => {
                        move_render_point((*p).clone(), &self.dest)
                    }
                    _ => self.dest(),
                },
                self.border_color.clone(),
            )
            .unwrap_or_else(|_| panic!("Failed to draw main menu background"));
    }

    pub fn prepare_ui(&mut self) {
        let width = self.config.read().unwrap().width();
        let height = u32::from(self.config.read().unwrap().menu_height());
        self.dest = Rect::new(0, 0, width, height);
    }
}

impl Update for MenuBar {
    fn update(&mut self, _ticks: i32, _context: &UpdateContext) -> UR {
        let config = self.config.read().unwrap();
        self.dest.set_width(config.width());
        UR::NoOp
    }
}

impl ClickHandler for MenuBar {
    fn on_left_click(&mut self, _point: &Point, _context: &UpdateContext) -> UR {
        UR::NoOp
    }

    fn is_left_click_target(&self, point: &Point, context: &UpdateContext) -> bool {
        let rect = match *context {
            UpdateContext::ParentPosition(p) => move_render_point(p.clone(), &self.dest),
            _ => self.dest(),
        };
        rect.contains_point(point.clone())
    }
}

impl RenderBox for MenuBar {
    fn render_start_point(&self) -> Point {
        self.dest.top_left()
    }

    fn dest(&self) -> Rect {
        self.dest
    }
}

#[cfg(test)]
mod test_getters {
    use crate::tests::*;
    use crate::ui::*;
    use sdl2::pixels::Color;
    use sdl2::rect::Rect;
    use std::sync::*;

    #[test]
    fn assert_background_color() {
        let config = support::build_config();
        let widget = MenuBar::new(Arc::clone(&config));
        let result = widget.background_color().clone();
        let expected = Color::RGBA(18, 18, 18, 0);
        assert_eq!(result, expected);
    }

    #[test]
    fn assert_dest() {
        let config = support::build_config();
        let (w, h) = {
            let c = config.read().unwrap();
            (c.width() as u32, c.menu_height() as u32)
        };
        let widget = MenuBar::new(Arc::clone(&config));
        let result = widget.dest().clone();
        let expected = Rect::new(0, 0, w, h);
        assert_eq!(result, expected);
    }
}

#[cfg(test)]
mod test_render_box {
    use crate::tests::*;
    use crate::ui::*;
    use sdl2::rect::Point;
    use std::sync::*;

    #[test]
    fn must_return_top_left_point() {
        let config = support::build_config();
        let widget = MenuBar::new(Arc::clone(&config));
        let result = widget.render_start_point();
        let expected = Point::new(0, 0);
        assert_eq!(result, expected);
    }
}

#[cfg(test)]
mod test_click_handler {
    use crate::app::*;
    use crate::tests::*;
    use crate::ui::*;
    use sdl2::rect::Point;
    use std::sync::*;

    #[test]
    fn refute_when_not_click_target() {
        let config = support::build_config();
        let widget = MenuBar::new(Arc::clone(&config));
        let point = Point::new(9999, 9999);
        let context = UpdateContext::Nothing;
        let result = widget.is_left_click_target(&point, &context);
        assert_eq!(result, false);
    }

    #[test]
    fn assert_when_click_target() {
        let config = support::build_config();
        let widget = MenuBar::new(Arc::clone(&config));
        let point = Point::new(20, 30);
        let context = UpdateContext::Nothing;
        let result = widget.is_left_click_target(&point, &context);
        assert_eq!(result, true);
    }

    #[test]
    fn refute_when_not_click_target_because_parent() {
        let config = support::build_config();
        let widget = MenuBar::new(Arc::clone(&config));
        let point = Point::new(20, 30);
        let context = UpdateContext::ParentPosition(Point::new(9999, 9999));
        let result = widget.is_left_click_target(&point, &context);
        assert_eq!(result, false);
    }

    #[test]
    fn assert_when_click_target_because_parent() {
        let config = support::build_config();
        let (w, h) = {
            (
                config.read().unwrap().width(),
                config.read().unwrap().menu_height(),
            )
        };
        let widget = MenuBar::new(Arc::clone(&config));
        let point = Point::new(w as i32 + 120, h as i32 + 130);
        let context = UpdateContext::ParentPosition(Point::new(130, 140));
        let result = widget.is_left_click_target(&point, &context);
        assert_eq!(result, true);
    }

    #[test]
    fn assert_on_click_do_nothing() {
        let config = support::build_config();
        let mut widget = MenuBar::new(Arc::clone(&config));
        let point = Point::new(12, 34);
        let context = UpdateContext::ParentPosition(Point::new(678, 293));
        let result = widget.on_left_click(&point, &context);
        let expected = UpdateResult::NoOp;
        assert_eq!(result, expected);
    }
}

#[cfg(test)]
mod test_render {
    use crate::tests::*;
    use crate::ui::*;
    use sdl2::pixels::Color;
    use sdl2::rect::{Point, Rect};
    use sdl2::render::Texture;
    use std::rc::Rc;
    use std::sync::*;

    #[derive(Debug, PartialEq)]
    struct CanvasMock {
        pub clipping: Rect,
        pub background_rect: Rect,
        pub background_color: Color,
        pub border_rect: Rect,
        pub border_color: Color,
    }

    impl CanvasAccess for CanvasMock {
        fn render_rect(&mut self, rect: Rect, color: Color) -> Result<(), String> {
            self.background_color = color;
            self.background_rect = rect;
            Ok(())
        }

        fn render_border(&mut self, rect: Rect, color: Color) -> Result<(), String> {
            self.border_color = color;
            self.border_rect = rect;
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

        fn render_line(&mut self, _start: Point, _end: Point, _color: Color) -> Result<(), String> {
            unimplemented!()
        }

        fn set_clipping(&mut self, rect: Rect) {
            self.clipping = rect;
        }
    }

    #[test]
    fn assert_render() {
        let context = RenderContext::Nothing;
        let config = support::build_config();
        let mut canvas = CanvasMock {
            clipping: Rect::new(0, 0, 0, 0),
            background_rect: Rect::new(0, 0, 0, 0),
            background_color: Color::RGB(0, 0, 0),
            border_rect: Rect::new(0, 0, 0, 0),
            border_color: Color::RGB(0, 0, 0),
        };
        let mut widget = MenuBar::new(Arc::clone(&config));
        widget.prepare_ui();
        widget.render(&mut canvas, &context);
        assert_eq!(widget.dest(), Rect::new(0, 0, 1024, 60));
        let expected = CanvasMock {
            clipping: Rect::new(0, 0, 1024, 60),
            background_rect: Rect::new(0, 0, 1024, 60),
            background_color: Color::RGBA(18, 18, 18, 0),
            border_rect: Rect::new(0, 0, 1024, 60),
            border_color: Color::RGBA(200, 200, 200, 0),
        };
        assert_eq!(canvas, expected);
    }

    #[test]
    fn assert_prepare_ui() {
        let config = support::build_config();
        let mut widget = MenuBar::new(Arc::clone(&config));
        widget.prepare_ui();
        assert_eq!(widget.dest(), Rect::new(0, 0, 1024, 60));
    }
}
