use super::generate;

/// Generate random lower case English letter string.
///
/// # Arguments
///
/// * `length` - The char size of random string.
///
/// # Returns
///
/// Returns random lower case string with specific length.
///
/// # Examples
///
/// ```
/// use ruf::random;
///
/// let lower_str = random::lower(6);
///
/// assert_eq!(true, lower_str.chars().all(char::is_lowercase));
/// assert_eq!(6, lower_str.len());
///
/// ```

pub fn lower(length: usize) -> String {
    generate::generate_str("LOWER", length)
}

mod tests {
    use super::*;

    #[test]
    fn test_lower() {
        let lower_str = lower(6);
        assert_eq!(true, lower_str.chars().all(char::is_lowercase));
        assert_eq!(6, lower_str.len());

        assert_eq!("".to_string(), lower(0));
    }
}
