use super::rand;

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
/// use rufl::random;
///
/// let alphabet_str = random::alphabet(6);
///
/// assert_eq!(true, alphabet_str.chars().all(char::is_alphabetic));
/// assert_eq!(6, alphabet_str.len());
///
/// ```

pub fn alphabet(length: usize) -> String {
    rand::rand_string(rand::CharType::Letter, length)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_alphabet() {
        let alphabet_str = alphabet(6);
        println!("{}", alphabet_str);

        assert_eq!(true, alphabet_str.chars().all(char::is_alphabetic));
        assert_eq!(6, alphabet_str.len());

        assert_eq!("".to_string(), alphabet(0));
    }
}
