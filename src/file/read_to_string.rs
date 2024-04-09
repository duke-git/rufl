use std::fs::File;
use std::io::Error;
use std::io::Read;

/// Reads file to string.
///
/// # Arguments
///
/// * `path` - The path of file to read.
///
/// # Returns
///
/// Returns file content string.
///
/// # Examples
///
/// ```
/// use ruf::file;
///
/// let result = file::read_to_string("./src/file/read_to_string.rs");
/// assert_eq!(true, result.is_ok());
///
/// match result {
///     Ok(content) => {
///         println!("Success, content is: {}", content);
///     }
///     Err(err) => {
///         println!("Error: {}", err);
///     }
/// }
///
/// ```

pub fn read_to_string(path: &str) -> Result<String, Error> {
    let mut file = File::open(path)?;

    let mut content = String::new();

    file.read_to_string(&mut content)?;

    Ok(content)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_read_to_string() {
        let result = read_to_string("./src/file/read_to_string.rs");
        assert_eq!(true, result.is_ok());

        match result {
            Ok(content) => {
                println!("Success, content is: {}", content);
            }
            Err(err) => {
                println!("Error: {}", err);
            }
        }
    }
}
