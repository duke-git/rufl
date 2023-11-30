/// Converts string to title case.
///
/// # Arguments
///
/// * `s` - The string to convert.
///
/// # Returns
///
/// Returns the title cased string.
///
/// # Examples
///
/// ```
/// use ruf::string;
///
/// assert_eq!("Foo Bar", string::title_case("Foo Bar"));
///
/// assert_eq!("Foo Bar", string::title_case("fooBar".to_string()));
///
/// assert_eq!("Foo Bar", string::title_case("__FOO_BAR__"));
///
/// ```

pub fn title_case(s: impl AsRef<str>) -> String {
    match s.as_ref().len() {
        0 => s.as_ref().to_string(),
        _ => crate::string::split_words(s)
            .into_iter()
            .map(|item| crate::string::capitalize(item))
            .collect::<Vec<String>>()
            .join(" "),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_title_case() {
        assert_eq!("Foo Bar", title_case("Foo Bar"));
        assert_eq!("Foo Bar", title_case("fooBar"));
        assert_eq!("Foo Bar", title_case("__FOO_BAR__"));
        assert_eq!("Foo Bar 123", title_case("foo_bar123"));
        assert_eq!("Foo 1 1 Bar", title_case("Foo-#1😄$_%^&*(1bar"));
    }
}
