/// Converts string to kebab case.
///
/// # Arguments
///
/// * `s` - The string to convert.
///
/// # Returns
///
/// Returns the kebab cased string.
///
/// # Examples
///
/// ```
/// use ruf::string;
///
/// assert_eq!("foo-bar", string::kebab_case("Foo Bar"));
///
/// assert_eq!("foo-bar", string::kebab_case("fooBar".to_string()));
///
/// assert_eq!("foo-bar", string::kebab_case("__FOO_BAR__"));
///
/// ```

pub fn kebab_case(s: impl AsRef<str>) -> String {
    match s.as_ref().len() {
        0 => s.as_ref().to_string(),
        _ => crate::string::split_words(s)
            .into_iter()
            .map(|item| item.to_lowercase())
            .collect::<Vec<String>>()
            .join("-"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_kebab_case() {
        assert_eq!("foo-bar", kebab_case("Foo Bar"));
        assert_eq!("foo-bar", kebab_case("fooBar"));
        assert_eq!("foo-bar", kebab_case("__FOO_BAR__"));
        assert_eq!("foo-bar-123", kebab_case("foo_bar123"));
        assert_eq!("foo-1-1-bar", kebab_case("Foo-#1😄$_%^&*(1bar"));
    }
}
