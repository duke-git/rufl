/// Returns the substring after the last occurrence of a specified `substr` in the source string.
///
/// # Arguments
///
/// * `s` - The input string to perform after_last operation.
/// * `substr` - The substring to look for last occurrence in `s`.
///
/// # Returns
///
/// Returns the substring after the last occurrence of `substr` in `s`.
///
/// # Examples
///
/// ```
/// use rufl::string;
///
/// let foo = string::after_last("foo", "o");
/// assert_eq!("", foo);
///
/// let bar = string::after_last("bar", "a");
/// assert_eq!("r", bar);
///
/// let boo = string::after_last("boo", "c");
/// assert_eq!("boo", boo);
///
/// ```

pub fn after_last(s: impl AsRef<str>, substr: &str) -> String {
    match s.as_ref().rfind(substr) {
        Some(index) => s.as_ref()[index + substr.len()..].to_string(),
        None => s.as_ref().to_string(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_after_last() {
        assert_eq!("", after_last("foo", ""));
        assert_eq!("", after_last("foo", "o"));
        assert_eq!("", after_last("foo", "foo"));
        assert_eq!("foo", after_last("foo", "a"));
        assert_eq!("boo", after_last("foo/bar/boo", "/"));

        assert_eq!("rust", after_last("你好c++你好rust", "你好"));
        assert_eq!(
            " programátor",
            after_last("Jsem počítačový programátor", "počítačový")
        );
    }
}
