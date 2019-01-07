use sdl2::pixels::Color;

#[derive(Clone, Debug, PartialEq)]
pub struct CaretColor {
    bright: Color,
    blur: Color,
}

impl CaretColor {
    pub fn new(bright: Color, blur: Color) -> Self {
        Self { bright, blur }
    }

    pub fn bright(&self) -> &Color {
        &self.bright
    }

    pub fn blur(&self) -> &Color {
        &self.blur
    }
}

#[cfg(test)]
mod test_getters {
    use super::*;
    use sdl2::pixels::*;

    #[test]
    fn assert_bright() {
        let target = CaretColor::new(Color::RGBA(1, 2, 3, 4), Color::RGBA(5, 6, 7, 8));
        let result = target.bright().clone();
        let expected = Color::RGBA(1, 2, 3, 4);
        assert_eq!(result, expected);
    }

    #[test]
    fn assert_blur() {
        let target = CaretColor::new(Color::RGBA(1, 2, 3, 4), Color::RGBA(5, 6, 7, 8));
        let result = target.blur().clone();
        let expected = Color::RGBA(5, 6, 7, 8);
        assert_eq!(result, expected);
    }
}
