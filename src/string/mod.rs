//! string mod contains several utility functions for handling string.
//!

mod after;
pub use after::*;

mod after_last;
pub use after_last::*;

mod before;
pub use before::*;

mod before_last;
pub use before_last::*;

mod camel_case;
pub use camel_case::*;

mod capitalize;
pub use capitalize::*;

mod cut;
pub use cut::*;

mod kebab_case;
pub use kebab_case::*;

mod pad;
pub use pad::*;

mod pad_start;
pub use pad_start::*;

mod pad_end;
pub use pad_end::*;

mod pascal_case;
pub use pascal_case::*;

mod remove;
pub use remove::*;

mod remove_first;
pub use remove_first::*;

mod remove_last;
pub use remove_last::*;

mod snake_case;
pub use snake_case::*;

mod split_words;
pub use split_words::*;
