// trait to constrain the type to Number.
use std::any::Any;

pub trait Number: Sized + Copy + PartialOrd + PartialEq + 'static {
    const ZERO: Self;
    const ONE: Self;
    const MAX: Self;
    const MIN: Self;

    fn add(&self, other: &Self) -> Self;
    fn sub(&self, other: &Self) -> Self;
    fn mul(&self, other: &Self) -> Self;
    fn div(&self, other: &Self) -> Self;
    fn rem(&self, other: &Self) -> Self;

    fn to_f32(&self) -> f32;
    fn to_f64(&self) -> f64;

    fn type_of(self) -> &'static str;
}

macro_rules! impl_number_for_type {
    ($T:ty) => {
        impl Number for $T {
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
            fn to_f32(&self) -> f32 {
                *self as f32
            }

            #[inline]
            fn to_f64(&self) -> f64 {
                *self as f64
            }

            #[inline]
            fn type_of(self) -> &'static str {
                if self.type_id() == std::any::TypeId::of::<i8>() {
                    "i8"
                } else if self.type_id() == std::any::TypeId::of::<i16>() {
                    "i16"
                } else if self.type_id() == std::any::TypeId::of::<i32>() {
                    "i32"
                } else if self.type_id() == std::any::TypeId::of::<i64>() {
                    "i64"
                } else if self.type_id() == std::any::TypeId::of::<i128>() {
                    "i128"
                } else if self.type_id() == std::any::TypeId::of::<isize>() {
                    "isize"
                } else if self.type_id() == std::any::TypeId::of::<u8>() {
                    "u8"
                } else if self.type_id() == std::any::TypeId::of::<u16>() {
                    "u16"
                } else if self.type_id() == std::any::TypeId::of::<u32>() {
                    "u32"
                } else if self.type_id() == std::any::TypeId::of::<u64>() {
                    "u64"
                } else if self.type_id() == std::any::TypeId::of::<u128>() {
                    "u128"
                } else if self.type_id() == std::any::TypeId::of::<usize>() {
                    "usize"
                } else if self.type_id() == std::any::TypeId::of::<f32>() {
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

impl_number_for_type!(i8);
impl_number_for_type!(i16);
impl_number_for_type!(i32);
impl_number_for_type!(i64);
impl_number_for_type!(i128);
impl_number_for_type!(isize);

impl_number_for_type!(u8);
impl_number_for_type!(u16);
impl_number_for_type!(u32);
impl_number_for_type!(u64);
impl_number_for_type!(u128);
impl_number_for_type!(usize);

impl_number_for_type!(f32);
impl_number_for_type!(f64);
