/// Pads string on the right side if it's shorter than length. Padding str are truncated if they exceed length.
///
/// # Examples
///
/// ```
/// use ruf::string;
///
/// let foo = string::pad_end("foobar", 6, "*");
/// assert_eq!("foobar".to_string(), foo);
///
/// let bar = string::pad_end("foobar".to_string(), 10, "**");
/// assert_eq!("foobar****".to_string(), bar);
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
        assert_eq!("foobar0000".to_string(), pad_end("foobar", 10, "0"));
        assert_eq!("foobar    ".to_string(), pad_end("foobar", 10, " "));
        assert_eq!("foobar".to_string(), pad_end("foobar", 10, ""));
        assert_eq!("foobar".to_string(), pad_end("foobar", 2, "0"));
        assert_eq!("foobar1".to_string(), pad_end("foobar", 7, "12345678"));
        assert_eq!("foobar", pad_end("foobar".to_string(), 1, ""));
        assert_eq!("foobar1234".to_string(), pad_end("foobar", 10, "12345678"));
    }
}