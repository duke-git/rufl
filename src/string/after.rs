/// Returns the substring after the first occurrence of a specified `substr` in the source string.
///
/// # Examples
///
/// ```
/// use ruf::string;
///
/// let foo = string::after("foo".to_string(), "f");
/// assert_eq!("oo".to_string(), foo);
///
///
/// let bar = string::after("bar".to_string(), "bar");
/// assert_eq!("".to_string(), bar);
///
/// ```

pub fn after(s: String, substr: &str) -> String {
    match s.find(substr) {
        Some(index) => s[index + substr.len()..].to_string(),
        None => s,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_after() {
        assert_eq!("foo".to_string(), after("foo".to_string(), ""));
        assert_eq!("foo".to_string(), after("foo".to_string(), "a"));
        assert_eq!("".to_string(), after("foo".to_string(), "foo"));
        assert_eq!("bar/boo".to_string(), after("foo/bar/boo".to_string(), "/"));
    }
}
