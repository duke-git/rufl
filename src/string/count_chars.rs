/// Returns the characters count in target string.
///
///
/// # Arguments
///
/// * `s` - The string to count characters.
///
/// # Returns
///
/// Returns the characters count of target string.
///
/// # Example
///
/// ```rust
/// use rufl::string;
///
/// assert_eq!(5, string::count_chars("hello"));
///
/// assert_eq!(2, string::count_chars("你好"));
///
/// assert_eq!(7, string::count_chars("a̐éö̲"));
/// ```

pub fn count_chars(s: impl AsRef<str>) -> usize {
    s.as_ref().chars().count()
    // match s.as_ref().len() {
    //     0 => 0,
    //     _ => crate::string::split_chars(s.as_ref()).len(),
    // }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_count_chars() {
        assert_eq!(0, count_chars(""));
        assert_eq!(5, count_chars("hello"));
        assert_eq!(2, count_chars("你好"));
        assert_eq!(7, count_chars("a̐éö̲"));
    }
}
