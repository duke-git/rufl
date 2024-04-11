use std::fs::{self, File};
use std::io::Error;
use std::path::Path;

/// Creates a file in path and returns it.
///
/// # Arguments
///
/// * `file_path` - The path of file to create.
///
/// # Returns
///
/// Returns the created file and error is exists.
///
/// # Examples
///
/// ```
/// use std::fs;
/// use std::path::Path;
/// use ruf::file;
///
/// let path = Path::new("./a/test.txt");
/// let result = file::create_file(path);
///
/// assert_eq!(false, result.is_err());
/// assert_eq!(true, path.exists());
///
/// let _ = fs::remove_file(path);
/// let _ = fs::remove_dir("./a");
///
/// ```

pub fn create_file<P: AsRef<Path>>(file_path: P) -> Result<File, Error> {
    if let Some(path) = file_path.as_ref().parent() {
        fs::create_dir_all(path)?;
    }

    File::create(file_path).map_err(Into::into)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_file() {
        let path = Path::new("./a/test.txt");
        let result = create_file(path);
        assert_eq!(false, result.is_err());
        assert_eq!(true, path.exists());

        let _ = fs::remove_file(path);
        let _ = fs::remove_dir("./a");
    }
}
