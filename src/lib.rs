//! A parser for wavefront `.obj` and `.mtl` file format.

#![unstable]
#![doc(html_root_url = "https://simnalamburt.github.io/obj-rs")]

#![deny(warnings)]
#![deny(missing_docs)]

pub use lex::ParseError;

mod lex;

pub mod mtl;
pub mod obj;
