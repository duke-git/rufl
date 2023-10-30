/// Pads string on the left side if it's shorter than length. Padding str are truncated if they exceed length.
///
/// # Examples
///
/// ```
/// use ruf::string;
///
/// let foo = string::pad_start("foobar", 6, "*");
/// assert_eq!("foobar", foo);
///
/// let bar = string::pad_start("foobar", 10, "**");
/// assert_eq!("****foobar", bar);
///
/// ```

pub fn pad_start(s: impl AsRef<str>, length: usize, pad_with: &str) -> String {
    if s.as_ref().len() >= length || pad_with.len() == 0 {
        return s.as_ref().to_string();
    }

    let pad_len = length - s.as_ref().len();
    let pad_str = &pad_with.repeat(pad_len)[..pad_len];

    format!("{:pad_len$}{}", pad_str, s.as_ref().to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pad_start() {
        assert_eq!("0000foobar", pad_start("foobar", 10, "0"));
        assert_eq!("    foobar", pad_start("foobar", 10, " "));
        assert_eq!("foobar", pad_start("foobar", 10, ""));
        assert_eq!("foobar", pad_start("foobar", 2, "0"));
        assert_eq!("1foobar", pad_start("foobar", 7, "12345678"));
        assert_eq!("foobar", pad_start("foobar", 1, ""));

        assert_eq!("1234foobar", pad_start("foobar", 10, "12345678"));
    }
}
