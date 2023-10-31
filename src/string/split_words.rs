/// Split the input string into an array of its words (based on the position of capital letters or numbers).
///
/// Note: This function will split words based on the position of capital letters or numbers.
///
/// # Arguments
///
/// * `s` - The input string to split into words.
///
/// # Returns
///
/// Returns a vector containing the words extracted from the input string.
///
/// # Example
///
/// ```rust
/// use ruf::string;
///
/// let words = string::split_words("fooBarFoo123");
/// assert_eq!(words, vec!["foo", "Bar", "Foo", "123"]);
///
/// ```

pub fn split_words(s: impl AsRef<str>) -> Vec<String> {
    if s.as_ref().is_empty() {
        return vec!["".to_string()];
    }

    let mut word_list: Vec<Vec<String>> = Vec::new();

    let mut char_type: usize = 0;
    let mut last_char_type: usize = 0;

    for (_, c) in s.as_ref().chars().enumerate() {
        if c.is_lowercase() {
            char_type = 1;
        } else if c.is_uppercase() {
            char_type = 2;
        } else if c.is_digit(10) {
            char_type = 3;
        } else {
            char_type = 4;
        }

        if char_type == last_char_type {
            let len = word_list.len();
            word_list[len - 1].push(c.to_string());
        } else {
            word_list.push(vec![c.to_string()]);
        }
        last_char_type = char_type;
    }

    let mut words = Vec::new();

    let mut i: usize = 0;

    while i < word_list.len() {
        if word_list[i][0].chars().all(char::is_uppercase)
            && word_list[i + 1][0].chars().all(char::is_lowercase)
        {
            words.push(word_list[i].join("") + &word_list[i + 1].join(""));
            i += 2;
            continue;
        }

        if word_list[i][0].chars().all(char::is_lowercase) {
            words.push(word_list[i].join(""));
            i += 1;
            continue;
        }

        if word_list[i][0].chars().all(char::is_uppercase) {
            words.push(word_list[i].join(""));
            i += 1;
            continue;
        }

        if word_list[i][0].chars().all(char::is_numeric) {
            words.push(word_list[i].join(""));
            i += 1;
            continue;
        }

        i += 1;
    }

    words
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_split_words() {
        assert_eq!(vec![""], split_words(""));
        assert_eq!(vec!["foo", "bar"], split_words("foo bar"));
        assert_eq!(vec!["foo", "Bar"], split_words("fooBar"));
        assert_eq!(vec!["foo", "bar"], split_words("foo&bar"));
        assert_eq!(vec!["FOO", "BAR"], split_words("__FOO_BAR__"));

        assert_eq!(
            vec!["foo", "Bar", "Foo", "123"],
            split_words("$&foo&-_BarFoo.123")
        );

        assert_eq!(
            vec!["Sześć", "звёзд", "Sześć", "звёзд"],
            split_words("Sześć звёзд Sześć звёзд")
        );
    }
}
