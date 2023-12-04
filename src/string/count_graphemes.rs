/// Returns the graphemes count in target string.
///
///
/// # Arguments
///
/// * `s` - The string to count graphemes.
///
/// # Returns
///
/// Returns the graphemes count of target string.
///
/// # Example
///
/// ```rust
/// use ruf::string;
///
/// assert_eq!(5, string::count_graphemes("hello"));
///
/// assert_eq!(2, string::count_graphemes("你好"));
///
/// assert_eq!(3, string::count_graphemes("a̐éö̲"));
/// ```

pub fn count_graphemes(s: impl AsRef<str>) -> usize {
    match s.as_ref().len() {
        0 => 0,
        _ => crate::string::split_graphemes(s.as_ref()).len(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_count_graphemes() {
        assert_eq!(0, count_graphemes(""));
        assert_eq!(5, count_graphemes("hello"));
        assert_eq!(2, count_graphemes("你好"));
        assert_eq!(3, count_graphemes("a̐éö̲"));
    }
}
