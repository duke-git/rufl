/// Split the target string into an array of graphemes.
///
///
/// # Arguments
///
/// * `s` - The string to split into characters.
///
/// # Returns
///
/// Returns a vector containing the characters extracted from the input string.
///
/// # Example
///
/// ```rust
/// use ruf::string;
///
/// assert_eq!(vec!["a̐", "é", "ö̲"], string::split_graphemes("a̐éö̲"));
///
/// assert_eq!(vec!["h", "e", "l", "l", "o"], string::split_graphemes("hello"));
///
/// assert_eq!(vec!["你", "好"], string::split_graphemes("你好"));
///
/// ```

pub fn split_graphemes(s: &str) -> Vec<&str> {
    if s.is_empty() {
        return vec![""];
    }

    unicode_segmentation::UnicodeSegmentation::graphemes(s, true).collect::<Vec<&str>>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_split_graphemes() {
        assert_eq!(vec![""], split_graphemes(""));
        assert_eq!(vec!["h", "e", "l", "l", "o"], split_graphemes("hello"));
        assert_eq!(vec!["a̐", "é", "ö̲"], split_graphemes("a̐éö̲"));
        assert_eq!(vec!["你", "好"], split_graphemes("你好"));
    }
}
