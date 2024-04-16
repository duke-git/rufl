use super::number::Number;

/// Calculats the sum of number vector.
///
/// # Arguments
///
/// * `numbers` - The number vector to calculate sum.
///
/// # Returns
///
/// sum of number vector.
///
/// # Examples
///
/// ```
/// use rufl::math;
///
/// assert_eq!(10, math::sum(&vec![1, 2, 3, 4]));
///
/// assert_eq!(6.6, math::sum(&vec![1.1, 2.2, 3.3]));
///
/// ```

pub fn sum<T: Number>(numbers: &[T]) -> T {
    let mut total: T = Number::ZERO;

    for n in numbers {
        total = total.add(n)
    }

    total
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sum() {
        assert_eq!(0, sum(&vec![-1, 1]));
        assert_eq!(10, sum(&vec![1, 2, 3, 4]));
        assert_eq!(6.6, sum(&vec![1.1, 2.2, 3.3])); // 6.5999999999999996 IEEE754 float point: https://floating-point-gui.de/
    }
}
