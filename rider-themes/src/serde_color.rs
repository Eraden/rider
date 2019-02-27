use sdl2::pixels::Color;

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct SerdeColor {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl SerdeColor {
    pub fn new(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self { r, g, b, a }
    }
}

impl Into<Color> for &SerdeColor {
    fn into(self) -> Color {
        Color {
            r: self.r,
            g: self.g,
            b: self.b,
            a: self.a,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use sdl2::pixels::Color;

    #[test]
    fn must_cast_serde_color_to_color() {
        let target = SerdeColor::new(12, 34, 56, 78);
        let color: Color = (&target).into();
        let expected = Color::RGBA(12, 34, 56, 78);
        assert_eq!(color, expected);
    }

    #[test]
    fn must_assign_to_proper_fields() {
        let color = SerdeColor::new(12, 34, 56, 78);
        assert_eq!(color.r, 12);
        assert_eq!(color.g, 34);
        assert_eq!(color.b, 56);
        assert_eq!(color.a, 78);
    }
}
