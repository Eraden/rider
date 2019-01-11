use dirs;
use std::env;
use std::path::PathBuf;

pub fn log_dir() -> PathBuf {
    let mut log_dir = config_dir();
    log_dir.push("log");
    log_dir
}

pub fn themes_dir() -> PathBuf {
    let mut themes_dir = config_dir();
    themes_dir.push("themes");
    themes_dir
}

pub fn fonts_dir() -> PathBuf {
    let mut fonts_dir = config_dir();
    fonts_dir.push("fonts");
    fonts_dir
}

pub fn config_dir() -> PathBuf {
    let home_dir = dirs::config_dir().unwrap();
    let mut config_dir = home_dir.clone();
    config_dir.push("rider");
    config_dir
}

pub fn project_dir() -> PathBuf {
    let runtime = dirs::runtime_dir().unwrap();
    let mut project_dir = runtime.clone();
    project_dir.push(".rider");
    project_dir
}

pub fn binaries_directory() -> Result<PathBuf, String> {
    let runtime = dirs::runtime_dir().unwrap();
    let mut rider_editor = runtime.clone();
    rider_editor.push("rider-editor");
    if rider_editor.exists() {
        return Ok(runtime);
    }

    let mut current_dir = env::current_dir().unwrap();
    current_dir.push("target");
    current_dir.push("debug");
    let mut rider_editor = current_dir.clone();
    rider_editor.push("rider-editor");
    if rider_editor.exists() {
        return Ok(current_dir);
    }

    let executable = dirs::executable_dir().unwrap();
    let mut rider_editor = executable.clone();
    rider_editor.push("rider-editor");
    if rider_editor.exists() {
        return Ok(executable);
    }

    Err("Cannot find binaries!".to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env::{set_var, temp_dir};
    use std::path::{Path, PathBuf};

    #[test]
    fn assert_log_dir() {
        set_var("XDG_CONFIG_HOME", temp_dir());
        let path = log_dir();
        let expected: PathBuf = Path::new("/tmp/rider/log").into();
        assert_eq!(path, expected);
    }

    #[test]
    fn assert_themes_dir() {
        set_var("XDG_CONFIG_HOME", temp_dir());
        let path = themes_dir();
        let expected: PathBuf = Path::new("/tmp/rider/themes").into();
        assert_eq!(path, expected);
    }

    #[test]
    fn assert_fonts_dir() {
        set_var("XDG_CONFIG_HOME", temp_dir());
        let path = fonts_dir();
        let expected: PathBuf = Path::new("/tmp/rider/fonts").into();
        assert_eq!(path, expected);
    }

    #[test]
    fn assert_config_dir() {
        set_var("XDG_CONFIG_HOME", temp_dir());
        let path = config_dir();
        let expected: PathBuf = Path::new("/tmp/rider").into();
        assert_eq!(path, expected);
    }

}
