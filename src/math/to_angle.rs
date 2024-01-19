/// Converts radian value to angle value.
///
/// # Arguments
///
/// * `radian` - The radian value to contert.
///
///
/// # Returns
///
/// Angle value.
///
/// # Examples
///
/// ```
/// use ruf::math;
///
/// assert_eq!(180_f64, math::to_angle(std::f64::consts::PI));
///
/// assert_eq!(90_f64, math::to_angle(std::f64::consts::PI / 2.0));
///
/// assert_eq!(45_f64, math::to_angle(std::f64::consts::PI / 4.0));
///
/// ```

pub fn to_angle(radian: f64) -> f64 {
    radian * (180.0 / std::f64::consts::PI)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_to_angle() {
        assert_eq!(0_f64, to_angle(0_f64));
        assert_eq!(180_f64, to_angle(std::f64::consts::PI));
        assert_eq!(90_f64, to_angle(std::f64::consts::PI / 2.0));
        assert_eq!(45_f64, to_angle(std::f64::consts::PI / 4.0));
    }
}
