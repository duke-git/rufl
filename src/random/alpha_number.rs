use super::generate;

/// Generate random alphabetic string.
///
/// # Arguments
///
/// * `length` - The char size of random string.
///
/// # Returns
///
/// Returns random alphabetic string with specific length.
///
/// # Examples
///
/// ```
/// use ruf::random;
///
/// let str = random::alpha_number(6);
///
/// assert_eq!(true, str.chars().all(char::is_alphanumeric));
/// assert_eq!(6, str.len());
///
/// ```

pub fn alpha_number(length: usize) -> String {
    generate::generate_str("ALPHANUMERIC", length)
}

mod tests {
    use super::*;

    #[test]
    fn test_alpha_number() {
        let str = alpha_number(6);

        assert_eq!(true, str.chars().all(char::is_alphanumeric));
        assert_eq!(6, str.len());

        assert_eq!("".to_string(), alpha_number(0));
    }
}
