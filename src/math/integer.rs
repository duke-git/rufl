// trait to constrain the type to integers.

use std::ops::Rem;
use std::ops::Sub;
use std::ops::Div;
use std::cmp::Ord;
pub trait Integer: Copy + Ord + Rem<Output = Self> + Sub<Output = Self> + Div<Output = Self> + 'static  {
    const ZERO: Self;
    const ONE: Self;
    const MAX: Self;
    const MIN: Self;
}

impl Integer for i8 {  
    const ZERO: i8 = 0;
    const ONE: i8 = 1;
    const MAX: i8 = std::i8::MAX;
    const MIN: i8 = std::i8::MIN;
}

impl Integer for i16 {  
    const ZERO: i16 = 0;
    const ONE: i16 = 1;
    const MAX: i16 = std::i16::MAX;
    const MIN: i16 = std::i16::MIN;
}

impl Integer for i32 {  
    const ZERO: i32 = 0;
    const ONE: i32 = 1;
    const MAX: i32 = std::i32::MAX;
    const MIN: i32 = std::i32::MIN;
}

impl Integer for i64 {  
    const ZERO: i64 = 0;  
    const ONE: i64 = 1;
    const MAX: i64 = std::i64::MAX;
    const MIN: i64 = std::i64::MIN;
}

impl Integer for i128 {  
    const ZERO: i128 = 0;
    const ONE: i128 = 1;
    const MAX: i128 = std::i128::MAX;
    const MIN: i128 = std::i128::MIN;
}

impl Integer for u8 {  
    const ZERO: u8 = 0;
    const ONE: u8 = 1;
    const MAX: u8 = std::u8::MAX;
    const MIN: u8 = std::u8::MIN;
}

impl Integer for u16 {  
    const ZERO: u16 = 0;
    const ONE: u16 = 1;
    const MAX: u16 = std::u16::MAX;
    const MIN: u16 = std::u16::MIN;
}

impl Integer for u32 {  
    const ZERO: u32 = 0;
    const ONE: u32 = 1;
    const MAX: u32 = std::u32::MAX;
    const MIN: u32 = std::u32::MIN;
}

impl Integer for u64 {  
    const ZERO: u64 = 0;
    const ONE: u64 = 1;
    const MAX: u64 = std::u64::MAX;
    const MIN: u64 = std::u64::MIN;
}

impl Integer for u128 {  
    const ZERO: u128 = 0;
    const ONE: u128 = 1;
    const MAX: u128 = std::u128::MAX;
    const MIN: u128 = std::u128::MIN;
}

impl Integer for isize {  
    const ZERO: isize = 0;
    const ONE: isize = 1;
    const MAX: isize = std::isize::MAX;
    const MIN: isize = std::isize::MIN;
}

impl Integer for usize {  
    const ZERO: usize = 0;
    const ONE: usize = 1;
    const MAX: usize = std::usize::MAX;
    const MIN: usize = std::usize::MIN;
}