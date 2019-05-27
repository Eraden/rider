#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct ThemeImages {
    directory_icon: String,
    file_icon: String,
    save_icon: String,
}

impl ThemeImages {
    pub fn new(directory_icon: String, file_icon: String, save_icon: String) -> Self {
        Self {
            file_icon,
            directory_icon,
            save_icon,
        }
    }

    pub fn directory_icon(&self) -> String {
        self.directory_icon.clone()
    }

    pub fn file_icon(&self) -> String {
        self.file_icon.clone()
    }

    pub fn save_icon(&self) -> String {
        self.save_icon.clone()
    }
}

impl Default for ThemeImages {
    fn default() -> Self {
        Self {
            directory_icon: "default/images/directory-64x64.png".to_string(),
            file_icon: "default/images/file-64x64.png".to_string(),
            save_icon: "default/images/save-64x64.png".to_string(),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn assert_directory_icon() {
        let config = ThemeImages::new("foo".to_owned(), "bar".to_owned(), "baz".to_owned());
        let result = config.directory_icon();
        let expected = "foo".to_owned();
        assert_eq!(result, expected);
    }

    #[test]
    fn assert_file_icon() {
        let config = ThemeImages::new("foo".to_owned(), "bar".to_owned(), "baz".to_owned());
        let result = config.file_icon();
        let expected = "bar".to_owned();
        assert_eq!(result, expected);
    }

    #[test]
    fn assert_save_icon() {
        let config = ThemeImages::new("foo".to_owned(), "bar".to_owned(), "baz".to_owned());
        let result = config.save_icon();
        let expected = "baz".to_owned();
        assert_eq!(result, expected);
    }
}
