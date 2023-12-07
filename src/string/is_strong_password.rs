use lazy_static::lazy_static;
use regex::Regex;

/// Checks if the string is a strong password.
/// A strong password must be 8 characters long and then contain characters from at least 3 of the following 4 rules:
///  1. Upper case English letters.
///  2. Lower case English letters.
///  3. Numbers(0-9).
///  4. Non-alpha.
///
/// # Arguments
///
/// * `s` - The string to check.
///
/// # Returns
///
/// Returns true if string is a strong password, false if not.
///
/// # Examples
///
/// ```
/// use ruf::string;
///
/// assert_eq!(true, string::is_strong_password("abc123@ABC"));
///
/// assert_eq!(false, string::is_strong_password("12@abAB"));
///
/// ```

pub fn is_strong_password(s: impl AsRef<str>) -> bool {
    if s.as_ref().len() < 8 {
        return false;
    }

    let has_number = NUMBER_REGEX.is_match(s.as_ref()) as i32;
    let has_upper = UPPERCA_CASE_REGEX.is_match(s.as_ref()) as i32;
    let has_lower = LOWER_CASE_REGEX.is_match(s.as_ref()) as i32;
    let has_nonalpha = NONALPHA_REGEX.is_match(s.as_ref()) as i32;

    has_number + has_upper + has_lower + has_nonalpha > 3
}

lazy_static! {
    static ref UPPERCA_CASE_REGEX: Regex = Regex::new(r"[A-Z]").unwrap();
    static ref LOWER_CASE_REGEX: Regex = Regex::new(r"[a-z]").unwrap();
    static ref NUMBER_REGEX: Regex = Regex::new(r"[0-9]").unwrap();
    static ref NONALPHA_REGEX: Regex = Regex::new(r"[^\w\s]").unwrap();

    // static ref STRONG_PASSWORD_REGEX: Regex = Regex::new(r"^(?:(?=.*[a-z])(?:(?=.*[A-Z])(?=.*[\d\W])|(?=.*\W)(?=.*\d))|(?=.*\W)(?=.*[A-Z])(?=.*\d)).{8,}$").unwrap();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_strong_password() {
        assert_eq!(true, is_strong_password("abc123@ABC"));

        assert_eq!(false, is_strong_password("abc123abc"));
        assert_eq!(false, is_strong_password("abcABC"));
        assert_eq!(false, is_strong_password("ab12AB"));
        assert_eq!(false, is_strong_password("ab12@AB"));
    }
}
