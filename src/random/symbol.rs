use super::generate;

/// Generate random string which only contains special chars
/// (!@#$%^&*()_+-=[]{}|;':\",./<>?).
///
/// # Arguments
///
/// * `length` - The char size of random string.
///
/// # Returns
///
/// Returns random string with specific length.
///
/// # Examples
///
/// ```
/// use rufl::random;
///
/// let symbol_str = random::symbol(6);
/// assert_eq!(6, symbol_str.len());
///
/// let symbol_chars: &str = "!@#$%^&*()_+-=[]{}|;':\",./<>?";
/// for c in symbol_str.chars() {
///     assert!(symbol_chars.contains(c))
/// }
/// ```

pub fn symbol(length: usize) -> String {
    generate::generate_str("SYMBOL", length)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_symbol() {
        let symbol_str = symbol(6);

        assert_eq!(6, symbol_str.len());

        let symbol_chars: &str = "!@#$%^&*()_+-=[]{}|;':\",./<>?";
        for c in symbol_str.chars() {
            assert!(symbol_chars.contains(c))
        }

        assert_eq!("".to_string(), symbol(0));
    }
}
