/// Returns the substring after the first occurrence of a specified `substr` in the source string.
///
/// # Arguments
///
/// * `s` - The input string to perform after operation.
/// * `substr` - The substring to look for first occurrence in `s`.
///
/// # Returns
///
/// Returns the substring after the first occurrence of `substr` in `s`.
///
/// # Examples
///
/// ```
/// use ruf::string;
///
/// let foo = string::after("foo", "f");
/// assert_eq!("oo", foo);
///
/// let bar = string::after("bar", "a");
/// assert_eq!("r", bar);
///
/// let boo = string::after("boo", "c");
/// assert_eq!("boo", boo);
///
/// ```

pub fn after(s: impl AsRef<str>, substr: &str) -> String {
    match s.as_ref().find(substr) {
        Some(index) => s.as_ref()[index + substr.len()..].to_string(),
        None => s.as_ref().to_string(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_after() {
        assert_eq!("foo", after("foo", ""));
        assert_eq!("foo", after("foo", "a"));
        assert_eq!("", after("foo", "foo"));
        assert_eq!("bar/boo", after("foo/bar/boo", "/"));
    }
}
