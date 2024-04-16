use super::float::Float;
/// Calculates square root of float number `n`.
///
/// # Arguments
///
/// * `n` - The float number to calculate square root value.
///
/// # Returns
///
/// Square root value of number `n`.
///
/// # Examples
///
/// ```
/// use rufl::math;
///
/// assert_eq!(0.1 as f32, math::sqrt(0.01 as f32));
///
/// assert_eq!(2.0, math::sqrt(4.0));
///
/// ```

pub fn sqrt<T: Float>(n: T) -> T {
    if n.type_of() == "f32" {
        Float::cast(n.to_f32().sqrt() as f64)
    } else {
        Float::cast(n.to_f64().sqrt())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sqrt() {
        assert_eq!(0.1 as f32, sqrt(0.01 as f32));
        assert_eq!(2.0, sqrt(4.0));
        assert_eq!(2.0, sqrt(4 as f64));
    }
}
