//! random mod contains several utility functions for generating random number or string.
//!

// static NUMBER: Vec<&str> = vec!["0", "1", "2"];
static NUMBERS: &str = "0123456789";
static LOWWER_LETTERS: &str = "abcdefghijklmnopqrstuvwxyz";
static UPPER_LETTERS: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";
static LETTERS: &str = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";
static ALPHANUMERIC: &str = "0123456789abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";
static SYMBOL_CHARS: &str = "!@#$%^&*()_+-=[]{}|;':\",./<>?";

static ALLCHARS: &str =
    "0123456789abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ!@#$%^&*()_+-=[]{}|;':\",./<>?";

mod numberic_str;
pub use numberic_str::*;

mod generate;
use generate::*;

// use internal::*;
