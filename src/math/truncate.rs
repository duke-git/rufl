/// Truncate number to n decimal places after decimal point.
///
/// # Arguments
///
/// * `number` - The number to calculate truncate.
///
/// * `n` - n digits after decimal point.
///
/// # Returns
///
/// Truncated value of number.
///
/// # Examples
///
/// ```
/// use rufl::math;
///
/// assert_eq!(0.0, math::truncate(0.1234, 0));
/// assert_eq!(0.1, math::truncate(0.1234, 1));
/// assert_eq!(0.12, math::truncate(0.1234, 2));
/// assert_eq!(0.123, math::truncate(0.1234, 3));
/// assert_eq!(0.1234, math::truncate(0.1234, 4));
/// assert_eq!(0.1234, math::truncate(0.1234, 5));
///
/// ```

pub fn truncate(number: f64, n: usize) -> f64 {
    let s = number.to_string();
    let mut n = n;

    match s.find(".") {
        Some(i) => {
            if n > s[i + 1..].len() {
                n = s[i + 1..].len();
            }

            let int_str = &s[0..i];
            let float_str = &s[i..i + n + 1];
            let num_str = int_str.to_owned() + float_str;

            let result: f64 = num_str.parse().unwrap();

            result
        }
        None => number,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_truncate() {
        assert_eq!(0.0, truncate(0.1234, 0));
        assert_eq!(0.1, truncate(0.1234, 1));
        assert_eq!(0.12, truncate(0.1234, 2));
        assert_eq!(0.123, truncate(0.1234, 3));
        assert_eq!(0.1234, truncate(0.1234, 4));
        assert_eq!(0.1234, truncate(0.1234, 5));
    }
}
