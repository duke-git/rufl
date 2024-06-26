use std::ops::{Mul, Sub};

use super::integer::Integer;

/// Calculats the factorial value of number `n`.
///
/// # Arguments
///
/// * `n` - The number to calculate factorial value.
///
/// # Returns
///
/// Factorial value of number `n`.
///
/// # Examples
///
/// ```
/// use rufl::math;
///
/// assert_eq!(1, math::factorial(0));
///
/// assert_eq!(6, math::factorial(3));
///
/// ```

pub fn factorial<T>(n: T) -> T
where
    T: Integer + Sub<usize, Output = T> + Mul<Output = T>,
{
    factorial_tail(n, Integer::cast(1 as i128))
}
fn factorial_tail<T: Integer + Sub<usize, Output = T> + Mul<Output = T>>(n: T, acc: T) -> T {
    if n <= T::ZERO {
        acc
    } else {
        factorial_tail(n - 1, acc * n)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_factorial() {
        assert_eq!(1, factorial(0));
        assert_eq!(1, factorial(1));
        assert_eq!(2, factorial(2));
        assert_eq!(6, factorial(3));
    }
}
