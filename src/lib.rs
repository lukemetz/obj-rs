//! A parser for wavefront `.obj` and `.mtl` file format.

#![crate_name = "obj"]
#![unstable]
#![crate_type = "lib"]

#![deny(warnings)]
#![deny(missing_docs)]

pub use lex::ParseError;

mod lex;

pub mod mtl;
pub mod obj;
