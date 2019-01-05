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
