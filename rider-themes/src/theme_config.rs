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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn assert_color() {
        let target = ThemeConfig::new(SerdeColor::new(29, 20, 45, 72), true, false);
        let result = target.color().clone();
        let expected = SerdeColor::new(29, 20, 45, 72);
        assert_eq!(result, expected);
    }

    #[test]
    fn assert_italic() {
        let target = ThemeConfig::new(SerdeColor::new(29, 20, 45, 72), true, false);
        let result = target.italic();
        let expected = true;
        assert_eq!(result, expected);
    }

    #[test]
    fn assert_bold() {
        let target = ThemeConfig::new(SerdeColor::new(29, 20, 45, 72), false, true);
        let result = target.bold();
        let expected = true;
        assert_eq!(result, expected);
    }
}
