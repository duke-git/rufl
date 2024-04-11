use std::fs;
use std::io::Error;
use std::path::Path;

/// Copys all directories in src path to dst path.
///
/// # Arguments
///
/// * `src_path` - The source path of directory to copy.
///
/// * `det_path` - The destination path of directory to copy to.
///
/// # Returns
///
/// Returns error if exists.
///
/// # Examples
///
/// ```
/// use std::fs;
/// use std::path::Path;
///
/// use ruf::file;
///
/// let dst_path = Path::new("./src/copyfile");
/// assert_eq!(false, dst_path.exists());
///
/// let result = file::copy_dirs("./src/file", dst_path.to_str().unwrap());
///
/// match result {
///    Ok(()) => {
///         assert_eq!(true, dst_path.exists());
///         let _ = fs::remove_dir_all(dst_path);
///    }
///    Err(err) => {
///         println!("Error: {}", err);
///    }
/// }
///
/// ```

pub fn copy_dirs<P: AsRef<Path>>(src_path: P, dst_path: P) -> Result<(), Error> {
    if !src_path.as_ref().exists() {
        return Err(Error::new(
            std::io::ErrorKind::NotFound,
            "Source directory not found",
        ));
    }

    fs::create_dir_all(&dst_path)?;

    for entry in fs::read_dir(src_path)? {
        let src = entry.unwrap().path();

        let mut dst = dst_path.as_ref().to_path_buf();
        dst.push(src.file_name().unwrap());

        if src.is_dir() {
            return copy_dirs(&src, &dst);
        } else {
            let _ = fs::copy(&src, &dst);
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_copy_dirs() {
        let dst_path = Path::new("./src/copyfile");
        assert_eq!(false, dst_path.exists());

        let result = copy_dirs("./src/file", dst_path.to_str().unwrap());

        match result {
            Ok(()) => {
                assert_eq!(true, dst_path.exists());
                let _ = fs::remove_dir_all(dst_path);
            }
            Err(err) => {
                println!("Error: {}", err);
            }
        }
    }
}
