use crate::SerdeColor;

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct ThemeConfig {
    color: SerdeColor,
    italic: bool,
    bold: bool,
}

impl ThemeConfig {
    pub fn new(color: SerdeColor, italic: bool, bold: bool) -> Self {
        Self {
            color,
            italic,
            bold,
        }
    }

    pub fn color(&self) -> &SerdeColor {
        &self.color
    }

    pub fn italic(&self) -> bool {
        self.italic
    }

    pub fn bold(&self) -> bool {
        self.bold
    }
}
