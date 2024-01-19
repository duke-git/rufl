/// Calculates harmonic value number `n`.
///
/// # Arguments
///
/// * `n` - The number to calculate harmonic value.
///
/// # Returns
///
/// Harmonic value.
///
/// # Examples
///
/// ```
/// use ruf::math;
///
/// assert_eq!(1.0000000001, math::harmonic(1));
///
/// assert_eq!(2.928968254968254, math::harmonic(10));
///
/// ```

pub fn harmonic(n: u32) -> f64 {
    let mut sum = 0.0;

    for i in 1..=n {
        sum += 1.0 / i as f64 + 1e-10;

        if sum.abs() < f64::EPSILON {
            return sum;
        }
    }
    sum
    // crate::math::round(sum, 3)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_harmonic() {
        assert_eq!(1.0000000001, harmonic(1));
        assert_eq!(2.928968254968254, harmonic(10));
    }
}
