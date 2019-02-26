use std::fs::File;
use std::io::Write;
use std::path::PathBuf;

pub fn write_bytes_to(dir: &PathBuf, file: &str, blob: &[u8]) -> std::io::Result<()> {
    let mut path = dir.clone();
    path.push(file);
    let mut f = File::create(path.to_str().unwrap())?;
    f.write(blob)?;
    f.flush()?;
    Ok(())
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
        let res = write_bytes_to(&test_dir, file_name.as_str(), blob.as_slice());
        assert_eq!(res.is_ok(), true);

        let mut test_file_path = test_dir.clone();
        test_file_path.push(file_name);
        let file_path = Path::new(&test_file_path);
        assert_eq!(file_path.exists(), true);
    }
}
