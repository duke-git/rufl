//! math mod contains several utility functions for handling mathematical calculations.
//!

mod float;
mod integer;
mod number;

mod abs;
pub use abs::*;

mod sqrt;
pub use sqrt::*;

mod gcd;
pub use gcd::*;

mod lcm;
pub use lcm::*;

mod fib_nth;
pub use fib_nth::*;

mod round;
pub use round::*;

mod round_up;
pub use round_up::*;

mod round_down;
pub use round_down::*;

mod is_prime;
pub use is_prime::*;

mod factorial;
pub use factorial::*;

mod sum;
pub use sum::*;

mod percent;
pub use percent::*;
