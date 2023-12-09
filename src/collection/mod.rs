//! collection mod contains several utility functions to manipulate collection data type.(array, vector)
//!

mod all_match;
pub use all_match::*;

mod none_match;
pub use none_match::*;

mod some_match;
pub use some_match::*;

mod filter;
pub use filter::*;

mod map;
pub use map::*;

mod filter_map;
pub use filter_map::*;
