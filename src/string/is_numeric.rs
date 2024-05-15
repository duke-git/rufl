/// Checks if the string numeric (can be parsed to number).
///
/// # Arguments
///
/// * `s` - The string to check.
///
/// # Returns
///
/// Returns true if string is numeric.
///
/// # Examples
///
/// ```
/// use rufl::string;
///
/// assert_eq!(true, string::is_numeric("1.23"));
///
/// assert_eq!(true, string::is_numeric("1.23e2"));
///
/// assert_eq!(false, string::is_numeric("a1"));
///
/// assert_eq!(false, string::is_numeric("1..2"));
///
/// ```

pub fn is_numeric(source_str: &str) -> bool {
    if source_str.is_empty() {
        return false;
    }

    fn can_parseable_to_num(n: &str) -> bool {
        match n.find('.') {
            Some(_) => n.parse::<f32>().is_ok(),
            None => n.parse::<i32>().is_ok(),
        }
    }

    let lower_source: String = source_str.to_lowercase();

    match source_str.to_lowercase().find('e') {
        Some(_) => {
            let nums: Vec<&str> = lower_source.split('e').collect();
            can_parseable_to_num(nums[0]) && can_parseable_to_num(nums[1])
        }
        None => {
            if source_str.to_lowercase().starts_with("0x") {
                let s = lower_source.trim_start_matches("0x");
                i32::from_str_radix(s, 16).is_ok()
            } else {
                can_parseable_to_num(source_str)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_numeric() {
        assert_eq!(true, is_numeric("0"));
        assert_eq!(true, is_numeric("+0"));
        assert_eq!(true, is_numeric("-0"));
        assert_eq!(true, is_numeric("0."));
        assert_eq!(true, is_numeric("0.0"));
        assert_eq!(true, is_numeric("+1"));
        assert_eq!(true, is_numeric("-1"));
        assert_eq!(true, is_numeric("1."));
        assert_eq!(true, is_numeric("-1.2"));
        assert_eq!(true, is_numeric("1.1e2"));
        assert_eq!(true, is_numeric("1.1e+2"));
        assert_eq!(true, is_numeric("1.1e-2"));
        assert_eq!(true, is_numeric("1.1E2"));
        assert_eq!(true, is_numeric("1.1E+2"));
        assert_eq!(true, is_numeric("1.1E-2"));

        assert_eq!(true, is_numeric("0xab"));
        assert_eq!(true, is_numeric("0xAB"));
        assert_eq!(true, is_numeric("0x20"));
        assert_eq!(true, is_numeric("0XaB"));

        assert_eq!(false, is_numeric(""));
        assert_eq!(false, is_numeric(" "));
        assert_eq!(false, is_numeric("a1"));
        assert_eq!(false, is_numeric("1.."));
        assert_eq!(false, is_numeric("1e"));
    }
}
