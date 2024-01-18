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
/// use ruf::math;
///
/// assert_eq!(1, math::factorial(-1));
///
/// assert_eq!(1, math::factorial(0));
///
/// assert_eq!(6, math::factorial(3));
///
/// ```

pub fn factorial<T: Integer>(n: T) -> T {
    factorial_tail(n, Integer::cast(1 as i128))
}
fn factorial_tail<T: Integer>(n: T, acc: T) -> T {
    if n.le(&Integer::cast(0 as i128)) {
        acc
    } else {
        factorial_tail(n.sub(&Integer::cast(1 as i128)), acc.mul(&n))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_factorial() {
        assert_eq!(1, factorial(-1));
        assert_eq!(1, factorial(0));
        assert_eq!(1, factorial(1));
        assert_eq!(2, factorial(2));
        assert_eq!(6, factorial(3));
    }
}
