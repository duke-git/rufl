use std::ops::Add;

use super::integer::Integer;

/// Return fibonacci number sequence.
///
/// # Arguments
///
/// * `size` - The size of fibonacci sequence.
///
/// # Returns
///
///  The fibonacci number sequence.
///
/// # Examples
///
/// ```
/// use rufl::math;
///
/// assert_eq!(vec![0], math::fib_seq(1));
/// assert_eq!(vec![0, 1], math::fib_seq(2));
/// assert_eq!(vec![0, 1, 1], math::fib_seq(3));
/// assert_eq!(vec![0, 1, 1, 2], math::fib_seq(4));
/// assert_eq!(vec![0, 1, 1, 2, 3], math::fib_seq(5));
///
/// ```

pub fn fib_seq<T>(size: usize) -> Vec<T>
where
    T: Integer + Add<Output = T>,
{
    let mut nums = vec![T::ZERO; size];

    for i in 0..size {
        let num = crate::math::fib_nth(T::ZERO, T::ONE, i);
        nums[i] = num;
    }

    nums
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fib_seq() {
        assert_eq!(vec![0], fib_seq(1));
        assert_eq!(vec![0, 1], fib_seq(2));
        assert_eq!(vec![0, 1, 1], fib_seq(3));
        assert_eq!(vec![0, 1, 1, 2], fib_seq(4));
        assert_eq!(vec![0, 1, 1, 2, 3], fib_seq(5));
    }
}
