use std::fs::File;
use std::io::Error;
use std::io::Read;
use std::path::Path;

/// Reads file to buffer byte array.
///
/// # Arguments
///
/// * `path` - The path of file to read.
///
/// # Returns
///
/// Returns file content byte array.
///
/// # Examples
///
/// ```
/// use ruf::file;
///
/// let result = file::read_to_buffer("./src/file/read_to_buffer.rs");
/// assert_eq!(true, result.is_ok());
///
/// ```

pub fn read_to_buffer<P: AsRef<Path>>(path: P) -> Result<Vec<u8>, Error> {
    let mut file = File::open(path)?;

    let mut buffer = Vec::new();

    file.read_to_end(&mut buffer)?;

    Ok(buffer)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_read_to_buffer() {
        let result = read_to_buffer("./src/file/read_to_buffer.rs");
        assert_eq!(true, result.is_ok());

        match result {
            Ok(buffer) => {
                println!(
                    "Success, buffer string is: {}",
                    String::from_utf8_lossy(&buffer)
                );
            }
            Err(err) => {
                println!("Error: {}", err);
            }
        }
    }
}
