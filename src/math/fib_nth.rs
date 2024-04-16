use std::ops::Add;

use super::integer::Integer;

/// Calculates the nth value of fibonacci number sequence.
///
/// # Arguments
///
/// * `first` - The first value of fibonacci number sequence.
///
/// * `second` - The second value of fibonacci number sequence.
///
/// * `n` - The nth fibonacci number to calculate.
///
/// # Returns
///
///  The nth value of fibonacci number sequence.
///
/// # Examples
///
/// ```
/// use rufl::math;
///
/// assert_eq!(0, math::fib_nth(0, 1, 0));
///
/// assert_eq!(1, math::fib_nth(0, 1, 1));
///
/// assert_eq!(5, math::fib_nth(0, 1, 5));
///
/// assert_eq!(8, math::fib_nth(1, 1, 5));
/// ```

pub fn fib_nth<T>(first: T, second: T, n: usize) -> T
where
    T: Integer + Add<Output = T>,
{
    match n {
        0 => first,
        1 => second,
        _ => fib_nth(second, first + second, n - 1),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fib_nth() {
        assert_eq!(0, fib_nth(0, 1, 0));
        assert_eq!(1, fib_nth(0, 1, 1));
        assert_eq!(1, fib_nth(0, 1, 2));
        assert_eq!(2, fib_nth(0, 1, 3));
        assert_eq!(3, fib_nth(0, 1, 4));
        assert_eq!(5, fib_nth(0, 1, 5));

        assert_eq!(1, fib_nth(1, 1, 0));
        assert_eq!(1, fib_nth(1, 1, 1));
        assert_eq!(2, fib_nth(1, 1, 2));
        assert_eq!(8, fib_nth(1, 1, 5));
    }
}
