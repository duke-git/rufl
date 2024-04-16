use std::fs::OpenOptions;
use std::io::{Error, Write};
use std::path::Path;

/// Clear file content.
///
/// # Arguments
///
/// * `path` - The path of file to clear.
///
/// # Returns
///
/// Returns result or error exists.
///
/// # Examples
///
/// ```
/// use std::fs;
/// use std::path::Path;
/// use rufl::file;
///
/// let file_path = Path::new("./src/file/test.txt");
/// let _ = file::write_to(file_path, "hello".as_bytes(), false);
///
/// let _ = file::clear(file_path);
///
/// let _ = fs::remove_file(file_path);
///
/// ```
pub fn clear<P: AsRef<Path>>(path: P) -> Result<(), Error> {
    let mut file = OpenOptions::new().write(true).truncate(true).open(path)?;

    file.write("".as_bytes())?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::file::read_to_string;
    use crate::file::write_to;
    use std::fs;

    #[test]
    fn test_clear() {
        let file_path = "./src/file/test.txt";

        let _ = write_to(file_path, "hello".as_bytes(), false);

        match read_to_string(file_path) {
            Ok(content) => {
                assert_eq!("hello", content);
            }
            Err(err) => {
                println!("Error: {}", err);
            }
        }

        let _ = clear(file_path);

        match read_to_string(file_path) {
            Ok(content) => {
                assert_eq!("", content);
                let _ = fs::remove_file(file_path);
            }
            Err(err) => {
                println!("Error: {}", err);
            }
        }
    }
}
