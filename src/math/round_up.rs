use std::ops::{Div, Mul};

use super::float::Float;

/// Round up and truncate off `n` decimal places to number.
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
/// assert_eq!(math::round_up(pi, 0), 4.0);
/// assert_eq!(math::round_up(pi, 1), 3.2);
/// assert_eq!(math::round_up(pi, 2), 3.15);
/// assert_eq!(math::round_up(pi, 3), 3.142);
/// assert_eq!(math::round_up(pi, 4), 3.1416);
///
/// ```

pub fn round_up<T>(number: T, n: usize) -> T
where
    T: Float + Mul<f64, Output = T> + Div<f64, Output = T>,
{
    let factor = 10f64.powi(n as i32);
    let rounded = (number * factor).ceil_val();

    rounded / factor
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_round_up() {
        let pi: f64 = std::f64::consts::PI;

        assert_eq!(round_up(pi, 0), 4.0);
        assert_eq!(round_up(pi, 1), 3.2);
        assert_eq!(round_up(pi, 2), 3.15);
        assert_eq!(round_up(pi, 3), 3.142);
        assert_eq!(round_up(pi, 4), 3.1416);
        assert_eq!(round_up(pi, 5), 3.14160);
        assert_eq!(round_up(pi, 6), 3.141593);
        assert_eq!(round_up(pi, 7), 3.1415927);
        assert_eq!(round_up(pi, 8), 3.14159266);
    }
}
