use md5::{Digest, Md5};
use std::fs::File;
use std::io::Error;
use std::io::Read;
use std::path::Path;

/// Gets the md5 value of file.
///
/// # Arguments
///
/// * `path` - The path of file to get md5.
///
/// # Returns
///
/// Returns the md5 string value.
///
/// # Examples
///
/// ```
/// use ruf::file;
///
/// let result = file::get_md5("./src/file/get_md5.rs");
///
/// match result {
///   Ok(md5) => {
///      println!("md5 string is {}", md5)
///   }
///   Err(err) => {
///      println!("Error: {}", err);
///   }
/// }
///
/// ```

pub fn get_md5<P: AsRef<Path>>(file_path: P) -> Result<String, Error> {
    let mut file = File::open(file_path)?;
    let mut hasher = Md5::new();

    let mut buffer = [0; 8192];

    loop {
        let bytes_read = file.read(&mut buffer)?;
        if bytes_read == 0 {
            break;
        }

        hasher.update(&buffer[..bytes_read]);
    }

    let digest = hasher.finalize();

    let mut result = String::with_capacity(32);
    for byte in digest {
        result.push_str(&format!("{:02x}", byte));
    }

    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_md5() {
        let result = get_md5("./src/file/get_md5.rs");

        match result {
            Ok(md5) => {
                println!("md5 string is {}", md5)
            }
            Err(err) => {
                println!("Error: {}", err);
            }
        }
    }
}
