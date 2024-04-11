use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::io::Error;
use std::path::Path;

/// Reads file and returns lines string vector.
///
/// # Arguments
///
/// * `path` - The path of file to read.
///
/// # Returns
///
/// Returns file content lines.
///
/// # Examples
///
/// ```
/// use ruf::file;
///
/// let result = file::read_to_lines("./src/file/read_to_lines.rs");
/// assert_eq!(true, result.is_ok());
///
/// match result {
///     Ok(lines) => {
///         for line in lines {
///            println!("file line is: {}", line);
///        }
///     }
///     Err(err) => {
///         println!("Error: {}", err);
///     }
/// }
///
/// ```

pub fn read_to_lines<P: AsRef<Path>>(path: P) -> Result<Vec<String>, Error> {
    let file = File::open(path)?;

    let reader = BufReader::new(file);

    let mut lines = Vec::new();
    for line in reader.lines() {
        lines.push(line.unwrap());
    }

    Ok(lines)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_read_to_lines() {
        let result = read_to_lines("./src/file/read_to_lines.rs");
        assert_eq!(true, result.is_ok());

        match result {
            Ok(lines) => {
                for line in lines {
                    println!("file line is: {}", line);
                }
            }
            Err(err) => {
                println!("Error: {}", err);
            }
        }
    }
}
