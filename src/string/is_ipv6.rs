use std::net::Ipv6Addr;

/// Checks if the string is a valid ipv6 address.
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
/// use rufl::string;
///
/// assert_eq!(true, string::is_ipv6("::0:0:0:0:0:0:1"));
///
/// assert_eq!(false, string::is_ipv6("127.0.0.1"));
///
/// ```

pub fn is_ipv6(s: impl AsRef<str>) -> bool {
    s.as_ref().parse::<Ipv6Addr>().is_ok()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_ipv6() {
        assert_eq!(true, is_ipv6("::0:0:0:0:0:0:1"));
        assert_eq!(true, is_ipv6("::1"));
        assert_eq!(true, is_ipv6("::"));

        assert_eq!(false, is_ipv6(""));
        assert_eq!(false, is_ipv6("127.0.0.1"));
        assert_eq!(false, is_ipv6("2001:db8::8a2e:37023:7334"));
        assert_eq!(false, is_ipv6("2001::25de::cade"));
    }
}
