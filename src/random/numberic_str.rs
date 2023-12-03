use super::generate;

/// Generate random numberic string.
///
/// # Arguments
///
/// * `length` - The char size of random string.
///
/// # Returns
///
/// Returns random numberic string with specific length.
///
/// # Examples
///
/// ```
/// use ruf::random;
///
/// let numberic_str = random::numberic_str(6);
///
/// assert_eq!(true, numberic_str.chars().all(char::is_numeric));
/// assert_eq!(6, numberic_str.len());
///
/// ```

pub fn numberic_str(length: usize) -> String {
    generate::generate_str("NUMBER", length)
}

mod tests {
    use super::*;

    #[test]
    fn test_numberic_str() {
        let str = numberic_str(6);
        assert_eq!(true, str.chars().all(char::is_numeric));
        assert_eq!(6, str.len());

        assert_eq!("".to_string(), numberic_str(0));
    }
}
