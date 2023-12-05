/// Counts the characters in target string with predicate function, returns the number of all matched characters.
///
///
/// # Arguments
///
/// * `s` - The string to count characters.
/// * `predicate` - The predicate function invoked on each character with a parameter.
///
/// # Returns
///
/// Returns the number of all matched characters.
///
/// # Example
///
/// ```rust
/// use ruf::string;
///
/// assert_eq!(5, string::count_by("hello!", string::is_alpha));
///
/// assert_eq!(3, string::count_by("1a2b3c", |s: &str| -> bool {s.chars().all(char::is_numeric)}));
///
/// ```

pub fn count_by(s: impl AsRef<str>, predicate: fn(&str) -> bool) -> usize {
    match s.as_ref().len() {
        0 => 0,
        _ => {
            let mut count = 0;

            for c in crate::string::split_graphemes(s.as_ref()).iter() {
                if predicate(c) {
                    count += 1;
                }
            }
            count
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_count_by() {
        assert_eq!(5, count_by("hello!", crate::string::is_alpha));

        assert_eq!(3, count_by("1a2b3c", crate::string::is_digit));

        assert_eq!(
            1,
            count_by("a b", |s: &str| -> bool {
                s.chars().all(char::is_whitespace)
            })
        );

        assert_eq!(
            1,
            count_by("hello\n", |s: &str| -> bool {
                s.chars().all(char::is_control)
            })
        );
    }
}
