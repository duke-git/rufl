use std::cmp::Ordering;

/// Checks whether a string starts with the specified prefix at `offset` position.
///
/// # Arguments
///
/// * `s` - The input string to perform remove.
/// * `prefix` - The prefix to find.
/// * `offset` - The position to start find.
///
/// # Returns
///
/// Returns true if source string starts with the prefix at offset position.
///
/// # Examples
///
/// ```
/// use ruf::string;
///
/// assert_eq!(true, string::starts_with_offset("abab", "a", 0));
///
/// assert_eq!(false, string::starts_with_offset("abab", "a", 1));
///
/// assert_eq!(true, string::starts_with_offset("abab", "a", 2));
///
/// assert_eq!(false, string::starts_with_offset("abab", "a", 6));
///
/// ```

pub fn starts_with_offset(s: impl AsRef<str>, prefix: &str, offset: usize) -> bool {
    match offset.cmp(&s.as_ref().len()) {
        Ordering::Greater => false,
        _ => (&s.as_ref()[offset..]).starts_with(prefix),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_starts_with_offset() {
        assert_eq!(true, starts_with_offset("abab", "a", 0));
        assert_eq!(false, starts_with_offset("abab", "a", 1));
        assert_eq!(true, starts_with_offset("abab", "a", 2));
        assert_eq!(false, starts_with_offset("abab", "a", 6));
    }
}
