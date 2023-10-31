/// Converts string to camel case.
///
/// # Arguments
///
/// * `s` - The string to convert.
///
/// # Returns
///
/// Returns the camel cased string.
///
/// # Examples
///
/// ```
/// use ruf::string;
///
/// assert_eq!("fooBar", string::camel_case("Foo Bar"));
///
/// assert_eq!("fooBar", string::camel_case("fooBar".to_string()));
///
/// assert_eq!("fooBar", string::camel_case("__FOO_BAR__"));
///
/// ```

pub fn camel_case(s: impl AsRef<str>) -> String {
    match s.as_ref().len() {
        0 => s.as_ref().to_string(),
        _ => crate::string::split_words(s)
            .into_iter()
            .enumerate()
            .map(|(i, item)| match i {
                0 => item.to_lowercase(),
                _ => crate::string::capitalize(item),
            })
            .collect::<Vec<String>>()
            .join(""),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_camel_case() {
        assert_eq!("fooBar", camel_case("fooBar"));
        assert_eq!("fooBar", camel_case("Foo Bar"));
        assert_eq!("fooBar", camel_case("__FOO_BAR__"));
        assert_eq!("fooBar123", camel_case("foo-bar123"));
        assert_eq!("foo11Bar", camel_case("Foo-#1😄$_%^&*(1bar"));
    }
}
