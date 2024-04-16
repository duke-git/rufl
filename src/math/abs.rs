use std::ops::Sub;

use super::number::Number;

/// Returns the absolute value of number `n`.
///
/// # Arguments
///
/// * `n` - The number to calculate absolute value.
///
/// # Returns
///
/// Absolute value of number `n`.
///
/// # Examples
///
/// ```
/// use rufl::math;
///
/// assert_eq!(1, math::abs(-1).unwrap());
///
/// assert_eq!(2 as i8, math::abs(-2 as i8).unwrap());
///
/// assert_eq!(0.1, math::abs(-0.1).unwrap());
///
/// ```

pub fn abs<T: Number + Sub<Output = T>>(n: T) -> Result<T, String> {
    if n == T::MIN {
        Err("Overflow: minimum value can't be representable".to_string())
    } else if n < T::ZERO {
        Ok(T::ZERO - n)
    } else {
        Ok(n)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_abs() {
        assert_eq!(0.1 as f32, abs(-0.1 as f32).unwrap());
        assert_eq!(0.2, abs(-0.2).unwrap());

        assert_eq!(1, abs(-1).unwrap());
        assert_eq!(2 as i8, abs(-2 as i8).unwrap());
    }
}
