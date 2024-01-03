use super::generate;

/// Generate random upper case English letter string.
///
/// # Arguments
///
/// * `length` - The char size of random string.
///
/// # Returns
///
/// Returns random upper case string with specific length.
///
/// # Examples
///
/// ```
/// use ruf::random;
///
/// let upper_str = random::upper(6);
///
/// assert_eq!(true, upper_str.chars().all(char::is_uppercase));
/// assert_eq!(6, upper_str.len());
///
/// ```

pub fn upper(length: usize) -> String {
    generate::generate_str("UPPER", length)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_upper() {
        let upper_str = upper(6);
        assert_eq!(true, upper_str.chars().all(char::is_uppercase));
        assert_eq!(6, upper_str.len());

        assert_eq!("".to_string(), upper(0));
    }
}
