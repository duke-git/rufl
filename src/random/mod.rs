//! random mod contains several utility functions for generating random number or string.
//!

mod numberic_str;
pub use numberic_str::*;

mod upper;
pub use upper::*;

mod lower;
pub use lower::*;

mod alphabeta;
pub use alphabeta::*;

mod alpha_number;
pub use alpha_number::*;

mod symbol;
pub use symbol::*;

mod generate;
use generate::*;

static NUMBERS: &str = "0123456789";
static LOWER_LETTERS: &str = "abcdefghijklmnopqrstuvwxyz";
static UPPER_LETTERS: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";
static LETTERS: &str = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";
static ALPHANUMERIC: &str = "0123456789abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";
static SYMBOL_CHARS: &str = "!@#$%^&*()_+-=[]{}|;':\",./<>?";
static ALLCHARS: &str =
    "0123456789abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ!@#$%^&*()_+-=[]{}|;':\",./<>?";
