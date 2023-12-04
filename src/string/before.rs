/// Returns the substring before the first occurrence of a specified `substr` in the source string.
///
/// # Arguments
///
/// * `s` - The input string to perform before operation.
/// * `substr` - The substring to look for first occurrence in `s`.
///
/// # Returns
///
/// Returns the substring before the first occurrence of `substr` in `s`.
///
/// # Examples
///
/// ```
/// use ruf::string;
///
/// let foo = string::before("foo", "f");
/// assert_eq!("", foo);
///
/// let bar = string::before("bar", "a");
/// assert_eq!("b", bar);
///
/// let boo = string::before("boo", "c");
/// assert_eq!("boo", boo);
///
/// ```

pub fn before(s: impl AsRef<str>, substr: &str) -> String {
    match s.as_ref().find(substr) {
        Some(index) => s.as_ref()[..index].to_string(),
        None => s.as_ref().to_string(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_before() {
        assert_eq!("", before("foo", ""));
        assert_eq!("foo", before("foo", "a"));
        assert_eq!("", before("foo", "foo"));
        assert_eq!("foo", before("foo/bar", "/"));

        assert_eq!("rust", before("rust你好c++你好", "你好"));
        assert_eq!("Jsem ", before("Jsem počítačový programátor", "počítačový"));
    }
}
