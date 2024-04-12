//! file mod contains several utility functions for handling file operation.
//!

mod create;
pub use create::*;

mod clear;
pub use clear::*;

mod copy_dirs;
pub use copy_dirs::*;

mod file_names;
pub use file_names::*;

mod read_to_buffer;
pub use read_to_buffer::*;

mod read_to_string;
pub use read_to_string::*;

mod read_to_lines;
pub use read_to_lines::*;

mod is_symlink;
pub use is_symlink::*;

mod write_to;
pub use write_to::*;
