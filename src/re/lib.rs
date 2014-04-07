#![crate_id = "rustre#0.1.1"]
#![feature(globs)]
#![feature(macro_rules)]

#![allow(dead_code)]

#![license = "MIT"]
#![comment = "Regular Expression Engine in Rust"]

pub use regexp::UncompiledRegexp;

mod test;
mod exec;
mod error;
mod parse;
mod state;
mod compile;
mod charclass;
mod unicode;

pub mod result;
pub mod regexp;

fn main() {
}
