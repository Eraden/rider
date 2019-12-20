use crate::*;
use rider_themes::predef::*;
use rider_themes::Theme;
use std::fs;
use std::path::PathBuf;

pub fn create(directories: &Directories) -> std::io::Result<()> {
    fs::create_dir_all(directories.themes_dir.clone())?;
    for theme in default_styles() {
        write_theme(&theme, directories)?;
    }
    Ok(())
}

fn write_theme(theme: &Theme, directories: &Directories) -> std::io::Result<()> {
    let mut theme_path = PathBuf::new();
    theme_path.push(directories.themes_dir.clone());
    theme_path.push(format!("{}.json", theme.name()));
    let contents = serde_json::to_string_pretty(&theme).unwrap();
    fs::write(&theme_path, contents)?;
    Ok(())
}

fn default_styles() -> Vec<Theme> {
    vec![default::build_theme(), railscasts::build_theme()]
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::create_dir_all;
    use std::path::Path;
    use uuid::Uuid;

    #[test]
    fn assert_default_styles() {
        assert_eq!(default_styles().len(), 2);
    }

    #[cfg(test)]
    fn join(a: String, b: String) -> String {
        vec![a, b].join("/")
    }

    #[test]
    fn assert_create_default() {
        let uniq = Uuid::new_v4();
        let test_path = join("/tmp/rider-tests".to_owned(), uniq.to_string());
        create_dir_all(test_path.clone()).unwrap();
        let directories = Directories::new(Some(test_path.clone()), None);
        let rider_dir = join(test_path.clone(), "rider".to_owned());
        assert_eq!(
            Path::new(join(rider_dir.clone(), "themes/default.json".to_owned()).as_str()).exists(),
            false
        );
        assert_eq!(create(&directories).is_ok(), true);
        assert_eq!(
            Path::new(join(rider_dir.clone(), "themes/default.json".to_owned()).as_str()).exists(),
            true
        );
    }

    #[test]
    fn assert_create_railscasts() {
        let uniq = Uuid::new_v4();
        let test_path = join("/tmp/rider-tests".to_owned(), uniq.to_string());
        create_dir_all(test_path.clone()).unwrap();
        let directories = Directories::new(Some(test_path.clone()), None);
        let rider_dir = join(test_path.clone(), "rider".to_owned());
        assert_eq!(
            Path::new(join(rider_dir.clone(), "themes/default.json".to_owned()).as_str()).exists(),
            false
        );
        assert_eq!(create(&directories).is_ok(), true);
        assert_eq!(
            Path::new(join(rider_dir.clone(), "themes/railscasts.json".to_owned()).as_str())
                .exists(),
            true
        );
    }
}
