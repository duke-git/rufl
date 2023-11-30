use std::iter;

/// Searches a string and returns the index of the first occurrence of the specified searched substring.
///
/// # Arguments
///
/// * `s` - The input string to perform index.
/// * `search` - The substring to search for.
/// * `position` - The position of source string to start to search. Start position is 0.
///
/// # Returns
///
/// Returns index of the first occurrence of searched substring.
///
/// # Examples
///
/// ```
/// use ruf::string;
///
/// assert_eq!(2, string::index("hello", "l", 0));
///
/// assert_eq!(2, string::index("hello", "l", 2));
///
/// assert_eq!(3, string::index("hello", "l", 3));
///
/// assert_eq!(-1, string::index("hello", "l", 4));
///
/// ```

pub fn index(s: impl AsRef<str>, search: &str, position: usize) -> i32 {
    match search.len() {
        0 => 0,
        _ => {
            if s.as_ref().chars().count() <= position {
                return -1;
            }

            let sub_string = &s.as_ref()[s.as_ref().char_indices().nth(position).unwrap().0..];

            match crate::string::split_chars(sub_string)
                .iter()
                .enumerate()
                .position(|(pos, _)| {
                    match &sub_string[sub_string.char_indices().nth(pos).unwrap().0..].find(search)
                    {
                        Some(n) => *n == 0,
                        None => false,
                    }
                }) {
                Some(n) => {
                    let result =
                        n + (s.as_ref().char_indices().count() - sub_string.char_indices().count());

                    result as i32
                }
                None => -1,
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_removen() {
        assert_eq!(0, index("hello", "", 0));
        assert_eq!(2, index("hello", "l", 0));
        assert_eq!(2, index("hello", "l", 1));
        assert_eq!(2, index("hello", "l", 2));
        assert_eq!(3, index("hello", "l", 3));

        assert_eq!(0, index("你好你好!", "你好", 0));
        assert_eq!(2, index("你好你好", "你好", 1));
        assert_eq!(2, index("你好你好", "你好", 2));
        assert_eq!(7, index("你好hello你好", "你好", 2));
        assert_eq!(-1, index("你好hello你好", "你好", 9));
        assert_eq!(-1, index("你好hello你好", "你好", 10));
    }
}
