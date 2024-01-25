/// Add comma to a number value by every 3 numbers from right. Ahead by prefix symbol.
///
/// # Arguments
///
/// * `number` - The number perform add commars.
/// * `prefix_symbol` - Prefix symbol of returned string.
///
/// # Returns
///
/// String form of number after adding comma.
///
/// # Examples
///
/// ```
/// use ruf::string;
///
/// assert_eq!("1,234,567", string::add_commas("1234567", ""));
///
/// assert_eq!("¥1,234,567", string::add_commas("1234567", "¥"));
///
/// assert_eq!("$1,234.567", string::add_commas(1234.567, "$"));
///
/// ```
///

pub fn add_commas<T: std::fmt::Display>(number: T, prefix_symbol: &str) -> String {
    let mut num_string = number.to_string();

    if num_string.len() == 0 {
        return "".to_string();
    }

    let point_index = num_string.find('.');
    let mut index = if let Some(point_index) = point_index {
        point_index
    } else {
        num_string.len()
    };

    while index > 3 {
        index -= 3;
        num_string.insert(index, ',');
    }

    prefix_symbol.to_string() + &num_string
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_commas() {
        assert_eq!("", add_commas("", ""));
        assert_eq!("", add_commas("", "$"));

        assert_eq!("1,234,567", add_commas("1234567", ""));
        assert_eq!("¥1,234,567", add_commas("1234567", "¥"));
        assert_eq!("$1,234,567", add_commas(1234567, "$"));
        assert_eq!("$1,234.567", add_commas(1234.567, "$"));
        assert_eq!("$12,345.6789", add_commas(12345.6789, "$"));
    }
}
