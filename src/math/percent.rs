use super::float::Float;
/// Calculates percentage.
///
/// # Arguments
///
/// * `numerator` - The numerator number.
///
/// * `denominator` - The denominator number.
///
/// # Returns
///
/// percentage(numerator / denominator * 100).
///
/// # Examples
///
/// ```
/// use ruf::math;
///
/// assert_eq!(50.0 as f32, math::percent(1.0 as f32, 2.0 as f32));
///
/// assert_eq!(50.0, math::percent(2.0, 4.0));
///
/// ```

pub fn percent<T: Float>(numerator: T, denominator: T) -> T {
    numerator.div(&denominator).mul(&Float::cast(100 as f64))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_percent() {
        assert_eq!(50.0 as f32, percent(1.0 as f32, 2.0 as f32));
        assert_eq!(50.0, percent(2.0, 4.0));
    }
}
