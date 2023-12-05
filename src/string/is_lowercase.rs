/// Checks if the string contains only lowercase unicode characters.
///
/// # Arguments
///
/// * `s` - The string to check.
///
/// # Returns
///
/// Returns true if string contains only lowercase unicode character, false if not.
///
/// # Examples
///
/// ```
/// use ruf::string;
///
/// assert_eq!(true, string::is_lowercase("abc"));
///
/// assert_eq!(true, string::is_lowercase("żółć"));
///
/// assert_eq!(false, string::is_lowercase("你好"));
///
/// assert_eq!(false, string::is_lowercase("a̐éö̲"));
///
/// ```

pub fn is_lowercase(s: impl AsRef<str>) -> bool {
    match s.as_ref().len() {
        0 => false,
        _ => {
            let mut is_all_lowercase = true;
            crate::string::split_graphemes(s.as_ref())
                .into_iter()
                .for_each(|item| {
                    item.chars().for_each(|c| {
                        if !c.is_lowercase() {
                            is_all_lowercase = false;
                        }
                    })
                });
            is_all_lowercase
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_lowercase() {
        assert_eq!(true, is_lowercase("abc"));
        assert_eq!(true, is_lowercase("żółć"));

        assert_eq!(false, is_lowercase("a̐éö̲"));
        assert_eq!(false, is_lowercase("你好"));
        assert_eq!(false, is_lowercase(""));
        assert_eq!(false, is_lowercase("foo1"));
        assert_eq!(false, is_lowercase("Foo"));
    }
}
