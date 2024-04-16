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

mod count_chars;
pub use count_chars::*;

mod count_graphemes;
pub use count_graphemes::*;

mod count_words;
pub use count_words::*;

mod count_by;
pub use count_by::*;

mod hide;
pub use hide::*;

mod cut;
pub use cut::*;

mod index;
pub use index::*;

mod last_index;
pub use last_index::*;

mod lower_first;
pub use lower_first::*;

mod index_all;
pub use index_all::*;

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

mod remove_all;
pub use remove_all::*;

mod removen;
pub use removen::*;

mod remove_first;
pub use remove_first::*;

mod remove_last;
pub use remove_last::*;

mod snake_case;
pub use snake_case::*;

mod split_chars;
pub use split_chars::*;

mod split_words;
pub use split_words::*;

mod split_graphemes;
pub use split_graphemes::*;

mod starts_with_offset;
pub use starts_with_offset::*;

mod substring;
pub use substring::*;

mod title_case;
pub use title_case::*;

mod upper_first;
pub use upper_first::*;

mod is_alpha;
pub use is_alpha::*;

mod is_alphanumeric;
pub use is_alphanumeric::*;

mod is_uppercase;
pub use is_uppercase::*;

mod is_lowercase;
pub use is_lowercase::*;

mod is_numeric;
pub use is_numeric::*;

mod is_digit;
pub use is_digit::*;

mod is_ipv4;
pub use is_ipv4::*;

mod is_ipv6;
pub use is_ipv6::*;

mod is_email;
pub use is_email::*;

mod is_url;
pub use is_url::*;

mod is_dns;
pub use is_dns::*;

mod is_strong_password;
pub use is_strong_password::*;

mod add_commas;
pub use add_commas::*;
