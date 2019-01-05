use crate::themes::SerdeColor;
use crate::themes::ThemeConfig;

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct DiffColor {
    pub add: ThemeConfig,
    pub delete: ThemeConfig,
    pub change: ThemeConfig,
    pub text: ThemeConfig,
}

impl Default for DiffColor {
    fn default() -> Self {
        Self {
            add: ThemeConfig::new(SerdeColor::new(0, 0, 0, 0), false, false),
            delete: ThemeConfig::new(SerdeColor::new(0, 0, 0, 0), false, false),
            change: ThemeConfig::new(SerdeColor::new(0, 0, 0, 0), false, false),
            text: ThemeConfig::new(SerdeColor::new(0, 0, 0, 0), false, false),
        }
    }
}

impl DiffColor {
    pub fn new(
        add: ThemeConfig,
        delete: ThemeConfig,
        change: ThemeConfig,
        text: ThemeConfig,
    ) -> Self {
        Self {
            add,
            delete,
            change,
            text,
        }
    }
}
