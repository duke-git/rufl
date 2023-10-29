/// Returns the substring before the first occurrence of a specified `substr` in the source string.
///
/// # Examples
///
/// ```
/// use ruf::string;
///
/// let foo = string::before("foo", "f");
/// assert_eq!("".to_string(), foo);
///
/// let bar = string::before("bar".to_string(), "a");
/// assert_eq!("b".to_string(), bar);
///
/// let boo = string::before("boo", "c");
/// assert_eq!("boo".to_string(), boo);
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
        assert_eq!("".to_string(), before("foo", ""));
        assert_eq!("foo".to_string(), before("foo", "a"));
        assert_eq!("".to_string(), before("foo", "foo"));
        assert_eq!("foo".to_string(), before("foo/bar", "/"));
    }
}
