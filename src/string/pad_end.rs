/// Pads string on the right side if it's shorter than length. Padding str are truncated if they exceed length.
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
/// use rufl::string;
///
/// let foo = string::pad_end("foobar", 6, "*");
/// assert_eq!("foobar", foo);
///
/// let bar = string::pad_end("foobar", 10, "**");
/// assert_eq!("foobar****", bar);
///
/// ```

pub fn pad_end(s: impl AsRef<str>, length: usize, pad_with: &str) -> String {
    if s.as_ref().len() >= length || pad_with.len() == 0 {
        return s.as_ref().to_string();
    }

    let pad_len = length - s.as_ref().len();
    let pad_str = &pad_with.repeat(pad_len)[..pad_len];

    format!("{}{:pad_len$}", s.as_ref().to_string(), pad_str)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pad_end() {
        assert_eq!("foobar0000", pad_end("foobar", 10, "0"));
        assert_eq!("foobar    ", pad_end("foobar", 10, " "));
        assert_eq!("foobar", pad_end("foobar", 10, ""));
        assert_eq!("foobar", pad_end("foobar", 2, "0"));
        assert_eq!("foobar1", pad_end("foobar", 7, "12345678"));
        assert_eq!("foobar", pad_end("foobar", 1, ""));
        assert_eq!("foobar1234", pad_end("foobar", 10, "12345678"));
    }
}
