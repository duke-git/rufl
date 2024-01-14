use super::integer::Integer;

/// Checks if number is prime or not.
///
/// # Arguments
///
/// * `number` - The number to perform check.
///
/// # Returns
///
/// True if the number is prime.
///
/// # Examples
///
/// ```
/// use ruf::math;
///
/// assert_eq!(false, math::is_prime(1));
///
/// assert_eq!(true, math::is_prime(2));
/// 
/// assert_eq!(true, math::is_prime(3 as usize));
/// 
/// assert_eq!(false, math::is_prime(4 as u8));
/// 
/// ```

pub fn is_prime<T: Integer>(number: T) -> bool {
    if number <= Integer::cast(1 as i128) {
        return false;
    }

    let mut i = 2;

    while <T as Integer>::cast((i * i) as i128) <= number {
        if number.rem(&Integer::cast(i as i128)) == Integer::cast(0 as i128) {
            return false;
        }
        i += 1;
    }

    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_prime() {
        assert_eq!(false, is_prime(1));
        assert_eq!(true, is_prime(2));
        assert_eq!(true, is_prime(3 as u8));
        assert_eq!(false, is_prime(4 as i8));
    }
}
