/// Removes first n matches `substr` in the source string.
///
/// # Arguments
///
/// * `s` - The input string to perform remove.
/// * `substr` - The substring to be removed.
/// * `n` - The count of removed substr.
///
/// # Returns
///
/// Returns string after being removed.
///
/// # Examples
///
/// ```
/// use ruf::string;
///
/// let foo1 = string::removen("abab", "a", 0);
/// assert_eq!("abab".to_string(), foo1);
///
/// let foo2 = string::removen("abab", "a", 1);
/// assert_eq!("bab".to_string(), foo2);
///
/// let foo3 = string::removen("abab", "a", 2);
/// assert_eq!("bb".to_string(), foo3);
///
/// let foo4 = string::removen("abab", "a", 3);
/// assert_eq!("bb".to_string(), foo3);
///
/// ```

pub fn removen(s: impl AsRef<str>, substr: &str, n: usize) -> String {
    s.as_ref().replacen(substr, "", n)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_removen() {
        let foo1 = removen("abab", "a", 0);
        assert_eq!("abab".to_string(), foo1);

        let foo2 = removen("abab", "a", 1);
        assert_eq!("bab".to_string(), foo2);

        let foo3 = removen("abab", "a", 2);
        assert_eq!("bb".to_string(), foo3);

        let foo4 = removen("abab", "a", 3);
        assert_eq!("bb".to_string(), foo4);
    }
}
