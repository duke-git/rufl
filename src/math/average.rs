use super::number::Number;
/// Calculats the average value of number vector.
///
/// # Arguments
///
/// * `numbers` - The number vector to calculate average.
///
/// # Returns
///
/// Average value of number vector.
///
/// # Examples
///
/// ```
/// use ruf::math;
///
/// assert_eq!(2.5, math::average(&vec![1, 2, 3, 4]));
///
/// assert_eq!(2.1999999999999997, math::average(&vec![1.1, 2.2, 3.3]));
///
/// ```

pub fn average<T: Number>(numbers: &[T]) -> f64 {
    if numbers.len() == 0 {
        return 0 as f64;
    }

    let total = crate::math::sum(numbers);
    let count = numbers.len() as f64;

    total.to_f64() / count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_average() {
        assert_eq!(2.5, average(&vec![1, 2, 3, 4]));
        assert_eq!(3.0, average(&vec![1.0, 2.0, 3.0, 4.0, 5.0]));

        assert_eq!(2.1999999999999997, average(&vec![1.1, 2.2, 3.3])); // IEEE754 float point: https://floating-point-gui.de/

        assert_eq!(2.2, crate::math::round(average(&vec![1.1, 2.2, 3.3]), 1));
    }
}
