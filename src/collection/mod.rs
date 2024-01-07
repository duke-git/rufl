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

mod reduce;
pub use reduce::*;

mod reduce_right;
pub use reduce_right::*;

mod index_of;
pub use index_of::*;

mod last_index_of;
pub use last_index_of::*;

mod max;
pub use max::*;

mod min;
pub use min::*;

mod insert_at;
pub use insert_at::*;

mod repalce_all;
pub use repalce_all::*;