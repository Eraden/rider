use crate::SerdeColor;
use crate::ThemeConfig;

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct CaretColor {
    bright: ThemeConfig,
    blur: ThemeConfig,
}

impl Default for CaretColor {
    fn default() -> Self {
        Self {
            bright: ThemeConfig::new(SerdeColor::new(120, 120, 120, 0), false, false),
            blur: ThemeConfig::new(SerdeColor::new(0, 0, 0, 0), false, false),
        }
    }
}

impl CaretColor {
    pub fn new(bright: ThemeConfig, blur: ThemeConfig) -> Self {
        Self { bright, blur }
    }

    pub fn bright(&self) -> &ThemeConfig {
        &self.bright
    }

    pub fn blur(&self) -> &ThemeConfig {
        &self.blur
    }
}
