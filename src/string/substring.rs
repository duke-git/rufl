///  Returns the part of target string from the start index up to and excluding the end index.
///
/// # Arguments
///
/// * `s` - The string to process.
/// * `start` - The index of the first character to include in the returned substring.
/// * `end` - The index of the first character to exclude from the returned substring.
///
/// # Returns
///
/// Returns the substring.
///
/// # Examples
///
/// Basic usage:
///
/// ```
/// use ruf::string;
///
/// assert_eq!("", string::substring("abcde", 0, 0));
///
/// assert_eq!("ab", string::substring("abcde", 0, 2));
///
/// assert_eq!("abcde", string::substring("abcde", 0, 10));
///
/// ```

pub fn substring(s: impl AsRef<str>, start: usize, end: usize) -> String {
    if start > end || start == end {
        return String::new();
    }

    // let end = s
    //     .as_ref()
    //     .chars()
    //     .map(|c| c.len_utf8())
    //     .take(end - start)
    //     .sum();

    // s.as_ref()[..end].to_string()

    s.as_ref().chars().skip(start).take(end - start).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_substring() {
        assert_eq!("", substring("abcde", 0, 0));
        assert_eq!("a", substring("abcde", 0, 1));
        assert_eq!("ab", substring("abcde", 0, 2));
        assert_eq!("abcde", substring("abcde", 0, 10));

        assert_eq!("żó", substring("żółć", 0, 2));
        assert_eq!("a̐", substring("a̐éö̲", 0, 2));
        assert_eq!("你好", substring("你好你好", 0, 2));

        assert_eq!("♥♥", substring("♥♥♥♥♥♥♥♥♥", 0, 2));
    }
}
