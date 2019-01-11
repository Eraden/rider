use rider_config::directories::*;
use std::fs::{create_dir_all};
use std::path::PathBuf;
use crate::write_bytes_to::write_bytes_to;

pub fn create() {
    default_theme();
    railscasts_theme();
}

fn create_default_directory_icon(dir: &PathBuf) {
    let blob = include_bytes!("../assets/themes/default/images/directory-64x64.png");
    write_bytes_to(dir, "directory-64x64.png", blob);
}

fn create_default_file_icon(dir: &PathBuf) {
    let blob = include_bytes!("../assets/themes/default/images/file-64x64.png");
    write_bytes_to(dir, "file-64x64.png", blob);
}

fn default_theme() {
    let mut dir = themes_dir();
    dir.push("default");
    dir.push("images");
    let r = create_dir_all(&dir);
    #[cfg_attr(tarpaulin, skip)]
        r.unwrap_or_else(|_| panic!("Cannot create themes config directory"));

    create_default_directory_icon(&dir);
    create_default_file_icon(&dir);
}

fn create_railscasts_directory_icon(dir: &PathBuf) {
    let blob = include_bytes!("../assets/themes/railscasts/images/directory-64x64.png");
    write_bytes_to(dir, "directory-64x64.png", blob);
}

fn create_railscasts_file_icon(dir: &PathBuf) {
    let blob = include_bytes!("../assets/themes/railscasts/images/file-64x64.png");
    write_bytes_to(dir, "file-64x64.png", blob);
}

fn railscasts_theme() {
    let mut dir = themes_dir();
    dir.push("railscasts");
    dir.push("images");
    let r = create_dir_all(&dir);
    #[cfg_attr(tarpaulin, skip)]
        r.unwrap_or_else(|_| panic!("Cannot create themes config directory"));

    create_railscasts_directory_icon(&dir);
    create_railscasts_file_icon(&dir);
}

#[cfg(test)]
mod tests {
    use super::*;
    use uuid::Uuid;
    use std::fs::create_dir_all;
    use std::env::set_var;
    use std::path::{Path, PathBuf};

    #[cfg(test)]
    fn join(a: String, b: String) -> String {
        vec![a, b].join("/")
    }

    #[test]
    fn assert_create() {
        let uniq = Uuid::new_v4();
        let test_path = join("/tmp".to_owned(), uniq.to_string());
        create_dir_all(test_path.clone()).unwrap();
        set_var("XDG_CONFIG_HOME", test_path.as_str());
        let themes_dir = join(test_path.clone(), "rider/themes".to_owned());
        assert_eq!(Path::new(join(themes_dir.clone(), "railscasts/images/directory-64x64.png".to_owned()).as_str()).exists(), false);
        assert_eq!(Path::new(join(themes_dir.clone(), "railscasts/images/file-64x64.png".to_owned()).as_str()).exists(), false);
        assert_eq!(Path::new(join(themes_dir.clone(), "default/images/directory-64x64.png".to_owned()).as_str()).exists(), false);
        assert_eq!(Path::new(join(themes_dir.clone(), "default/images/file-64x64.png".to_owned()).as_str()).exists(), false);
        create();
        assert_eq!(Path::new(join(themes_dir.clone(), "railscasts/images/directory-64x64.png".to_owned()).as_str()).exists(), true);
        assert_eq!(Path::new(join(themes_dir.clone(), "railscasts/images/file-64x64.png".to_owned()).as_str()).exists(), true);
        assert_eq!(Path::new(join(themes_dir.clone(), "default/images/directory-64x64.png".to_owned()).as_str()).exists(), true);
        assert_eq!(Path::new(join(themes_dir.clone(), "default/images/file-64x64.png".to_owned()).as_str()).exists(), true);
    }

    #[test]
    fn assert_create_default_directory_icon() {
        let uniq = Uuid::new_v4();
        let test_path = join("/tmp".to_owned(), uniq.to_string());
        create_dir_all(test_path.clone()).unwrap();
        set_var("XDG_CONFIG_HOME", test_path.as_str());
        let file_path: String = join(test_path.clone(), "directory-64x64.png".to_owned());
        let dir: PathBuf = test_path.into();
        assert_eq!(Path::new(file_path.as_str()).exists(), false);
        create_default_directory_icon(&dir);
        assert_eq!(Path::new(file_path.as_str()).exists(), true);
    }

    #[test]
    fn assert_create_default_file_icon() {
        let uniq = Uuid::new_v4();
        let test_path = join("/tmp".to_owned(), uniq.to_string());
        create_dir_all(test_path.clone()).unwrap();
        let file_path: String = join(test_path.clone(), "file-64x64.png".to_owned());
        set_var("XDG_CONFIG_HOME", test_path.as_str());
        let dir: PathBuf = test_path.into();
        assert_eq!(Path::new(file_path.as_str()).exists(), false);
        create_default_file_icon(&dir);
        assert_eq!(Path::new(file_path.as_str()).exists(), true);
    }

    #[test]
    fn assert_create_railscasts_directory_icon() {
        let uniq = Uuid::new_v4();
        let test_path = join("/tmp".to_owned(), uniq.to_string());
        create_dir_all(test_path.clone()).unwrap();
        let file_path: String = join(test_path.clone(), "directory-64x64.png".to_owned());
        set_var("XDG_CONFIG_HOME", test_path.as_str());
        let dir: PathBuf = test_path.into();
        assert_eq!(Path::new(file_path.as_str()).exists(), false);
        create_railscasts_directory_icon(&dir);
        assert_eq!(Path::new(file_path.as_str()).exists(), true);
    }

    #[test]
    fn assert_create_railscasts_file_icon() {
        let uniq = Uuid::new_v4();
        let test_path = join("/tmp".to_owned(), uniq.to_string());
        create_dir_all(test_path.clone()).unwrap();
        let file_path: String = join(test_path.clone(), "file-64x64.png".to_owned());
        set_var("XDG_CONFIG_HOME", test_path.as_str());
        let dir: PathBuf = test_path.into();
        assert_eq!(Path::new(file_path.as_str()).exists(), false);
        create_railscasts_file_icon(&dir);
        assert_eq!(Path::new(file_path.as_str()).exists(), true);
    }
}
