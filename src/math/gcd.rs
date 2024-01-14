/// return greatest common divisor (GCD) of integers
///
/// # Arguments
///
/// * `numbers` - The integers to calculate gcd.
///
/// # Returns
///
/// Returns greatest common divisor.
///
/// # Examples
///
/// ```
/// use ruf::math;
///
/// assert_eq!(6, math::gcd(&vec![6, 12, 18]));
///
/// ```

pub fn gcd<T: crate::math::integer::Integer>(numbers: &Vec<T>) -> T {
    if numbers.is_empty() {
        return T::MIN;
    }

    let mut result = numbers[0];

    for n in &numbers[1..] {
        result = calculate_gcd(*n, result);
    }

    result
}

fn calculate_gcd<T: crate::math::integer::Integer>(a: T, b: T) -> T {
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
