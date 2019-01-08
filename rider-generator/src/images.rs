use rider_config::directories::*;
use std::fs;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;

pub fn create() {
    default_theme();
    railscasts_theme();
}

fn write_bytes_to(dir: &PathBuf, file: &str, blob: &[u8]) {
    let mut path = dir.clone();
    path.push(file);
    let r = File::create(path.to_str().unwrap());
    #[cfg_attr(tarpaulin, skip)]
    let mut f = r.unwrap_or_else(|e| panic!(e));
    let r = f.write(blob);
    #[cfg_attr(tarpaulin, skip)]
    r.unwrap_or_else(|e| panic!(e));
    let r = f.flush();
    #[cfg_attr(tarpaulin, skip)]
    r.unwrap_or_else(|e| panic!(e));
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
    let r = fs::create_dir_all(&dir);
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
    let r = fs::create_dir_all(&dir);
    #[cfg_attr(tarpaulin, skip)]
    r.unwrap_or_else(|_| panic!("Cannot create themes config directory"));

    create_railscasts_directory_icon(&dir);
    create_railscasts_file_icon(&dir);
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env::temp_dir;
    use std::path::Path;
    use uuid::Uuid;

    #[test]
    fn must_create_file() {
        let test_dir = temp_dir();
        let file_name = Uuid::new_v4().to_string();
        let blob: Vec<u8> = vec![1, 2, 3, 4];
        write_bytes_to(&test_dir, file_name.as_str(), blob.as_slice());

        let mut test_file_path = test_dir.clone();
        test_file_path.push(file_name);
        let file_path = Path::new(&test_file_path);
        assert_eq!(file_path.exists(), true);
    }
}
