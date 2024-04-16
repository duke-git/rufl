/// Converts string to pascal case.
///
/// # Arguments
///
/// * `s` - The string to convert.
///
/// # Returns
///
/// Returns the pascal cased string.
///
/// # Examples
///
/// ```
/// use rufl::string;
///
/// assert_eq!("FooBar", string::pascal_case("Foo Bar"));
///
/// assert_eq!("FooBar", string::pascal_case("fooBar".to_string()));
///
/// assert_eq!("FooBar", string::pascal_case("__FOO_BAR__"));
///
/// ```

pub fn pascal_case(s: impl AsRef<str>) -> String {
    match s.as_ref().len() {
        0 => s.as_ref().to_string(),
        _ => crate::string::split_words(s)
            .into_iter()
            .map(|item| crate::string::capitalize(item))
            .collect::<Vec<String>>()
            .join(""),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pascal_case() {
        assert_eq!("FooBar", pascal_case("fooBar"));
        assert_eq!("FooBar", pascal_case("Foo Bar"));
        assert_eq!("FooBar", pascal_case("__FOO_BAR__"));
        assert_eq!("FooBar123", pascal_case("foo-bar123"));
        assert_eq!("Foo11Bar", pascal_case("Foo-#1😄$_%^&*(1bar"));
    }
}
