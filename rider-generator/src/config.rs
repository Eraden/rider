use crate::images;
use rider_config::directories::*;
use std::fs;
use std::path::PathBuf;

pub fn create(directories: &Directories) -> std::io::Result<()> {
    if !directories.themes_dir.exists() {
        fs::create_dir_all(&directories.themes_dir)?;
        images::create(directories)?;
    }

    if !directories.fonts_dir.exists() {
        fs::create_dir_all(&directories.fonts_dir)?;
    }
    write_default_fonts(directories)?;

    if !directories.log_dir.exists() {
        fs::create_dir_all(&directories.log_dir)?;
    }

    if !directories.project_dir.exists() {
        fs::create_dir_all(&directories.project_dir)?;
    }
    Ok(())
}

fn write_default_fonts(directories: &Directories) -> std::io::Result<()> {
    let path = directories.fonts_dir.clone().to_str().unwrap().to_owned();
    let mut buf = PathBuf::new();
    buf.push(path);
    buf.push("DejaVuSansMono.ttf");
    if !buf.exists() {
        let contents = include_bytes!("../assets/fonts/DejaVuSansMono.ttf");
        fs::write(buf, contents.to_vec())?;
    }

    let path = directories.fonts_dir.clone().to_str().unwrap().to_owned();
    let mut buf = PathBuf::new();
    buf.push(path);
    buf.push("ElaineSans-Medium.ttf");
    if !buf.exists() {
        let contents = include_bytes!("../assets/fonts/ElaineSans-Medium.ttf");
        fs::write(buf, contents.to_vec())?;
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::create_dir_all;
    use std::path::Path;
    use uuid::Uuid;

    #[cfg(test)]
    fn join(a: String, b: String) -> String {
        vec![a, b].join("/")
    }

    #[test]
    fn assert_create_fonts() {
        let unique = Uuid::new_v4();
        let test_path = join("/tmp/rider-tests".to_owned(), unique.to_string());
        create_dir_all(test_path.clone()).unwrap();
        let directories = Directories::new(Some(test_path.clone()), None);
        assert_eq!(create(&directories).is_ok(), true);
        assert_eq!(
            Path::new(join(test_path.clone(), "rider/fonts".to_owned()).as_str()).exists(),
            true
        );
    }

    #[test]
    fn assert_create_log() {
        let unique = Uuid::new_v4();
        let test_path = join("/tmp/rider-tests".to_owned(), unique.to_string());
        create_dir_all(test_path.clone()).unwrap();
        let directories = Directories::new(Some(test_path.clone()), None);
        assert_eq!(create(&directories).is_ok(), true);
        assert_eq!(
            Path::new(join(test_path.clone(), "rider/log".to_owned()).as_str()).exists(),
            true
        );
    }

    #[test]
    fn assert_create_themes() {
        let unique = Uuid::new_v4();
        let test_path = join("/tmp/rider-tests".to_owned(), unique.to_string());
        create_dir_all(test_path.clone()).unwrap();
        let directories = Directories::new(Some(test_path.clone()), None);
        assert_eq!(
            Path::new(join(test_path.clone(), "rider/themes".to_owned()).as_str()).exists(),
            false
        );
        assert_eq!(create(&directories).is_ok(), true);
        assert_eq!(
            Path::new(join(test_path.clone(), "rider/themes".to_owned()).as_str()).exists(),
            true
        );
    }
}
