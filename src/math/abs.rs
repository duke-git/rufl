use super::float;
use super::integer;

/// Returns the absolute value of `number`.
///
/// # Arguments
///
/// * `number` - The integer to calculate absolute value.
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
/// assert_eq!(5, math::abs(&vec![5]));
///
/// assert_eq!(6, math::abs(&vec![6, 12, 18]));
///
/// ```

// pub fn abs<T: float::Float + integer::Integer>(number: T) -> Result<T, String> {
//     if number.type_of() == "f32" || number.type_of() == "f64" {
//         abs_float(number)
//     } else {
//         abs_integer(number)
//     }
// }

fn abs_integer<T: integer::Integer>(number: T) -> Result<T, String> {
    if number == T::MIN {
        Err("Overflow: minimum value can't be representable".to_string())
    } else if number < T::ZERO {
        Ok(T::ZERO.sub(&number))
    } else {
        Ok(number)
    }
}

fn abs<T: float::Float>(number: T) -> Result<T, String> {
    println!("data type is {}", number.type_of());
    if number == T::MIN {
        Err("Overflow: minimum value can't be representable".to_string())
    } else if number < T::ZERO {
        Ok(T::ZERO.sub(&number))
    } else {
        Ok(number)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_abs() {
        assert_eq!(0.1 as f32, abs(-0.1 as f32).unwrap());
        assert_eq!(0.2, abs(-0.2).unwrap());
    }
}
