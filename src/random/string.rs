use super::rand;

/// Generate random string.
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
/// let random_str = random::string(6);
///
/// println!("{}", random_str);
/// assert_eq!(6, random_str.len());
///
/// ```

pub fn string(length: usize) -> String {
    rand::rand_string(rand::CharType::All, length)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_string() {
        let random_str = string(6);
        println!("{}", random_str);

        assert_eq!(6, random_str.len());

        assert_eq!("".to_string(), string(0));
    }
}
