use crate::write_bytes_to::write_bytes_to;
use rider_config::directories::*;
use std::fs::create_dir_all;
use std::path::PathBuf;

pub fn create(directories: &Directories) -> std::io::Result<()> {
    default_theme(directories)?;
    railscasts_theme(directories)?;
    Ok(())
}

fn create_default_directory_icon(dir: &PathBuf) -> std::io::Result<()> {
    let blob = include_bytes!("../assets/themes/default/images/directory-64x64.png");
    write_bytes_to(dir, "directory-64x64.png", blob)?;
    Ok(())
}

fn create_default_file_icon(dir: &PathBuf) -> std::io::Result<()> {
    let blob = include_bytes!("../assets/themes/default/images/file-64x64.png");
    write_bytes_to(dir, "file-64x64.png", blob)?;
    Ok(())
}

fn default_theme(directories: &Directories) -> std::io::Result<()> {
    let mut dir = PathBuf::new();
    dir.push(directories.themes_dir.clone());
    dir.push("default");
    dir.push("images");
    let r = create_dir_all(&dir);
    #[cfg_attr(tarpaulin, skip)]
    r.unwrap_or_else(|_| panic!("Cannot create themes config directory"));

    create_default_directory_icon(&dir)?;
    create_default_file_icon(&dir)?;
    Ok(())
}

fn create_railscasts_directory_icon(dir: &PathBuf) -> std::io::Result<()> {
    let blob = include_bytes!("../assets/themes/railscasts/images/directory-64x64.png");
    write_bytes_to(dir, "directory-64x64.png", blob)?;
    Ok(())
}

fn create_railscasts_file_icon(dir: &PathBuf) -> std::io::Result<()> {
    let blob = include_bytes!("../assets/themes/railscasts/images/file-64x64.png");
    write_bytes_to(dir, "file-64x64.png", blob)?;
    Ok(())
}

fn create_railscasts_save_icon(dir: &PathBuf) -> std::io::Result<()> {
    let blob = include_bytes!("../assets/themes/railscasts/images/save-64x64.png");
    write_bytes_to(dir, "save-64x64.png", blob)?;
    Ok(())
}

fn railscasts_theme(directories: &Directories) -> std::io::Result<()> {
    let mut dir = PathBuf::new();
    dir.push(directories.themes_dir.clone());
    dir.push("railscasts");
    dir.push("images");
    create_dir_all(&dir)?;
    create_railscasts_directory_icon(&dir)?;
    create_railscasts_file_icon(&dir)?;
    create_railscasts_save_icon(&dir)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::create_dir_all;
    use std::path::{Path, PathBuf};
    use uuid::Uuid;

    #[cfg(test)]
    fn join(a: String, b: String) -> String {
        vec![a, b].join("/")
    }

    #[test]
    fn assert_create() {
        let uniq = Uuid::new_v4();
        let test_path = join("/tmp/rider-tests".to_owned(), uniq.to_string());
        create_dir_all(test_path.clone()).unwrap();
        let directories = Directories::new(Some(test_path.clone()), None);
        let themes_dir = join(test_path.clone(), "rider/themes".to_owned());
        assert_eq!(
            Path::new(
                join(
                    themes_dir.clone(),
                    "railscasts/images/directory-64x64.png".to_owned()
                )
                .as_str()
            )
            .exists(),
            false
        );
        assert_eq!(
            Path::new(
                join(
                    themes_dir.clone(),
                    "railscasts/images/file-64x64.png".to_owned()
                )
                .as_str()
            )
            .exists(),
            false
        );
        assert_eq!(
            Path::new(
                join(
                    themes_dir.clone(),
                    "default/images/directory-64x64.png".to_owned()
                )
                .as_str()
            )
            .exists(),
            false
        );
        assert_eq!(
            Path::new(
                join(
                    themes_dir.clone(),
                    "default/images/file-64x64.png".to_owned()
                )
                .as_str()
            )
            .exists(),
            false
        );
        assert_eq!(create(&directories).is_ok(), true);
        assert_eq!(
            Path::new(
                join(
                    themes_dir.clone(),
                    "railscasts/images/directory-64x64.png".to_owned()
                )
                .as_str()
            )
            .exists(),
            true
        );
        assert_eq!(
            Path::new(
                join(
                    themes_dir.clone(),
                    "railscasts/images/file-64x64.png".to_owned()
                )
                .as_str()
            )
            .exists(),
            true
        );
        assert_eq!(
            Path::new(
                join(
                    themes_dir.clone(),
                    "default/images/directory-64x64.png".to_owned()
                )
                .as_str()
            )
            .exists(),
            true
        );
        assert_eq!(
            Path::new(
                join(
                    themes_dir.clone(),
                    "default/images/file-64x64.png".to_owned()
                )
                .as_str()
            )
            .exists(),
            true
        );
    }

    #[test]
    fn assert_create_default_directory_icon() {
        let uniq = Uuid::new_v4();
        let test_path = join("/tmp/rider-tests".to_owned(), uniq.to_string());
        create_dir_all(test_path.clone()).unwrap();
        let file_path: String = join(test_path.clone(), "directory-64x64.png".to_owned());
        let dir: PathBuf = test_path.into();
        assert_eq!(Path::new(file_path.as_str()).exists(), false);
        assert_eq!(create_default_directory_icon(&dir).is_ok(), true);
        assert_eq!(Path::new(file_path.as_str()).exists(), true);
    }

    #[test]
    fn assert_create_default_file_icon() {
        let uniq = Uuid::new_v4();
        let test_path = join("/tmp/rider-tests".to_owned(), uniq.to_string());
        create_dir_all(test_path.clone()).unwrap();
        let file_path: String = join(test_path.clone(), "file-64x64.png".to_owned());
        let dir: PathBuf = test_path.into();
        assert_eq!(Path::new(file_path.as_str()).exists(), false);
        assert_eq!(create_default_file_icon(&dir).is_ok(), true);
        assert_eq!(Path::new(file_path.as_str()).exists(), true);
    }

    #[test]
    fn assert_create_railscasts_directory_icon() {
        let uniq = Uuid::new_v4();
        let test_path = join("/tmp/rider-tests".to_owned(), uniq.to_string());
        create_dir_all(test_path.clone()).unwrap();
        let file_path: String = join(test_path.clone(), "directory-64x64.png".to_owned());
        let dir: PathBuf = test_path.into();
        assert_eq!(Path::new(file_path.as_str()).exists(), false);
        assert_eq!(create_railscasts_directory_icon(&dir).is_ok(), true);
        assert_eq!(Path::new(file_path.as_str()).exists(), true);
    }

    #[test]
    fn assert_create_railscasts_file_icon() {
        let uniq = Uuid::new_v4();
        let test_path = join("/tmp/rider-tests".to_owned(), uniq.to_string());
        create_dir_all(test_path.clone()).unwrap();
        let file_path: String = join(test_path.clone(), "file-64x64.png".to_owned());
        let dir: PathBuf = test_path.into();
        assert_eq!(Path::new(file_path.as_str()).exists(), false);
        assert_eq!(create_railscasts_file_icon(&dir).is_ok(), true);
        assert_eq!(Path::new(file_path.as_str()).exists(), true);
    }
}
