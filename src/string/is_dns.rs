use lazy_static::lazy_static;
use regex::Regex;

/// Checks if the string is a valid domain name.
///
/// # Arguments
///
/// * `s` - The string to check.
///
/// # Returns
///
/// Returns true if string is a valid domain name, false if not.
///
/// # Examples
///
/// ```
/// use rufl::string;
///
/// assert_eq!(true, string::is_dns("abc.com"));
///
/// assert_eq!(true, string::is_dns("123.cn"));
///
/// assert_eq!(false, string::is_dns("abc@123.com"));
///
/// ```

pub fn is_dns(s: impl AsRef<str>) -> bool {
    DNS_REGEX.is_match(s.as_ref())
}

lazy_static! {
    static ref DNS_REGEX: Regex =
        Regex::new(r"^(?:[a-zA-Z0-9](?:[a-zA-Z0-9-]{0,61}[a-zA-Z0-9])?\.)+[a-zA-Z]{2,}$").unwrap();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_dns() {
        assert_eq!(true, is_dns("abc.com"));
        assert_eq!(true, is_dns("123.cn"));
        assert_eq!(true, is_dns("123.abc.com"));

        assert_eq!(false, is_dns(""));
        assert_eq!(false, is_dns("abc@com"));
        assert_eq!(false, is_dns("@#$.com."));
        assert_eq!(false, is_dns("http://abc.com"));
    }
}
