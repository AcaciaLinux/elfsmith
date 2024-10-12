//! All the used structs, enum and traits that form the
//! core model of `elfsmith`
mod ident;
pub use ident::*;

mod header;
pub use header::*;

mod program;
pub use program::*;

mod section;
pub use section::*;
