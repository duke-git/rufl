/// Returns the substring before the last occurrence of a specified `substr` in the source string.
///
/// # Examples
///
/// ```
/// use ruf::string;
///
/// let foo = string::before_last("foo", "o");
/// assert_eq!("fo", foo);
///
/// let bar = string::before_last("bar", "a");
/// assert_eq!("b", bar);
///
/// let boo = string::before_last("boo", "c");
/// assert_eq!("boo", boo);
///
/// ```

pub fn before_last(s: impl AsRef<str>, substr: &str) -> String {
    match s.as_ref().rfind(substr) {
        Some(index) => s.as_ref()[..index].to_string(),
        None => s.as_ref().to_string(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_before_last() {
        assert_eq!("", before_last("foo", "foo"));
        assert_eq!("foo", before_last("foo", ""));
        assert_eq!("fo", before_last("foo", "o"));
        assert_eq!("foo/bar", before_last("foo/bar/boo", "/"));
    }
}
