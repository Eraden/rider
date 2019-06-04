use crate::app::UpdateResult as UR;
use crate::ui::*;
use rider_config::ConfigAccess;
use sdl2::rect::{Point, Rect};
use std::ops::Deref;

#[derive(Clone, Debug, PartialEq)]
pub struct Caret {
    blink_delay: u8,
    state: CaretState,
    position: CaretPosition,
    dest: Rect,
    colors: CaretColor,
}

impl Caret {
    pub fn new(config: ConfigAccess) -> Self {
        let read_config = config.read().unwrap();
        let bright = read_config.theme().caret().bright().color().into();
        let blur = read_config.theme().caret().blur().color().into();
        Self {
            state: CaretState::Bright,
            blink_delay: 0,
            dest: Rect::new(0, 0, 6, 0),
            colors: CaretColor::new(bright, blur),
            position: CaretPosition::new(0, 0, 0),
        }
    }

    fn toggle_state(&mut self) {
        self.state = match self.state {
            CaretState::Bright => CaretState::Blur,
            CaretState::Blur => CaretState::Bright,
        };
    }

    pub fn reset_caret(&mut self) {
        self.dest.set_x(0);
        self.dest.set_y(0);
        self.position.reset();
    }

    pub fn move_caret(&mut self, position: CaretPosition, pos: Point) {
        self.position = position;
        self.dest.set_x(pos.x());
        self.dest.set_y(pos.y());
    }

    pub fn position(&self) -> &CaretPosition {
        &self.position
    }

    pub fn state(&self) -> &CaretState {
        &self.state
    }
}

impl Deref for Caret {
    type Target = CaretPosition;

    fn deref(&self) -> &<Self as Deref>::Target {
        self.position()
    }
}

#[cfg_attr(tarpaulin, skip)]
impl Caret {
    pub fn render<T>(&self, canvas: &mut T, context: &RenderContext)
    where
        T: CanvasAccess,
    {
        use std::borrow::*;

        let dest = match context.borrow() {
            RenderContext::ParentPosition(p) => move_render_point(p.clone(), &self.dest),
            _ => self.dest().clone(),
        };
        let start = Point::new(dest.x(), dest.y());
        let end = Point::new(dest.x(), dest.y() + dest.height() as i32);
        let color = match self.state {
            CaretState::Bright => self.colors.bright(),
            CaretState::Blur => self.colors.blur(),
        }
        .clone();
        canvas
            .render_line(start, end, color)
            .unwrap_or_else(|_| panic!("Failed to draw a caret"));
    }

    pub fn prepare_ui<T>(&mut self, renderer: &mut T)
    where
        T: CharacterSizeManager,
    {
        let rect = renderer.load_character_size('I');
        self.dest.set_height(rect.height());
    }
}

impl Caret {
    pub fn update(&mut self) -> UR {
        self.blink_delay += 1;
        if self.blink_delay >= 15 {
            self.blink_delay = 0;
            self.toggle_state();
        }
        UR::NoOp
    }
}

impl ClickHandler for Caret {
    fn on_left_click(&mut self, _point: &Point, _context: &UpdateContext) -> UR {
        UR::NoOp
    }

    fn is_left_click_target(&self, point: &Point, context: &UpdateContext) -> bool {
        let dest = self.dest();
        is_in_rect(
            point,
            &match context {
                &UpdateContext::ParentPosition(p) => move_render_point(p, &dest),
                _ => dest,
            },
        )
    }
}

impl RenderBox for Caret {
    fn render_start_point(&self) -> Point {
        self.dest().top_left()
    }

    fn dest(&self) -> Rect {
        self.dest.clone()
    }
}

#[cfg(test)]
mod test_own_methods {
    use crate::tests::*;
    use crate::ui::*;
    use sdl2::rect::{Point, Rect};
    use std::sync::*;

    #[test]
    fn assert_move_caret() {
        let config = support::build_config();
        let mut widget = Caret::new(Arc::clone(&config));
        widget.move_caret(widget.moved(10, 21, 34), Point::new(10, 20));
        let result = (
            widget.text_position(),
            widget.line_number(),
            widget.line_position(),
            widget.dest().clone(),
        );
        let expected = (10, 21, 34, Rect::new(10, 20, 6, 1));
        assert_eq!(result, expected);
    }

    #[test]
    fn assert_reset() {
        let config = support::build_config();
        let mut widget = Caret::new(Arc::clone(&config));
        widget.reset_caret();
        let result = (
            widget.text_position(),
            widget.line_number(),
            widget.line_position(),
            widget.dest().clone(),
        );
        let expected = (0, 0, 0, Rect::new(0, 0, 6, 1));
        assert_eq!(result, expected);
    }

    #[test]
    fn assert_toggle_state() {
        let config = support::build_config();
        let mut widget = Caret::new(Arc::clone(&config));

        let old = widget.state().clone();
        widget.toggle_state();
        let new = widget.state().clone();
        assert_ne!(old, new);

        let old = widget.state().clone();
        widget.toggle_state();
        let new = widget.state().clone();
        assert_ne!(old, new);

        let old = widget.state().clone();
        widget.toggle_state();
        let new = widget.state().clone();
        assert_ne!(old, new);
    }
}

#[cfg(test)]
mod test_deref {
    use crate::tests::*;
    use crate::ui::*;
    use sdl2::rect::Point;
    use std::sync::*;

    #[test]
    fn must_deref_text_position() {
        let config = support::build_config();
        let mut widget = Caret::new(Arc::clone(&config));
        widget.move_caret(widget.moved(10, 21, 34), Point::new(0, 0));
        let result = widget.text_position();
        let expected: usize = 10;
        assert_eq!(result, expected);
    }

    #[test]
    fn must_deref_line_number() {
        let config = support::build_config();
        let mut widget = Caret::new(Arc::clone(&config));
        widget.move_caret(widget.moved(10, 21, 34), Point::new(0, 0));
        let result = widget.line_number();
        let expected: usize = 21;
        assert_eq!(result, expected);
    }

    #[test]
    fn must_deref_line_position() {
        let config = support::build_config();
        let mut widget = Caret::new(Arc::clone(&config));
        widget.move_caret(widget.moved(10, 21, 34), Point::new(0, 0));
        let result = widget.line_position();
        let expected: usize = 34;
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
        let widget = Caret::new(Arc::clone(&config));
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
        let widget = Caret::new(Arc::clone(&config));
        let point = Point::new(9999, 9999);
        let context = UpdateContext::Nothing;
        let result = widget.is_left_click_target(&point, &context);
        assert_eq!(result, false);
    }

    #[test]
    fn assert_when_click_target() {
        let config = support::build_config();
        let widget = Caret::new(Arc::clone(&config));

        let point = Point::new(0, 0);
        let context = UpdateContext::Nothing;
        let result = widget.is_left_click_target(&point, &context);
        assert_eq!(result, true);
    }

    #[test]
    fn refute_when_not_click_target_because_parent() {
        let config = support::build_config();
        let widget = Caret::new(Arc::clone(&config));
        let point = Point::new(20, 30);
        let context = UpdateContext::ParentPosition(Point::new(9999, 9999));
        let result = widget.is_left_click_target(&point, &context);
        assert_eq!(result, false);
    }

    #[test]
    fn assert_when_click_target_because_parent() {
        let config = support::build_config();
        let widget = Caret::new(Arc::clone(&config));
        let point = Point::new(10, 10);
        let context = UpdateContext::ParentPosition(Point::new(10, 10));
        let result = widget.is_left_click_target(&point, &context);
        assert_eq!(result, true);
    }

    #[test]
    fn assert_on_click_do_nothing() {
        let config = support::build_config();
        let mut widget = Caret::new(Arc::clone(&config));
        let point = Point::new(12, 34);
        let context = UpdateContext::ParentPosition(Point::new(678, 293));
        let result = widget.on_left_click(&point, &context);
        let expected = UpdateResult::NoOp;
        assert_eq!(result, expected);
    }
}

#[cfg(test)]
mod test_render {
    use crate::tests::support::build_config;
    use crate::ui::*;
    use sdl2::pixels::Color;
    use sdl2::rect::{Point, Rect};
    use sdl2::render::Texture;
    use std::rc::Rc;

    struct CanvasMock {
        pub start: Point,
        pub end: Point,
        pub color: sdl2::pixels::Color,
    }

    impl CanvasAccess for CanvasMock {
        fn render_rect(&mut self, _rect: Rect, _color: Color) -> Result<(), String> {
            unimplemented!()
        }

        fn render_border(&mut self, _rect: Rect, _color: Color) -> Result<(), String> {
            unimplemented!()
        }

        fn render_image(
            &mut self,
            _tex: Rc<Texture>,
            _src: Rect,
            _dest: Rect,
        ) -> Result<(), String> {
            unimplemented!()
        }

        fn render_line(
            &mut self,
            start: Point,
            end: Point,
            color: sdl2::pixels::Color,
        ) -> Result<(), String> {
            self.start = start;
            self.end = end;
            self.color = color;
            Ok(())
        }

        fn set_clipping(&mut self, _rect: Rect) {
            unimplemented!()
        }
    }

    impl CharacterSizeManager for CanvasMock {
        fn load_character_size(&mut self, _c: char) -> Rect {
            Rect::new(0, 2, 12, 23)
        }
    }

    #[test]
    fn assert_render_line() {
        let config = build_config();
        let context = RenderContext::ParentPosition(Point::new(10, 14));
        let mut canvas = CanvasMock {
            start: Point::new(0, 0),
            end: Point::new(0, 0),
            color: sdl2::pixels::Color::RGB(0, 0, 0),
        };
        let mut widget = Caret::new(config);
        widget.move_caret(CaretPosition::new(0, 0, 0), Point::new(23, 23));
        widget.render(&mut canvas, &context);
        assert_eq!(canvas.start, Point::new(33, 37));
        assert_eq!(canvas.end, Point::new(33, 38));
        assert_eq!(canvas.color, sdl2::pixels::Color::RGBA(121, 121, 121, 0));
    }

    #[test]
    fn assert_prepare_ui() {
        let config = build_config();
        let mut canvas = CanvasMock {
            start: Point::new(0, 0),
            end: Point::new(0, 0),
            color: sdl2::pixels::Color::RGB(0, 0, 0),
        };
        let mut widget = Caret::new(config);
        widget.move_caret(CaretPosition::new(0, 0, 0), Point::new(11, 12));
        widget.prepare_ui(&mut canvas);
        assert_eq!(widget.dest(), Rect::new(11, 12, 6, 23));
    }
}
