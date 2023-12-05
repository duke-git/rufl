/// Checks if the string contains only alphabetic or numeric characters.
///
/// # Arguments
///
/// * `s` - The string to check.
///
/// # Returns
///
/// Returns true if string contains only alphabetic or numeric characters, false if not.
///
/// # Examples
///
/// ```
/// use ruf::string;
///
/// assert_eq!(true, string::is_alphanumeric("abc123"));
///
/// assert_eq!(true, string::is_alphanumeric("żółć123"));
///
/// assert_eq!(true, string::is_alphanumeric("你好1"));
///
/// assert_eq!(false, string::is_alphanumeric("a̐éö̲123"));
///
/// ```

pub fn is_alphanumeric(s: impl AsRef<str>) -> bool {
    return s.as_ref().chars().all(|c| c.is_alphanumeric());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_alphanumeric() {
        assert_eq!(true, is_alphanumeric("foo1"));
        assert_eq!(true, is_alphanumeric("FOO1"));
        assert_eq!(true, is_alphanumeric("żółć123"));
        assert_eq!(true, is_alphanumeric("你好123"));
        assert_eq!(true, is_alphanumeric(""));

        assert_eq!(false, is_alphanumeric("a̐éö̲2"));
        assert_eq!(false, is_alphanumeric("café1"));
    }
}
