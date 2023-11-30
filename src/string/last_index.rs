/// Searches a string and returns the index of the last occurrence of the specified searched substring.
///
/// Note: this function behive the same as [lastIndexOf](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/String/lastIndexOf) in javascript.
///
/// # Arguments
///
/// * `s` - The input string to perform index.
/// * `search` - The substring to search for.
/// * `position` - The position of source string to end to search. (not included)
///
/// # Returns
///
/// Returns index of the last occurrence of searched substring.
///
/// # Examples
///
/// ```
/// use ruf::string;
///
/// assert_eq!(-1, string::last_index("hello world", "l", 0));
///
/// assert_eq!(9, string::last_index("hello world", "l", "hello world".len()));
///
/// assert_eq!(3, string::last_index("hello world", "l", 6));
///
/// ```

pub fn last_index(s: impl AsRef<str>, search: &str, position: usize) -> i32 {
    let mut substring = "";
    if s.as_ref().chars().count() <= position {
        substring = s.as_ref();
    } else {
        substring = &s.as_ref()[..s.as_ref().char_indices().nth(position).unwrap().0];
    }

    // println!("substring is {}", substring);

    match substring.rfind(search) {
        Some(i) => i as i32,
        None => -1,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_removen() {
        assert_eq!(-1, last_index("canal", "a", 0));
        assert_eq!(-1, last_index("canal", "a", 1));
        assert_eq!(1, last_index("canal", "a", 2));
        assert_eq!(3, last_index("canal", "a", 5));
        assert_eq!(3, last_index("canal", "a", 100));
        assert_eq!(-1, last_index("canal", "x", 5));
        assert_eq!(2, last_index("canal", "", 2));
        assert_eq!(0, last_index("你好hello你好", "你好", 4));
        assert_eq!(11, last_index("你好hello你好", "你好", 10));
    }
}
