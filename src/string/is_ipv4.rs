use std::net::Ipv4Addr;

/// Checks if the string is a valid ipv4 address.
///
/// # Arguments
///
/// * `s` - The string to check.
///
/// # Returns
///
/// Returns true if string is a valid ipv4 address, false if not.
///
/// # Examples
///
/// ```
/// use ruf::string;
///
/// assert_eq!(true, string::is_ipv4("127.0.0.1"));
///
/// assert_eq!(false, string::is_ipv4("256.0.0.1"));
///
/// ```

pub fn is_ipv4(s: impl AsRef<str>) -> bool {
    s.as_ref().parse::<Ipv4Addr>().is_ok()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_ipv4() {
        assert_eq!(true, is_ipv4("127.0.0.1"));

        assert_eq!(false, is_ipv4(""));
        assert_eq!(false, is_ipv4("::0:0:0:0:0:0:1"));
        assert_eq!(false, is_ipv4("127.0.0.1.1"));
        assert_eq!(false, is_ipv4("256.0.0.1"));
        assert_eq!(false, is_ipv4("127.0.0.a"));
    }
}
