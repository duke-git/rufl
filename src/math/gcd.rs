use std::ops::Rem;

use super::integer::Integer;

/// return greatest common divisor (GCD) of integers.
///
/// # Arguments
///
/// * `numbers` - The integers to calculate gcd.
///
/// # Returns
///
/// Greatest common divisor of integers.
///
/// # Examples
///
/// ```
/// use ruf::math;
///
/// assert_eq!(5, math::gcd(&vec![5]));
///
/// assert_eq!(6, math::gcd(&vec![6, 12, 18]));
///
/// ```

pub fn gcd<T>(numbers: &Vec<T>) -> T
where
    T: Integer + Rem<Output = T>,
{
    if numbers.is_empty() {
        return T::MIN;
    }

    let mut result = numbers[0];

    for n in &numbers[1..] {
        result = calculate_gcd(*n, result);
    }

    result
}

pub(crate) fn calculate_gcd<T: Integer + Rem<Output = T>>(a: T, b: T) -> T {
    if a == T::ZERO {
        b
    } else if b == T::ZERO {
        a
    } else {
        calculate_gcd(b, a % b)
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gcd() {
        assert_eq!(6, gcd(&vec![6]));
        assert_eq!(6, gcd(&vec![12, 6]));
        assert_eq!(6, gcd(&vec![48, 18, 6]));
    }
}
