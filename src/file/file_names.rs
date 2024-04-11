use std::fs;
use std::path::Path;

/// Return all file names of specific directory path.
///
/// # Arguments
///
/// * `dir_path` - The path of directory to list files.
///
/// # Returns
///
/// Returns files name vector.
///
/// # Examples
///
/// ```
/// use ruf::file;
///
/// let file_names = file::file_names("./"); //project root dir
/// assert_eq!(6, file_names.len());
///
/// ```

pub fn file_names<P: AsRef<Path>>(dir_path: P) -> Vec<String> {
    let mut file_names: Vec<String> = Vec::new();

    if !dir_path.as_ref().exists() || !dir_path.as_ref().is_dir() {
        return file_names;
    }

    for entry in fs::read_dir(dir_path).unwrap() {
        let path = entry.unwrap().path();

        if path.is_file() {
            file_names.push(path.file_name().unwrap().to_str().unwrap().to_owned());
        }
    }

    file_names
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_file_names() {
        let files = file_names("./"); //project root dir
        assert_eq!(6, files.len());
    }
}
