/// Converts the first character of string to upper case and the remaining to lower case.
///
/// # Arguments
///
/// * `s` - The string to capitalize.
///
/// # Returns
///
/// Returns the capitalized string.
///
/// # Examples
///
/// Basic usage:
///
/// ```
/// use ruf::string;
///
/// assert_eq!("Foo", string::capitalize("FOO"));
///
/// assert_eq!("_foo", string::capitalize("_fOO"));
///
/// assert_eq!("Foo_bar", string::capitalize("foo_Bar"));
///
/// assert_eq!("Foo bar", string::capitalize("foo Bar"));
///
/// ```

pub fn capitalize(s: impl AsRef<str>) -> String {
    let mut result = String::with_capacity(s.as_ref().len());

    for (i, char) in s.as_ref().chars().enumerate() {
        if i == 0 {
            result.push_str(&char.to_uppercase().to_string());
        } else {
            result.push_str(&char.to_lowercase().to_string());
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    #[test]
    fn test_capitalize() {
        let case_list: Vec<(String, String)> = vec![
            ("foo".to_string(), "Foo".to_string()),
            ("FOo".to_string(), "Foo".to_string()),
            ("foo_".to_string(), "Foo_".to_string()),
            ("foo Bar".to_string(), "Foo bar".to_string()),
            ("_foo".to_string(), "_foo".to_string()),
            ("foo_Bar".to_string(), "Foo_bar".to_string()),
            ("foo-bar".to_string(), "Foo-bar".to_string()),
        ];

        let case_map: HashMap<String, String> = case_list.into_iter().collect();

        for (key, value) in case_map {
            assert_eq!(value, capitalize(key));
        }
    }
}
