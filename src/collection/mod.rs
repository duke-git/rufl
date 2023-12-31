//! collection mod contains several utility functions to manipulate collection data type.(array, vector)
//!

mod all_match;
pub use all_match::*;

mod none_match;
pub use none_match::*;

mod some_match;
pub use some_match::*;

mod count;
pub use count::*;

mod count_by;
pub use count_by::*;

mod find;
pub use find::*;

mod find_last;
pub use find_last::*;

mod filter;
pub use filter::*;

mod map;
pub use map::*;

mod filter_map;
pub use filter_map::*;

mod difference;
pub use difference::*;

mod difference_by;
pub use difference_by::*;

mod difference_with;
pub use difference_with::*;

mod unique;
pub use unique::*;

mod unique_by;
pub use unique_by::*;

mod union;
pub use union::*;

mod union_by;
pub use union_by::*;

mod intersection;
pub use intersection::*;

mod shuffle;
pub use shuffle::*;
