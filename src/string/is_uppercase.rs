/// Checks if the string contains only uppercase unicode characters.
///
/// # Arguments
///
/// * `s` - The string to check.
///
/// # Returns
///
/// Returns true if string contains only uppercase unicode character, false if not.
///
/// # Examples
///
/// ```
/// use rufl::string;
///
/// assert_eq!(true, string::is_uppercase("FOO"));
///
/// assert_eq!(false, string::is_uppercase("你好"));
///
/// assert_eq!(false, string::is_uppercase("FOO1"));
///
/// assert_eq!(false, string::is_uppercase("a̐éö̲"));
///
/// ```

pub fn is_uppercase(s: &str) -> bool {
    match s.len() {
        0 => false,
        _ => {
            let mut is_all_uppercase = true;
            crate::string::split_graphemes(s)
                .into_iter()
                .for_each(|item| {
                    item.chars().for_each(|c| {
                        if !c.is_uppercase() {
                            is_all_uppercase = false;
                        }
                    })
                });
            is_all_uppercase
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_uppercase() {
        assert_eq!(true, is_uppercase("FOO"));
        assert_eq!(true, is_uppercase("ŻŻŻ"));

        assert_eq!(false, is_uppercase("你好"));
        assert_eq!(false, is_uppercase(""));
        assert_eq!(false, is_uppercase("FOO1"));
        assert_eq!(false, is_uppercase("Foo"));
        assert_eq!(false, is_uppercase("a̐éö̲"));
    }
}
