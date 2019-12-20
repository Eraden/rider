use dirs;
use std::env;
use std::path::PathBuf;

#[derive(Debug, Clone)]
pub struct Directories {
    pub log_dir: PathBuf,
    pub themes_dir: PathBuf,
    pub fonts_dir: PathBuf,
    pub config_dir: PathBuf,
    pub project_dir: PathBuf,
}

impl Directories {
    pub fn new(config_dir: Option<String>, project_dir: Option<String>) -> Self {
        let path = match config_dir {
            Some(s) => s,
            None => dirs::config_dir().unwrap().to_str().unwrap().to_owned(),
        };
        let mut config_dir = PathBuf::new();
        config_dir.push(path);
        config_dir.push("rider");

        let path = match project_dir {
            Some(s) => s,
            None => dirs::runtime_dir().unwrap().to_str().unwrap().to_owned(),
        };
        let mut project_dir = PathBuf::new();
        project_dir.push(path);
        project_dir.push(".rider");

        Self {
            log_dir: log_dir(&config_dir),
            themes_dir: themes_dir(&config_dir),
            fonts_dir: fonts_dir(&config_dir),
            config_dir,
            project_dir,
        }
    }
}

pub fn log_dir(config_dir: &PathBuf) -> PathBuf {
    let path = config_dir.to_str().unwrap().to_owned();
    let mut path_buf = PathBuf::new();
    path_buf.push(path);
    path_buf.push("log");
    path_buf
}

pub fn themes_dir(config_dir: &PathBuf) -> PathBuf {
    let path = config_dir.to_str().unwrap().to_owned();
    let mut path_buf = PathBuf::new();
    path_buf.push(path);
    path_buf.push("themes");
    path_buf
}

pub fn fonts_dir(config_dir: &PathBuf) -> PathBuf {
    let path = config_dir.to_str().unwrap().to_owned();
    let mut path_buf = PathBuf::new();
    path_buf.push(path);
    path_buf.push("fonts");
    path_buf
}

pub fn project_dir() -> PathBuf {
    let path = dirs::runtime_dir().unwrap().to_str().unwrap().to_owned();
    let mut path_buf = PathBuf::new();
    path_buf.push(path);
    path_buf.push(".rider");
    path_buf
}

#[cfg_attr(tarpaulin, skip)]
pub fn binaries_directory() -> Result<PathBuf, String> {
    let mut exec_dir = PathBuf::new();
    exec_dir.push(dirs::executable_dir().unwrap());
    let mut rider_editor = exec_dir.clone();
    rider_editor.push("rider-editor");
    if rider_editor.exists() {
        return Ok(exec_dir);
    }

    let path = dirs::runtime_dir().unwrap().to_str().unwrap().to_owned();
    let mut path_buf = PathBuf::new();
    path_buf.push(path.clone());
    path_buf.push("rider-editor");
    if path_buf.exists() {
        let mut path_buf = PathBuf::new();
        path_buf.push(path);
        return Ok(path_buf);
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

pub fn get_binary_path(name: &str) -> Result<String, String> {
    if cfg!(test) {
        use std::fs;
        println!("#[cfg(test)]");

        let mut current_dir = env::current_dir().unwrap();
        current_dir.push("target");
        current_dir.push("debug");
        let name = name.to_string().to_lowercase().replace("-", "_");
        println!("  name {:?}", name);
        current_dir.push(vec![name.clone(), "*".to_string()].join("-"));
        for entry in fs::read_dir(current_dir.to_str().unwrap()).unwrap() {
            if let Ok(entry) = entry {
                if let Ok(meta) = entry.metadata() {
                    if meta.is_file() && !entry.path().ends_with(".d") {
                        return Ok(entry.path().to_str().unwrap().to_string());
                    }
                }
            }
        }
        Err(format!("Cannot find {:?}", name))
    } else {
        println!("#[cfg(not(test))]");
        let r = binaries_directory();
        let mut binaries: PathBuf = r.unwrap_or_else(|e| panic!(e));
        binaries.push(name.to_string());
        println!("  name {}", name);
        match binaries.to_str() {
            Some(s) => Ok(s.to_owned()),
            _ => Err(format!("Cannot find {:?}", name)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::{Path, PathBuf};

    #[test]
    fn assert_log_dir() {
        let directories = Directories::new(Some("/tmp".to_owned()), None);
        let path = directories.log_dir.clone();
        let expected: PathBuf = Path::new("/tmp/rider/log").into();
        assert_eq!(path, expected);
    }

    #[test]
    fn assert_themes_dir() {
        let directories = Directories::new(Some("/tmp".to_owned()), None);
        let path = directories.themes_dir.clone();
        let expected: PathBuf = Path::new("/tmp/rider/themes").into();
        assert_eq!(path, expected);
    }

    #[test]
    fn assert_fonts_dir() {
        let directories = Directories::new(Some("/tmp".to_owned()), None);
        let path = directories.fonts_dir.clone();
        let expected: PathBuf = Path::new("/tmp/rider/fonts").into();
        assert_eq!(path, expected);
    }

    #[test]
    fn assert_config_dir() {
        let directories = Directories::new(Some("/tmp".to_owned()), None);
        let path = directories.config_dir.clone();
        let expected: PathBuf = Path::new("/tmp/rider").into();
        assert_eq!(path, expected);
    }
}
