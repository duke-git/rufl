use std::ops::Add;

use super::integer::Integer;

/// Calculates the sum value of fibonacci number sequence.
///
/// # Arguments
///
/// * `n` - The nth fibonacci sum value to calculate.
///
/// # Returns
///
///  The sum value of fibonacci number sequence.
///
/// # Examples
///
/// ```
/// use rufl::math;
///
/// assert_eq!(0, math::fib_sum(0));
///
/// assert_eq!(0, math::fib_sum(1));
///
/// assert_eq!(1, math::fib_sum(2));
///
/// assert_eq!(2, math::fib_sum(3));
/// ```

pub fn fib_sum<T>(n: usize) -> T
where
    T: Integer + Add<Output = T>,
{
    let mut sum = T::ZERO;

    for i in 1..=n {
        let k = fib_num(i - 1);
        sum = sum + k;
    }

    sum
}

fn fib_num<T>(n: usize) -> T
where
    T: Integer + Add<Output = T>,
{
    match n {
        0 => T::ZERO,
        1 => T::ONE,
        _ => fib_num::<T>(n - 1) + fib_num::<T>(n - 2),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fib_sum() {
        assert_eq!(0, fib_sum(0));
        assert_eq!(0, fib_sum(1));
        assert_eq!(1, fib_sum(2));
        assert_eq!(2, fib_sum(3));
        assert_eq!(4, fib_sum(4));
        assert_eq!(7, fib_sum(5));
    }
}
