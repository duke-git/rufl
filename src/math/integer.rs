// trait to constrain the type to integers.
use super::number;

pub trait Integer: number::Number + Ord + Eq + 'static {
    fn cast(other: i128) -> Self;
}

macro_rules! impl_integer_for_number {
    ($T:ty) => {
        impl Integer for $T {
            #[inline]
            fn cast(other: i128) -> Self {
                other as $T
            }
        }
    };
}

impl_integer_for_number!(i8);
impl_integer_for_number!(i16);
impl_integer_for_number!(i32);
impl_integer_for_number!(i64);
impl_integer_for_number!(i128);
impl_integer_for_number!(isize);

impl_integer_for_number!(u8);
impl_integer_for_number!(u16);
impl_integer_for_number!(u32);
impl_integer_for_number!(u64);
impl_integer_for_number!(u128);
impl_integer_for_number!(usize);
