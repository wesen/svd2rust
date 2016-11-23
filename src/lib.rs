#![recursion_limit = "100"]

extern crate inflections;
extern crate svd_parser as svd;
#[macro_use]
extern crate quote;
extern crate syn;
extern crate regex;
extern crate libc;

pub mod list;
pub mod generate;

pub mod tty;

pub use generate::{gen_peripheral};
pub use list::{list_peripheral, create_regex, match_peripheral};

