/// Hides some chars in source string and replace with speicfic substring.
///
/// # Arguments
///
/// * `s` - The string to hide.
/// * `start` - The position start to hide.
/// * `end` - The position end to hide (not include).
/// * `replace_str` - The string used to replace.
///
/// # Returns
///
/// Returns the hiden string.
///
/// # Examples
///
/// Basic usage:
///
/// ```
/// use ruf::string;
///
/// assert_eq!("13242658976", string::hide("13242658976", 3, 3, "*"));
///
/// assert_eq!("132*2658976", string::hide("13242658976", 3, 4, "*"));
///
/// assert_eq!("1324265****", string::hide("13242658976", 7, 100, "*"));
///
/// ```

pub fn hide(s: impl AsRef<str>, start: usize, end: usize, replace_str: &str) -> String {
    let str_size = s.as_ref().len();

    if start > str_size - 1 || start > end || replace_str == "" {
        return s.as_ref().to_string();
    }

    let mut end = end;
    if end > str_size {
        end = str_size;
    }

    let replace_str = replace_str.repeat(end - start);

    format!(
        "{}{}{}",
        crate::string::substring(&s, 0, start),
        replace_str,
        crate::string::substring(&s, end, str_size),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hide() {
        assert_eq!("13242658976", hide("13242658976", 3, 3, "*"));
        assert_eq!("132*2658976", hide("13242658976", 3, 4, "*"));
        assert_eq!("1324265****", hide("13242658976", 7, 100, "*"));
        assert_eq!("13242658976", hide("13242658976", 100, 103, "*"));
    }
}
