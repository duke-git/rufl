// trait to constrain the type to integers.
use std::any::Any;

pub trait Integer: Sized + Copy + Ord + PartialOrd + Eq + 'static {
    const ZERO: Self;
    const ONE: Self;
    const MAX: Self;
    const MIN: Self;

    fn add(&self, other: &Self) -> Self;
    fn sub(&self, other: &Self) -> Self;
    fn mul(&self, other: &Self) -> Self;
    fn div(&self, other: &Self) -> Self;
    fn rem(&self, other: &Self) -> Self;

    fn cast(other: i128) -> Self;

    // fn type_of(self) -> &'static str;
}

macro_rules! impl_integer_for_isize {
    ($T:ty, $test_mod:ident) => {
        impl Integer for $T {
            const ZERO: $T = 0;
            const ONE: $T = 1;
            const MAX: $T = <$T>::max_value();
            const MIN: $T = <$T>::min_value();

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
            fn cast(other: i128) -> Self {
                other as $T
            }

            // fn type_of(self) -> &'static str {
            //     if self.type_id() == std::any::TypeId::of::<i8>() {
            //         "i8"
            //     } else if self.type_id() == std::any::TypeId::of::<i16>() {
            //         "i16"
            //     } else {
            //         ""
            //     }
            // }
        }
    };
}

impl_integer_for_isize!(i8, test_integer_i8);
impl_integer_for_isize!(i16, test_integer_i16);
impl_integer_for_isize!(i32, test_integer_i32);
impl_integer_for_isize!(i64, test_integer_i64);
impl_integer_for_isize!(i128, test_integer_i128);
impl_integer_for_isize!(isize, test_integer_isize);

impl_integer_for_isize!(u8, test_integer_u8);
impl_integer_for_isize!(u16, test_integer_u16);
impl_integer_for_isize!(u32, test_integer_u32);
impl_integer_for_isize!(u64, test_integer_u64);
impl_integer_for_isize!(u128, test_integer_u128);
impl_integer_for_isize!(usize, test_integer_usize);
