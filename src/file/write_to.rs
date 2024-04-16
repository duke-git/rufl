use std::fs::{File, OpenOptions};
use std::io::{Error, Write};
use std::path::Path;

/// Write data to file, if file isn't exist, create it.
///
/// # Arguments
///
/// * `path` - The path of file to write data.
///
/// # Returns
///
/// Returns result error if exits .
///
/// # Examples
///
/// ```
/// use std::fs;
/// use rufl::file;
///
/// let result = file::write_to("./src/file/test_write_to.txt", "hello".as_bytes(), false);
///
/// match result {
///    Ok(()) => {
///         let _ = fs::remove_file("./src/file/test_write_to.txt");
///    }
///    Err(err) => {
///         println!("Error: {}", err);
///    }
/// }
///
/// ```

pub fn write_to<P: AsRef<Path>>(path: P, data: &[u8], append: bool) -> Result<(), Error> {
    let mut file = if append {
        OpenOptions::new().append(true).create(true).open(path)?
    } else {
        File::create(path)?
    };

    file.write_all(data)?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::file::read_to_string;
    use std::fs;

    #[test]
    fn test_write_to() {
        let result = write_to("./src/file/test_write_to.txt", "hello".as_bytes(), false);

        match result {
            Ok(()) => {
                let read_file = read_to_string("./src/file/test_write_to.txt");
                match read_file {
                    Ok(content) => {
                        assert_eq!("hello", content);
                        let _ = fs::remove_file("./src/file/test_write_to.txt");
                    }
                    Err(err) => {
                        println!("Error: {}", err);
                    }
                }
            }
            Err(err) => {
                println!("Error: {}", err);
            }
        }
    }
}
