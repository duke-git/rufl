/// Pads string on the left and right sides if it's shorter than length. Padding characters are truncated if they can't be evenly divided by length.
///
/// # Arguments
///
/// * `s` - The string to pad.
/// * `length` - The padding length.
/// * `pad_with` - The string used as padding.
///
/// # Returns
///
/// Returns the padded string.
///
/// # Examples
///
/// ```
/// use ruf::string;
///
/// let foo = string::pad("foo", 3, "*");
/// assert_eq!("foo", foo);
///
/// let bar = string::pad("foo", 6, "**");
/// assert_eq!("*foo**", bar);
///
/// let boo = string::pad("foo", 8, "********");
/// assert_eq!("**foo***", boo);
///
/// ```

pub fn pad(s: impl AsRef<str>, length: usize, pad_with: &str) -> String {
    if pad_with.len() == 0 {
        return s.as_ref().to_string();
    }

    let pad_len = (length as f64 - s.as_ref().len() as f64) / 2 as f64;

    let repeat_count = (pad_len / pad_with.len() as f64) as f64;
    let repeat_count = repeat_count.ceil() as usize;

    let left_part: &str = &(pad_with.repeat(repeat_count)[..(pad_len.floor() as usize)]);
    let right_part: &str = &(pad_with.repeat(repeat_count)[..(pad_len.ceil() as usize)]);

    format!("{}{}{}", left_part, s.as_ref().to_string(), right_part)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pad() {
        assert_eq!("*foo**", pad("foo", 6, "*"));
        assert_eq!("*foo**", pad("foo", 6, "**"));
        assert_eq!("*foo**", pad("foo", 6, "******"));
        assert_eq!("_-foo_-_", pad("foo", 8, "_-"));
        assert_eq!("1foo12", pad("foo", 6, "12345678"));
    }
}
