/// Searches a string and returns all the indexs of the occurrence of the specified searched substring.
///
/// # Arguments
///
/// * `s` - The input string where to search.
/// * `search` - The substring to search for.
/// * `position` - The position to start searching.
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
/// assert_eq!(vec![2, 3], string::index_all("hello", "l", 0));
///
/// assert_eq!(vec![0, 2], string::index_all("你好你好!", "你好", 0));
/// ```

pub fn index_all(s: impl AsRef<str>, search: &str, position: usize) -> Vec<usize> {
    if s.as_ref().is_empty() || search.is_empty() || s.as_ref().chars().count() <= position {
        return vec![];
    }

    let mut result = Vec::new();
    let mut index = crate::string::index(&s, search, 0);

    while index != -1 {
        result.push(index as usize);
        index = crate::string::index(&s, search, (index + 1) as usize);
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_index_all() {
        assert_eq!(true, index_all("hello", "", 0).is_empty());
        assert_eq!(true, index_all("hello", "", 10).is_empty());

        assert_eq!(vec![2, 3], index_all("hello", "l", 0));
        assert_eq!(vec![2], index_all("hello", "ll", 0));
        assert_eq!(vec![2, 3], index_all("hello", "l", 3));

        assert_eq!(vec![0, 2], index_all("你好你好!", "你好", 0));
    }
}
