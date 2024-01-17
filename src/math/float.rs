// trait to constrain the type to floats.
use super::number;

pub trait Float: number::Number {
    fn cast(other: f64) -> Self;
    fn to_f32(&self) -> f32;
    fn to_f64(&self) -> f64;
    fn round_val(&self) -> Self;
}

macro_rules! impl_float_for_number {
    ($T:ty) => {
        impl Float for $T {
            #[inline]
            fn cast(other: f64) -> Self {
                other as $T
            }

            #[inline]
            fn to_f32(&self) -> f32 {
                *self as f32
            }

            #[inline]
            fn to_f64(&self) -> f64 {
                *self as f64
            }

            #[inline]
            fn round_val(&self) -> Self {
                (*self).round()
            }
        }
    };
}

impl_float_for_number!(f32);
impl_float_for_number!(f64);
