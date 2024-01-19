/// Converts angle value to radian value.
///
/// # Arguments
///
/// * `angle` - The angle value to contert.
///
///
/// # Returns
///
/// Radian value.
///
/// # Examples
///
/// ```
/// use ruf::math;
///
/// assert_eq!(0.7853981633974483, math::to_radian(45_f64));
///
/// assert_eq!(1.5707963267948966, math::to_radian(90_f64));
///
/// assert_eq!(3.141592653589793, math::to_radian(180_f64));
///
/// ```

pub fn to_radian(angle: f64) -> f64 {
    angle * (std::f64::consts::PI / 180.0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_to_radian() {
        assert_eq!(0_f64, to_radian(0_f64));
        assert_eq!(0.7853981633974483, to_radian(45_f64));
        assert_eq!(1.5707963267948966, to_radian(90_f64));
        assert_eq!(3.141592653589793, to_radian(180_f64));
    }
}
