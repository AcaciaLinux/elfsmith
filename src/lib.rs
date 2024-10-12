mod packable;
pub use packable::*;

mod ident;
pub use ident::*;

mod header;
pub use header::*;

mod program;
pub use program::*;

mod section;
pub use section::*;

mod blob;
pub use blob::*;

mod file;
pub use file::*;

use core::str;

/// Creates a string slice from a pointer to a null terminated
/// C-style string
/// # Arguments
/// * `ptr` - The pointer to traverse for the string slice
///
/// Finds the length of the string by iterating over it until
/// the 0 byte and assumes a UTF-8 string from that
pub unsafe fn str_from_null_terminated(ptr: *const u8) -> &'static str {
    let mut len = 0usize;

    loop {
        if *(ptr.wrapping_add(len)) == 0 {
            break;
        } else {
            len += 1;
        }
    }

    let slice = core::slice::from_raw_parts(ptr, len);
    str::from_utf8_unchecked(slice)
}
