/// Returns the substring after the first occurrence of a specified `substr` in the source string.
///
/// # Examples
///
/// ```
/// use ruf::string;
///
/// let foo = string::after("foo", "f");
/// assert_eq!("oo".to_string(), foo);
///
/// let bar = string::after("bar".to_string(), "a");
/// assert_eq!("r".to_string(), bar);
///
/// let boo = string::after("boo", "c");
/// assert_eq!("boo".to_string(), boo);
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
        assert_eq!("foo".to_string(), after("foo", ""));
        assert_eq!("foo".to_string(), after("foo", "a"));
        assert_eq!("".to_string(), after("foo", "foo"));
        assert_eq!("bar/boo".to_string(), after("foo/bar/boo", "/"));
    }
}
