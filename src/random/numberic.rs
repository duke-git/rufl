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
/// let numberic_str = random::numberic(6);
///
/// assert_eq!(true, numberic_str.chars().all(char::is_numeric));
/// assert_eq!(6, numberic_str.len());
///
/// ```

pub fn numberic(length: usize) -> String {
    generate::generate_str("NUMBER", length)
}

mod tests {
    use super::*;

    #[test]
    fn test_numberic() {
        let str = numberic(6);
        assert_eq!(true, str.chars().all(char::is_numeric));
        assert_eq!(6, str.len());

        assert_eq!("".to_string(), numberic(0));
    }
}
