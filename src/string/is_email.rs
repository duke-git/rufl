use lazy_static::lazy_static;
use regex::Regex;

/// Checks if the string is a valid email address.
///
/// # Arguments
///
/// * `s` - The string to check.
///
/// # Returns
///
/// Returns true if string is a valid email address, false if not.
///
/// # Examples
///
/// ```
/// use rufl::string;
///
/// assert_eq!(true, string::is_email("123@abc.com"));
///
/// assert_eq!(false, string::is_email("123@abc"));
///
/// ```

pub fn is_email(s: impl AsRef<str>) -> bool {
    EMAIL_REGEX.is_match(s.as_ref())
}

lazy_static! {
    static ref EMAIL_REGEX: Regex = Regex::new(
        r"^([a-z0-9_+]([a-z0-9_+.]*[a-z0-9_+])?)@([a-z0-9]+([\-\.]{1}[a-z0-9]+)*\.[a-z]{2,6})"
    )
    .unwrap();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_email() {
        assert_eq!(true, is_email("123@abc.com"));
        assert_eq!(true, is_email("1_23@abc.com"));

        assert_eq!(false, is_email("email@[127.0.0.1]"));
        assert_eq!(false, is_email(""));
        assert_eq!(false, is_email("123@abc"));
        assert_eq!(false, is_email("123@abc."));
        assert_eq!(false, is_email("123 @abc.com"));
        assert_eq!(false, is_email("123@@abc.com"));
    }
}
