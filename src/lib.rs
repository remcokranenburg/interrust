// https://ruslanspivak.com/lsbasi-part9/

extern crate peg;

mod parser;
mod interpreter;

pub use parser::rust_parser;
pub use interpreter::eval;
