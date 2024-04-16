use std::path::Path;

/// Checks if file is symbol link file.
///
/// # Arguments
///
/// * `path` - The path of file to check.
///
/// # Returns
///
/// Returns true if file is symbol link, false or not.
///
/// # Examples
///
/// ```
/// use rufl::file;
///
/// let result = file::is_symlink("./src/file/is_symlink.rs");
/// assert_eq!(false, result);
///
/// ```

pub fn is_symlink<P: AsRef<Path>>(path: P) -> bool {
    path.as_ref()
        .symlink_metadata()
        .map(|meta| meta.file_type().is_symlink())
        .unwrap_or(false)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_symlink() {
        let result = is_symlink("./src/file/is_symlink.rs");
        assert_eq!(false, result);
    }
}
