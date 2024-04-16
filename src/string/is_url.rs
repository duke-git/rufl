use lazy_static::lazy_static;
use regex::Regex;

/// Checks if the string is a valid url.
///
/// # Arguments
///
/// * `s` - The string to check.
///
/// # Returns
///
/// Returns true if string is a valid url, false if not.
///
/// # Examples
///
/// ```
/// use rufl::string;
///
/// assert_eq!(true, string::is_url("http://www.abc.com"));
///
/// assert_eq!(false, string::is_url("http://abc"));
///
/// ```

pub fn is_url(s: impl AsRef<str>) -> bool {
    URL_REGEX.is_match(s.as_ref())
}

lazy_static! {
    static ref URL_REGEX: Regex = Regex::new(
        r"^(http|ftp|https)://([\w_-]+(?:(?:\.[\w_-]+)+))([\w.,@?^=%&amp;:/~+#-]*[\w@?^=%&amp;/~+#-])?"
    )
    .unwrap();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_url() {
        assert_eq!(true, is_url("http://www.abc.com"));
        assert_eq!(true, is_url("http://www.abc.com:80"));
        assert_eq!(true, is_url("https://www.123.com"));
        assert_eq!(true, is_url("https://abc.com"));
        assert_eq!(true, is_url("ftp://ftpserver.com:8090"));

        assert_eq!(false, is_url(""));
        assert_eq!(false, is_url("http://abc"));
        assert_eq!(false, is_url("https://abc:80"));
    }
}
