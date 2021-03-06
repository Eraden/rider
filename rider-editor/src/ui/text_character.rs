use crate::app::UpdateResult as UR;
use crate::renderer::*;
use crate::ui::caret::CaretPosition;
use crate::ui::*;
use rider_config::{ConfigAccess, ConfigHolder};

use sdl2::pixels::Color;
use sdl2::rect::{Point, Rect};
use std::fmt::Debug;
use std::fmt::Error;
use std::fmt::Formatter;

pub trait CharacterSizeManager {
    fn load_character_size(&mut self, c: char) -> Rect;
}

#[derive(Clone)]
pub struct TextCharacter {
    text_character: char,
    position: usize,
    line: usize,
    last_in_line: bool,
    source: Rect,
    dest: Rect,
    color: Color,
    config: ConfigAccess,
}

impl TextCharacter {
    pub fn new(
        text_character: char,
        position: usize,
        line: usize,
        last_in_line: bool,
        color: Color,
        config: ConfigAccess,
    ) -> Self {
        Self {
            text_character,
            position,
            line,
            last_in_line,
            source: Rect::new(0, 0, 0, 0),
            dest: Rect::new(0, 0, 0, 0),
            color,
            config,
        }
    }

    pub fn is_last_in_line(&self) -> bool {
        self.last_in_line
    }

    pub fn color(&self) -> &Color {
        &self.color
    }

    pub fn update_position(&mut self, current: &mut Rect) {
        if self.is_new_line() {
            let y = self.source.height() as i32;
            self.dest.set_x(current.x());
            self.dest.set_y(current.y());
            current.set_x(0);
            current.set_y(current.y() + y);
        } else {
            self.dest.set_x(current.x());
            self.dest.set_y(current.y());
            self.dest.set_width(self.source.width());
            self.dest.set_height(self.source.height());
            current.set_x(self.dest.x() + self.source.width() as i32);
        }
    }

    #[inline]
    pub fn is_new_line(&self) -> bool {
        self.text_character == '\n'
    }

    pub fn position(&self) -> usize {
        self.position
    }

    pub fn line(&self) -> usize {
        self.line
    }

    pub fn text_character(&self) -> char {
        self.text_character.clone()
    }
}

impl Widget for TextCharacter {
    fn texture_path(&self) -> Option<String> {
        None
    }

    fn dest(&self) -> &Rect {
        &self.dest
    }

    fn set_dest(&mut self, rect: &Rect) {
        self.dest = rect.clone()
    }

    fn source(&self) -> &Rect {
        &self.source
    }

    fn set_source(&mut self, rect: &Rect) {
        self.source = rect.clone();
    }

    fn on_left_click(&mut self, _point: &Point, _context: &UpdateContext) -> UR {
        UR::MoveCaret(
            self.dest.clone(),
            CaretPosition::new(self.position(), self.line(), 0),
        )
    }

    /**
     * Must first create targets so even if new line appear renderer will know
     * where move render starting point
     */
    fn render<C, R>(&self, canvas: &mut C, renderer: &mut R, context: &RenderContext)
    where
        C: CanvasAccess,
        R: Renderer + CharacterSizeManager + ConfigHolder,
    {
        let font_details: FontDetails = renderer.config().read().unwrap().editor_config().into();

        let c = match self.text_character.clone() {
            '\n' => '¬',
            ' ' => '·',
            c => c,
        };
        let mut details = TextDetails {
            text: c.to_string(),
            color: self.color.clone(),
            font: font_details.clone(),
        };
        let dest = match context {
            RenderContext::ParentPosition(p) => move_render_point(p.clone(), &self.dest),
            _ => self.dest().clone(),
        };

        if let Ok(texture) = renderer.load_text_tex(&mut details, font_details) {
            canvas
                .render_image(texture, self.source.clone(), dest)
                .unwrap();
        }
    }

    fn prepare_ui<'l, T>(&mut self, renderer: &mut T)
    where
        T: Renderer + CharacterSizeManager + ConfigHolder,
    {
        let font_details: FontDetails = renderer.config().read().unwrap().editor_config().into();
        let rect = renderer.load_character_size(self.text_character);
        self.set_source(&rect);
        self.set_dest(&rect);

        let mut details = TextDetails {
            text: self.text_character.to_string(),
            color: self.color.clone(),
            font: font_details.clone(),
        };

        if let Err(error_message) = renderer.load_text_tex(&mut details, font_details) {
            info!(
                "Could not create texture for '{:?}' with {:?}",
                self.text_character, error_message
            )
        }
    }
}

impl PartialEq for TextCharacter {
    fn eq(&self, other: &Self) -> bool {
        self.line == other.line
            && self.position == other.position
            && self.last_in_line == other.last_in_line
            && self.dest == other.dest
            && self.source == other.source
            && self.color == other.color
    }
}

impl Debug for TextCharacter {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        write!(
            f,
            "TextCharacter {{ text_character: {:?}, position: {:?}, line: {:?}, last_in_line: {:?}, source: {:?}, dest: {:?}, color: {:?} }}",
            self.text_character,
            self.position,
            self.line,
            self.last_in_line,
            self.source,
            self.dest,
            self.color
        )
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
    fn must_return_valid_is_last_in_line() {
        let config = build_config();
        let widget = TextCharacter::new(
            '\n',
            0,
            0,
            true,
            Color::RGB(1, 12, 123),
            Arc::clone(&config),
        );
        assert_eq!(widget.is_last_in_line(), true);
    }

    #[test]
    fn must_return_true_for_is_new_line_if_new_line() {
        let config = build_config();
        let widget = TextCharacter::new(
            '\n',
            0,
            0,
            true,
            Color::RGB(1, 12, 123),
            Arc::clone(&config),
        );
        assert_eq!(widget.is_new_line(), true);
    }

    #[test]
    fn must_return_false_for_is_new_line_if_new_line() {
        let config = build_config();
        let widget =
            TextCharacter::new('W', 0, 0, true, Color::RGB(1, 12, 123), Arc::clone(&config));
        assert_eq!(widget.is_new_line(), false);
    }

    #[test]
    fn must_return_valid_position() {
        let config = build_config();
        let widget = TextCharacter::new(
            '\n',
            1,
            123,
            true,
            Color::RGB(1, 12, 123),
            Arc::clone(&config),
        );
        assert_eq!(widget.position(), 1);
    }

    #[test]
    fn must_return_valid_line() {
        let config = build_config();
        let widget = TextCharacter::new(
            '\n',
            1,
            123,
            true,
            Color::RGB(1, 12, 123),
            Arc::clone(&config),
        );
        assert_eq!(widget.line(), 123);
    }

    #[test]
    fn must_return_valid_text_character() {
        let config = build_config();
        let widget = TextCharacter::new(
            '\n',
            87,
            123,
            true,
            Color::RGB(1, 12, 123),
            Arc::clone(&config),
        );
        assert_eq!(widget.text_character(), '\n');
    }

    #[test]
    fn must_return_valid_source() {
        let config = build_config();
        let widget = TextCharacter::new(
            '\n',
            0,
            0,
            true,
            Color::RGB(1, 12, 123),
            Arc::clone(&config),
        );
        assert_eq!(widget.source(), &Rect::new(0, 0, 0, 0));
    }

    #[test]
    fn must_return_valid_dest() {
        let config = build_config();
        let widget = TextCharacter::new(
            '\n',
            0,
            0,
            true,
            Color::RGB(1, 12, 123),
            Arc::clone(&config),
        );
        assert_eq!(widget.dest(), &Rect::new(0, 0, 0, 0));
    }

    #[test]
    fn must_return_valid_color() {
        let config = build_config();
        let widget = TextCharacter::new(
            '\n',
            0,
            0,
            true,
            Color::RGB(1, 12, 123),
            Arc::clone(&config),
        );
        assert_eq!(widget.color(), &Color::RGB(1, 12, 123));
    }
}

#[cfg(test)]
mod test_own_methods {
    use crate::tests::*;
    use crate::ui::*;
    use sdl2::rect::Rect;
    use std::sync::*;

    #[test]
    fn must_update_position_of_new_line() {
        let config = build_config();
        let mut widget = TextCharacter::new(
            '\n',
            0,
            0,
            true,
            sdl2::pixels::Color::RGB(0, 0, 0),
            Arc::clone(&config),
        );
        widget.set_dest(&Rect::new(10, 20, 30, 40));
        widget.set_source(&Rect::new(50, 60, 70, 80));
        let mut current = Rect::new(10, 23, 0, 0);
        widget.update_position(&mut current);
        assert_eq!(current, Rect::new(0, 103, 1, 1));
        assert_eq!(widget.dest(), &Rect::new(10, 23, 30, 40));
        assert_eq!(widget.source(), &Rect::new(50, 60, 70, 80));
    }

    #[test]
    fn must_update_position_of_non_new_line() {
        let config = build_config();
        let mut widget = TextCharacter::new(
            'W',
            0,
            0,
            true,
            sdl2::pixels::Color::RGB(0, 0, 0),
            Arc::clone(&config),
        );
        widget.set_dest(&Rect::new(10, 20, 30, 40));
        widget.set_source(&Rect::new(50, 60, 70, 80));
        let mut current = Rect::new(10, 23, 0, 0);
        widget.update_position(&mut current);
        assert_eq!(current, Rect::new(80, 23, 1, 1));
        assert_eq!(widget.dest(), &Rect::new(10, 23, 70, 80));
        assert_eq!(widget.source(), &Rect::new(50, 60, 70, 80));
    }
}

#[cfg(test)]
mod test_click_handler {
    use crate::app::*;
    use crate::tests::*;
    use crate::ui::*;
    use sdl2::rect::{Point, Rect};
    use std::sync::*;

    #[test]
    fn refute_when_not_click_target() {
        let config = build_config();
        let mut widget = TextCharacter::new(
            '\n',
            0,
            0,
            true,
            sdl2::pixels::Color::RGB(0, 0, 0),
            Arc::clone(&config),
        );
        widget.set_dest(&Rect::new(10, 20, 30, 40));
        widget.set_source(&Rect::new(50, 60, 70, 80));
        let point = Point::new(0, 0);
        let context = UpdateContext::Nothing;
        let result = widget.is_left_click_target(&point, &context);
        assert_eq!(result, false);
    }

    #[test]
    fn assert_when_click_target() {
        let config = build_config();
        let mut widget = TextCharacter::new(
            '\n',
            0,
            0,
            true,
            sdl2::pixels::Color::RGB(0, 0, 0),
            Arc::clone(&config),
        );
        widget.set_dest(&Rect::new(10, 20, 30, 40));
        widget.set_source(&Rect::new(50, 60, 70, 80));
        let point = Point::new(20, 30);
        let context = UpdateContext::Nothing;
        let result = widget.is_left_click_target(&point, &context);
        assert_eq!(result, true);
    }

    #[test]
    fn refute_when_not_click_target_because_parent() {
        let config = build_config();
        let mut widget = TextCharacter::new(
            '\n',
            0,
            0,
            true,
            sdl2::pixels::Color::RGB(0, 0, 0),
            Arc::clone(&config),
        );
        widget.set_dest(&Rect::new(10, 20, 30, 40));
        widget.set_source(&Rect::new(50, 60, 70, 80));
        let point = Point::new(20, 30);
        let context = UpdateContext::ParentPosition(Point::new(100, 100));
        let result = widget.is_left_click_target(&point, &context);
        assert_eq!(result, false);
    }

    #[test]
    fn assert_when_click_target_because_parent() {
        let config = build_config();
        let mut widget = TextCharacter::new(
            '\n',
            0,
            0,
            true,
            sdl2::pixels::Color::RGB(0, 0, 0),
            Arc::clone(&config),
        );
        widget.set_dest(&Rect::new(10, 20, 30, 40));
        widget.set_source(&Rect::new(50, 60, 70, 80));
        let point = Point::new(120, 130);
        let context = UpdateContext::ParentPosition(Point::new(100, 100));
        let result = widget.is_left_click_target(&point, &context);
        assert_eq!(result, true);
    }

    #[test]
    fn assert_on_click_return_move_caret() {
        let config = build_config();
        let position = 1233;
        let line = 2893;
        let mut widget = TextCharacter::new(
            '\n',
            position.clone(),
            line.clone(),
            true,
            sdl2::pixels::Color::RGB(0, 0, 0),
            Arc::clone(&config),
        );
        let dest = Rect::new(10, 20, 30, 40);
        widget.set_dest(&dest);
        widget.set_source(&Rect::new(50, 60, 70, 80));

        let point = Point::new(12, 34);
        let context = UpdateContext::ParentPosition(Point::new(678, 293));
        let result = widget.on_left_click(&point, &context);
        let expected = UpdateResult::MoveCaret(dest, CaretPosition::new(position, line, 0));
        assert_eq!(result, expected);
    }
}

#[cfg(test)]
mod test_render_box {
    use crate::tests::*;
    use crate::ui::*;
    use sdl2::rect::{Point, Rect};
    use std::sync::*;

    #[test]
    fn must_return_top_left_point() {
        let config = build_config();
        let mut widget = TextCharacter::new(
            '\n',
            0,
            0,
            true,
            sdl2::pixels::Color::RGB(0, 0, 0),
            Arc::clone(&config),
        );
        widget.set_dest(&Rect::new(10, 20, 30, 40));
        widget.set_source(&Rect::new(50, 60, 70, 80));
        let result = widget.render_start_point();
        let expected = Point::new(10, 20);
        assert_eq!(result, expected);
    }
}

#[cfg(test)]
mod test_update {
    use crate::app::*;
    use crate::tests::*;
    use crate::ui::*;
    use sdl2::rect::{Point, Rect};
    use std::sync::*;

    #[test]
    fn assert_do_nothing() {
        let config = build_config();
        let mut widget = TextCharacter::new(
            '\n',
            0,
            0,
            true,
            sdl2::pixels::Color::RGB(0, 0, 0),
            Arc::clone(&config),
        );
        widget.set_dest(&Rect::new(10, 20, 30, 40));
        widget.set_source(&Rect::new(50, 60, 70, 80));
        let result = widget.update(
            3234,
            &UpdateContext::ParentPosition(Point::new(234, 234234)),
        );
        let expected = UpdateResult::NoOp;
        assert_eq!(result, expected);
    }
}
