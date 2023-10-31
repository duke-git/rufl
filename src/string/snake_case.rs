/// Converts string to snake case.
///
/// # Arguments
///
/// * `s` - The string to convert.
///
/// # Returns
///
/// Returns the snake cased string.
///
/// # Examples
///
/// ```
/// use ruf::string;
///
/// assert_eq!("foo_bar", string::snake_case("Foo Bar"));
///
/// assert_eq!("foo_bar", string::snake_case("fooBar".to_string()));
///
/// assert_eq!("foo_bar", string::snake_case("__FOO_BAR__"));
///
/// ```

pub fn snake_case(s: impl AsRef<str>) -> String {
    match s.as_ref().len() {
        0 => s.as_ref().to_string(),
        _ => crate::string::split_words(s)
            .into_iter()
            .map(|item| item.to_lowercase())
            .collect::<Vec<String>>()
            .join("_"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_snake_case() {
        assert_eq!("foo_bar", snake_case("Foo Bar"));
        assert_eq!("foo_bar", snake_case("fooBar"));
        assert_eq!("foo_bar", snake_case("__FOO_BAR__"));
        assert_eq!("foo_bar_123", snake_case("foo-bar123"));
        assert_eq!("foo_1_1_bar", snake_case("Foo-#1😄$_%^&*(1bar"));
    }
}
