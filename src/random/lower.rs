use super::rand;

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
/// use rufl::random;
///
/// let lower_str = random::lower(6);
///
/// assert_eq!(true, lower_str.chars().all(char::is_lowercase));
/// assert_eq!(6, lower_str.len());
///
/// ```

pub fn lower(length: usize) -> String {
    rand::rand_string(rand::CharType::LowerLetter, length)
}

#[cfg(test)]
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
