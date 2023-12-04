/// Returns the word count in target string.
///
///
/// # Arguments
///
/// * `s` - The string to count word.
///
/// # Returns
///
/// Returns the number of words in target string.
///
/// # Example
///
/// ```rust
/// use ruf::string;
///
/// assert_eq!(2, string::count_words("hello world", ""));
///
/// assert_eq!(2, string::count_words("hello-world", ""));
///
/// assert_eq!(3, string::count_words("你好-hello-world", "-"));
///
/// assert_eq!(3, string::count_words("fooBarBoo", ""));
///
/// ```

pub fn count_words(s: impl AsRef<str>, pattern: &str) -> usize {
    match s.as_ref().len() {
        0 => 0,
        _ => match pattern.len() {
            0 => {
                let word_list = crate::string::split_words(s);
                let words: Vec<String> = word_list
                    .into_iter()
                    .filter(|v| v.chars().all(char::is_alphabetic))
                    .collect();

                words.len()
            }
            _ => s.as_ref().split_terminator(pattern).count(),
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_count_words() {
        assert_eq!(2, count_words("hello world", ""));
        assert_eq!(2, count_words("hello-world", ""));
        assert_eq!(3, count_words("fooBarBoo", ""));
        assert_eq!(3, count_words("foo-Bar-Boo", "-"));
        assert_eq!(2, count_words("你好 rust", " "));
        assert_eq!(5, count_words("hello--αρχές** (του) 21ου, αιώνα!'", ""));
    }
}
