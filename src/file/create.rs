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
/// let path = Path::new("./src/file/test_create.txt");
/// assert_eq!(false, path.exists());
///
/// let result = file::create(path);
///
/// match result {
///   Ok(file) => {
///      assert_eq!(true, path.exists());
///      let _ = fs::remove_file("./src/file/test_create.txt");
///   }
///   Err(err) => {
///      println!("Error: {}", err);
///   }
/// }
///
/// ```

pub fn create<P: AsRef<Path>>(file_path: P) -> Result<File, Error> {
    if let Some(path) = file_path.as_ref().parent() {
        fs::create_dir_all(path)?;
    }

    File::create(file_path).map_err(Into::into)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create() {
        let path = Path::new("./src/file/test_create.txt");
        assert_eq!(false, path.exists());

        let result = create(path);

        match result {
            Ok(_) => {
                assert_eq!(true, path.exists());
                let _ = fs::remove_file("./src/file/test_create.txt");
            }
            Err(err) => {
                println!("Error: {}", err);
            }
        }
    }
}
