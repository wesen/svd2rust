#![recursion_limit = "100"]

extern crate inflections;
extern crate svd_parser as svd;
#[macro_use]
extern crate quote;
extern crate syn;

pub mod generate;

pub use generate::{gen_peripheral};

