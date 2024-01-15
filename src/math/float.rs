// trait to constrain the type to floats.
use std::any::Any;

pub trait Float: Sized + Copy + PartialOrd + PartialEq + 'static {
    const ZERO: Self;
    const ONE: Self;
    const MAX: Self;
    const MIN: Self;

    fn add(&self, other: &Self) -> Self;
    fn sub(&self, other: &Self) -> Self;
    fn mul(&self, other: &Self) -> Self;
    fn div(&self, other: &Self) -> Self;
    fn rem(&self, other: &Self) -> Self;

    fn cast(other: f64) -> Self;

    fn type_of(self) -> &'static str;
}

macro_rules! impl_float_for_f {
    ($T:ty) => {
        impl Float for $T {
            const ZERO: $T = 0 as $T;
            const ONE: $T = 1 as $T;
            const MAX: $T = <$T>::MAX;
            const MIN: $T = <$T>::MIN;

            #[inline]
            fn add(&self, other: &Self) -> Self {
                *self + *other
            }

            #[inline]
            fn sub(&self, other: &Self) -> Self {
                *self - *other
            }

            #[inline]
            fn mul(&self, other: &Self) -> Self {
                *self * *other
            }

            #[inline]
            fn div(&self, other: &Self) -> Self {
                *self / *other
            }

            #[inline]
            fn rem(&self, other: &Self) -> Self {
                *self % *other
            }

            #[inline]
            fn cast(other: f64) -> Self {
                other as $T
            }

            fn type_of(self) -> &'static str {
                if self.type_id() == std::any::TypeId::of::<f32>() {
                    "f32"
                } else if self.type_id() == std::any::TypeId::of::<f64>() {
                    "f64"
                } else {
                    ""
                }
            }
        }
    };
}

impl_float_for_f!(f32);
impl_float_for_f!(f64);
