/// Removes all the specified substring which occurrence in the source string.
///
/// # Arguments
///
/// * `s` - The input string to perform remove.
/// * `substr` - The substring to be removed.
///
/// # Returns
///
/// Returns string after being removed.
///
/// # Examples
///
/// ```
/// use rufl::string;
///
/// let foo = string::remove_all("abca", "a");
/// assert_eq!("bc".to_string(), foo);
///
/// let bar = string::remove_all("abc", "d");
/// assert_eq!("abc".to_string(), bar);
///
/// let boo = string::remove_all("abc", "abc");
/// assert_eq!("".to_string(), boo);
///
/// ```

pub fn remove_all(s: impl AsRef<str>, substr: &str) -> String {
    let str_list: Vec<&str> = s.as_ref().split(substr).collect();

    str_list
        .into_iter()
        .filter(|v: &&str| v.to_string() != "")
        .collect::<String>()

    // s.as_ref().replace(substr, "")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_remove_all() {
        assert_eq!("abc".to_string(), remove_all("abc", ""));
        assert_eq!("bc".to_string(), remove_all("abac", "a"));
        assert_eq!("abc".to_string(), remove_all("abc", "d"));
    }
}
