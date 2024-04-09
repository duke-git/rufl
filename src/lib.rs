//! Ruf is an util function library for rust.
//! It provides a series of useful functions to make your rust development easier.
//!
#[cfg(feature = "collection")]
pub mod collection;
#[cfg(feature = "eventbus")]
pub mod eventbus;

#[cfg(feature = "file")]
pub mod file;

#[cfg(feature = "math")]
pub mod math;

#[cfg(feature = "random")]
pub mod random;

#[cfg(feature = "string")]
pub mod string;
