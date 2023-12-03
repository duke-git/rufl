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
/// let alphabeta_str = random::alphabeta(6);
///
/// assert_eq!(true, alphabeta_str.chars().all(char::is_alphabetic));
/// assert_eq!(6, alphabeta_str.len());
///
/// ```

pub fn alphabeta(length: usize) -> String {
    generate::generate_str("LETTER", length)
}

mod tests {
    use super::*;

    #[test]
    fn test_alphabeta() {
        let alphabeta_str = alphabeta(6);
        println!("{}", alphabeta_str);

        assert_eq!(true, alphabeta_str.chars().all(char::is_alphabetic));
        assert_eq!(6, alphabeta_str.len());

        assert_eq!("".to_string(), alphabeta(0));
    }
}
