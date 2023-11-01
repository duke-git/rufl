/// Removes the specified substring which last occurrence in the source string.
///
/// # Arguments
///
/// * `s` - The input string to perform remove.
/// * `substr` - The substring to be removed.
///
/// # Returns
///
/// Returns string after being removed.
///
/// # Examples
///
/// ```
/// use ruf::string;
///
/// let foo = string::remove_last("abca", "a");
/// assert_eq!("abc".to_string(), foo);
///
/// let bar = string::remove_last("abc", "d");
/// assert_eq!("abc".to_string(), bar);
///
///
/// ```

pub fn remove_last(s: impl AsRef<str>, substr: &str) -> String {
    match s.as_ref().rfind(substr) {
        Some(i) => s.as_ref()[0..i].to_string() + &s.as_ref()[i + substr.len()..].to_string(),
        None => s.as_ref().to_string(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_remove_last() {
        assert_eq!("abc".to_string(), remove_last("abc", ""));
        assert_eq!("abc".to_string(), remove_last("abac", "a"));
        assert_eq!("abc".to_string(), remove_last("abc", "d"));
    }
}
