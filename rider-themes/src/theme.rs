use crate::CaretColor;
use crate::CodeHighlightingColor;
use crate::DiffColor;
use crate::SerdeColor;
use crate::ThemeImages;

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct Theme {
    name: String,
    background: SerdeColor,
    border_color: SerdeColor,
    caret: CaretColor,
    code_highlighting: CodeHighlightingColor,
    diff: DiffColor,
    images: ThemeImages,
}

impl Default for Theme {
    fn default() -> Self {
        Self {
            name: "default".to_string(),
            background: SerdeColor::new(255, 255, 255, 0),
            border_color: SerdeColor::new(0, 0, 0, 0),
            caret: CaretColor::default(),
            code_highlighting: CodeHighlightingColor::default(),
            diff: DiffColor::default(),
            images: ThemeImages::default(),
        }
    }
}

impl Theme {
    pub fn new(
        name: String,
        background: SerdeColor,
        border_color: SerdeColor,
        caret: CaretColor,
        code_highlighting: CodeHighlightingColor,
        diff: DiffColor,
        images: ThemeImages,
    ) -> Self {
        Self {
            name,
            background,
            border_color,
            caret,
            code_highlighting,
            diff,
            images,
        }
    }

    pub fn name(&self) -> &String {
        &self.name
    }

    pub fn background(&self) -> &SerdeColor {
        &self.background
    }

    pub fn border_color(&self) -> &SerdeColor {
        &self.border_color
    }

    pub fn caret(&self) -> &CaretColor {
        &self.caret
    }

    pub fn diff(&self) -> &DiffColor {
        &self.diff
    }

    pub fn code_highlighting(&self) -> &CodeHighlightingColor {
        &self.code_highlighting
    }

    pub fn images(&self) -> &ThemeImages {
        &self.images
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn assert_name() {
        let target = Theme::default();
        let result = target.name().clone();
        let expected = "default".to_owned();
        assert_eq!(result, expected);
    }

    #[test]
    fn assert_background() {
        let target = Theme::default();
        let result = target.background().clone();
        let expected = SerdeColor::new(255, 255, 255, 0);
        assert_eq!(result, expected);
    }

    #[test]
    fn assert_border_color() {
        let target = Theme::default();
        let result = target.border_color().clone();
        let expected = SerdeColor::new(0, 0, 0, 0);
        assert_eq!(result, expected);
    }

    #[test]
    fn assert_caret() {
        let target = Theme::default();
        let result = target.caret().clone();
        let expected = CaretColor::default();
        assert_eq!(result, expected);
    }

    #[test]
    fn assert_diff() {
        let target = Theme::default();
        let result = target.diff().clone();
        let expected = DiffColor::default();
        assert_eq!(result, expected);
    }

    #[test]
    fn assert_code_highlighting() {
        let target = Theme::default();
        let result = target.code_highlighting().clone();
        let expected = CodeHighlightingColor::default();
        assert_eq!(result, expected);
    }

    #[test]
    fn assert_images() {
        let target = Theme::default();
        let result = target.images().clone();
        let expected = ThemeImages::default();
        assert_eq!(result, expected);
    }
}
