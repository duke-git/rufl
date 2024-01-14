/// return least common multiple (lcm) of integers.
///
/// # Arguments
///
/// * `numbers` - The integers to calculate lcm.
///
/// # Returns
///
/// Common multiple of integers.
///
/// # Examples
///
/// ```
/// use ruf::math;
///
/// assert_eq!(6, math::lcm(&vec![6]));
/// 
/// assert_eq!(168, math::lcm(&vec![6, 7, 8]));
///
/// ```

pub fn lcm<T: crate::math::integer::Integer>(numbers: &Vec<T>) -> T {
    if numbers.is_empty() {
        return T::MIN;
    }

    let mut result = numbers[0];

    for n in &numbers[1..] {
        result = calculate_lcm(*n, result);
    }

    result
}

pub(crate) fn calculate_lcm<T: crate::math::integer::Integer>(a: T, b: T) -> T {
    a * b / crate::math::calculate_gcd(a, b)
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lcm() {
        assert_eq!(6, lcm(&vec![6]));
        assert_eq!(18, lcm(&vec![9, 6]));
        assert_eq!(168, lcm(&vec![6, 7, 8]));
    }
}
