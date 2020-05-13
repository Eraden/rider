use crate::directories::Directories;

#[cfg_attr(test, derive(PartialEq))]
#[derive(Debug, Clone)]
pub struct EditorConfig {
    character_size: u16,
    font_path: String,
    current_theme: String,
    margin_left: u16,
    margin_top: u16,
}

impl EditorConfig {
    pub fn new(directories: &Directories) -> Self {
        let mut default_font_path = directories.fonts_dir.clone();
        default_font_path.push("DejaVuSansMono.ttf");
        Self {
            character_size: 14,
            font_path: default_font_path.to_str().unwrap().to_string(),
            current_theme: "railscasts".to_string(),
            margin_left: 10,
            margin_top: 10,
        }
    }

    pub fn character_size(&self) -> u16 {
        self.character_size
    }

    pub fn font_path(&self) -> &str {
        self.font_path.as_str()
    }

    pub fn set_font_path<S>(&mut self, path: S)
    where
        S: Into<String>,
    {
        self.font_path = path.into();
    }

    pub fn current_theme(&self) -> &String {
        &self.current_theme
    }

    pub fn margin_left(&self) -> u16 {
        self.margin_left
    }

    pub fn margin_top(&self) -> u16 {
        self.margin_top
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn assert_font_path() {
        let directories = Directories::new(Some("/tmp".to_owned()), None);
        let mut config = EditorConfig::new(&directories);
        let path = config.font_path();
        let expected = "/tmp/rider/fonts/DejaVuSansMono.ttf";
        assert_eq!(path, expected);

        config.set_font_path("/a");
        let path = config.font_path();
        let expected = "/a";
        assert_eq!(path, expected);
    }

    #[test]
    fn assert_character_size() {
        let directories = Directories::new(Some("/tmp".to_owned()), None);
        let config = EditorConfig::new(&directories);
        let result = config.character_size();
        let expected: u16 = 14;
        assert_eq!(result, expected);
    }

    #[test]
    fn assert_current_theme() {
        let directories = Directories::new(Some("/tmp".to_owned()), None);
        let config = EditorConfig::new(&directories);
        let result = config.current_theme().to_owned();
        let expected = "railscasts".to_owned();
        assert_eq!(result, expected);
    }

    #[test]
    fn assert_margin_left() {
        let directories = Directories::new(Some("/tmp".to_owned()), None);
        let config = EditorConfig::new(&directories);
        let result = config.margin_left();
        let expected: u16 = 10;
        assert_eq!(result, expected);
    }

    #[test]
    fn assert_margin_top() {
        let directories = Directories::new(Some("/tmp".to_owned()), None);
        let config = EditorConfig::new(&directories);
        let result = config.margin_top();
        let expected: u16 = 10;
        assert_eq!(result, expected);
    }
}
