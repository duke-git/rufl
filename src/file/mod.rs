//! file mod contains several utility functions for handling file operation.
//!

mod create_file;
pub use create_file::*;

mod file_names;
pub use file_names::*;

mod read_to_buffer;
pub use read_to_buffer::*;

mod read_to_string;
pub use read_to_string::*;

mod read_to_lines;
pub use read_to_lines::*;
