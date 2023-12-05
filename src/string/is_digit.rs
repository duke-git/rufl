/// Checks if the string contains only digit characters.(0-9)
///
/// # Arguments
///
/// * `s` - The string to check.
///
/// # Returns
///
/// Returns true if string contains only digit characters, false if not.
///
/// # Examples
///
/// ```
/// use ruf::string;
///
/// assert_eq!(true, string::is_digit("0"));
///
/// assert_eq!(true, string::is_digit("123"));
///
/// assert_eq!(false, string::is_digit("0.1"));
///
/// assert_eq!(false, string::is_digit("-1"));
///
/// ```

pub fn is_digit(s: &str) -> bool {
    match s.len() {
        0 => false,
        _ => {
            let digits = "0123456789";
            for n in crate::string::split_graphemes(s).into_iter() {
                if !digits.contains(n) {
                    return false;
                }
            }
            true
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_digit() {
        assert_eq!(true, is_digit("0"));
        assert_eq!(true, is_digit("1"));
        assert_eq!(true, is_digit("123"));

        assert_eq!(false, is_digit(""));
        assert_eq!(false, is_digit("1.1"));
        assert_eq!(false, is_digit("1.1+e10"));
        assert_eq!(false, is_digit("1a"));
        assert_eq!(false, is_digit("+1"));
        assert_eq!(false, is_digit("-1"));
        assert_eq!(false, is_digit("0.0"));
        assert_eq!(false, is_digit("0xff"));
    }
}
