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

    fn type_of(self) -> &'static str;
}

macro_rules! impl_number_for_data {
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

impl_number_for_data!(i8);
impl_number_for_data!(i16);
impl_number_for_data!(i32);
impl_number_for_data!(i64);
impl_number_for_data!(i128);
impl_number_for_data!(isize);

impl_number_for_data!(u8);
impl_number_for_data!(u16);
impl_number_for_data!(u32);
impl_number_for_data!(u64);
impl_number_for_data!(u128);
impl_number_for_data!(usize);

impl_number_for_data!(f32);
impl_number_for_data!(f64);
