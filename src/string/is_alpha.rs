/// Checks if the string contains only alphabetic characters.
///
/// # Arguments
///
/// * `s` - The string to check.
///
/// # Returns
///
/// Returns true if string contains only alphabetic characters, false if not.
///
/// # Examples
///
/// ```
/// use ruf::string;
///
/// assert_eq!(true, string::is_alpha("abc"));
///
/// assert_eq!(true, string::is_alpha("żółć"));
///
/// assert_eq!(true, string::is_alpha("你好"));
///
/// assert_eq!(false, string::is_alpha("a̐éö̲"));
///
/// ```

pub fn is_alpha(s: &str) -> bool {
    return s.chars().all(|c| c.is_alphabetic());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_alpha() {
        assert_eq!(true, is_alpha("foo"));
        assert_eq!(true, is_alpha("żółć"));
        assert_eq!(true, is_alpha("你好"));
        assert_eq!(true, is_alpha(""));

        assert_eq!(false, is_alpha("a̐éö̲"));
        assert_eq!(false, is_alpha("café"));
        assert_eq!(false, is_alpha("foo1"));
    }
}
