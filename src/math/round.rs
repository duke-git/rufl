use super::float::Float;

/// Round off `n` decimal places to number.
///
/// # Arguments
///
/// * `number` - The number to round and truncate.
///
/// * `n` - n digits after decimal point.
///
/// # Returns
///
/// Rounded number.
///
/// # Examples
///
/// ```
/// use ruf::math;
///
/// let pi: f64 = std::f64::consts::PI;
///
/// assert_eq!(math::round(pi, 0), 3.0);
/// assert_eq!(math::round(pi, 1), 3.1);
/// assert_eq!(math::round(pi, 2), 3.14);
/// assert_eq!(math::round(pi, 3), 3.142);
/// assert_eq!(math::round(pi, 4), 3.1416);
///
/// ```

pub fn round<T: Float>(number: T, n: usize) -> T {
    let factor = 10f64.powi(n as i32);
    let rounded = number.mul(&Float::cast(factor as f64)).round_val();

    rounded.div(&Float::cast(factor as f64))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_round() {
        let pi: f64 = std::f64::consts::PI;

        assert_eq!(round(pi, 0), 3.0);
        assert_eq!(round(pi, 1), 3.1);
        assert_eq!(round(pi, 2), 3.14);
        assert_eq!(round(pi, 3), 3.142);
        assert_eq!(round(pi, 4), 3.1416);
        assert_eq!(round(pi, 5), 3.14159);
        assert_eq!(round(pi, 6), 3.141593);
        assert_eq!(round(pi, 7), 3.1415927);
        assert_eq!(round(pi, 8), 3.14159265);
    }
}
