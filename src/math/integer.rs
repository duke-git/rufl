// trait to constrain the type to integers.

// use std::cmp::Ord;
// use std::ops::Add;
// use std::ops::Div;
// use std::ops::Mul;
// use std::ops::Rem;
// use std::ops::Sub;
pub trait Integer:
    Sized
    + Copy
    + Ord
    + PartialOrd
    + Eq
    // + Rem<Output = Self>
    // + Add<Output = Self>
    // + Sub<Output = Self>
    // + Mul<Output = Self>
    // + Div<Output = Self>
    + 'static
{
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
}

macro_rules! impl_integer_for_isize {
    ($T:ty, $test_mod:ident) => {
        impl Integer for $T {
            const ZERO: $T = 0;
            const ONE: $T = 1;
            const MAX: $T = <$T>::max_value();
            const MIN: $T = <$T>::min_value();

            fn add(&self, other: &Self) -> Self {
                *self + *other
            }

            fn sub(&self, other: &Self) -> Self {
                *self - *other
            }

            fn mul(&self, other: &Self) -> Self {
                *self * *other
            }

            fn div(&self, other: &Self) -> Self {
                *self / *other
            }

            fn rem(&self, other: &Self) -> Self {
                *self % *other
            }

            fn cast(other: i128) -> Self {
                other as $T
            }
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
