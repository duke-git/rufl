// trait to constrain the type to floats.
use super::number;

pub trait Float: number::Number {
    fn cast(other: f64) -> Self;
}

macro_rules! impl_float_for_f {
    ($T:ty) => {
        impl Float for $T {
            #[inline]
            fn cast(other: f64) -> Self {
                other as $T
            }
        }
    };
}

impl_float_for_f!(f32);
impl_float_for_f!(f64);
