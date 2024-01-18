// trait to constrain the type to floats.
use super::number;

pub trait Float: number::Number {
    fn cast(other: f64) -> Self;
    fn round_val(&self) -> Self;
    fn ceil_val(&self) -> Self;
    fn floor_val(&self) -> Self;
}

macro_rules! impl_float_for_number {
    ($T:ty) => {
        impl Float for $T {
            #[inline]
            fn cast(other: f64) -> Self {
                other as $T
            }

            #[inline]
            fn round_val(&self) -> Self {
                (*self).round()
            }

            #[inline]
            fn ceil_val(&self) -> Self {
                (*self).ceil()
            }

            #[inline]
            fn floor_val(&self) -> Self {
                (*self).floor()
            }
        }
    };
}

impl_float_for_number!(f32);
impl_float_for_number!(f64);
