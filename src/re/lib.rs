#[link(
  name = "re",
  package_id = "re",
  vers = "0.1",
  url = "https://github.com/ferristseng/regex-rust/tree/master"
 )];

#[license = "MIT"];
#[comment = "Regular Expression Engine in Rust"];
#[crate_type = "lib"];

extern mod extra;

pub use regexp::UncompiledRegexp;

mod error;
mod exec;
mod compile;
mod parse;
mod state;

pub mod regexp;
