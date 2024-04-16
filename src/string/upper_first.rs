/// Converts the first character of string to upper case.
///
/// # Arguments
///
/// * `s` - The string to convert.
///
/// # Returns
///
/// Returns the converted string.
///
/// # Examples
///
/// Basic usage:
///
/// ```
/// use rufl::string;
///
/// assert_eq!("FOO", string::upper_first("FOO"));
///
/// assert_eq!("_fOO", string::upper_first("_fOO"));
///
/// assert_eq!("Foo_Bar", string::upper_first("foo_Bar"));
///
/// assert_eq!("Foo Bar", string::upper_first("foo Bar"));
///
/// ```

pub fn upper_first(s: impl AsRef<str>) -> String {
    match s.as_ref().len() {
        0 => s.as_ref().to_string(),
        _ => s
            .as_ref()
            .chars()
            .enumerate()
            .map(|(i, item)| match i {
                0 => item.to_uppercase().to_string(),
                _ => item.to_string(),
            })
            .collect::<Vec<String>>()
            .join(""),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    #[test]
    fn test_upper_first() {
        let case_list: Vec<(String, String)> = vec![
            ("foo".to_string(), "Foo".to_string()),
            ("FOo".to_string(), "FOo".to_string()),
            ("foo_".to_string(), "Foo_".to_string()),
            ("Foo Bar".to_string(), "Foo Bar".to_string()),
            ("_foo".to_string(), "_foo".to_string()),
            ("foo_bar".to_string(), "Foo_bar".to_string()),
            ("foo-bar".to_string(), "Foo-bar".to_string()),
        ];

        let case_map: HashMap<String, String> = case_list.into_iter().collect();

        for (key, value) in case_map {
            assert_eq!(value, upper_first(key));
        }
    }
}
