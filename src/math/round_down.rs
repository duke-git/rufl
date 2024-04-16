use super::float::Float;
use std::ops::{Div, Mul};

/// Round down and truncate off `n` decimal places to number.
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
/// use rufl::math;
///
/// let pi: f64 = std::f64::consts::PI;
///
/// assert_eq!(math::round_down(pi, 0), 3.0);
/// assert_eq!(math::round_down(pi, 1), 3.1);
/// assert_eq!(math::round_down(pi, 2), 3.14);
/// assert_eq!(math::round_down(pi, 3), 3.141);
/// assert_eq!(math::round_down(pi, 4), 3.1415);
///
/// ```

pub fn round_down<T>(number: T, n: usize) -> T
where
    T: Float + Mul<f64, Output = T> + Div<f64, Output = T>,
{
    let factor = 10f64.powi(n as i32);
    let rounded = (number * factor).floor_val();

    rounded / factor
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_round_down() {
        let pi: f64 = std::f64::consts::PI;

        assert_eq!(round_down(pi, 0), 3.0);
        assert_eq!(round_down(pi, 1), 3.1);
        assert_eq!(round_down(pi, 2), 3.14);
        assert_eq!(round_down(pi, 3), 3.141);
        assert_eq!(round_down(pi, 4), 3.1415);
        assert_eq!(round_down(pi, 5), 3.14159);
        assert_eq!(round_down(pi, 6), 3.141592);
        assert_eq!(round_down(pi, 7), 3.1415926);
        assert_eq!(round_down(pi, 8), 3.14159265);
    }
}
