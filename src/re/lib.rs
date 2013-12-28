#[crate_id = "rustre#0.1.1"];
#[link(
  name = "re",
  package_id = "re",
  vers = "0.1.1",
  url = "https://github.com/ferristseng/regex-rust/tree/master"
 )];

#[feature(globs)];
#[feature(macro_rules)];

#[allow(dead_code)];

#[license = "MIT"];
#[comment = "Regular Expression Engine in Rust"];

extern mod extra;

pub use regexp::UncompiledRegexp;

mod test;
mod exec;
mod error;
mod parse;
mod state;
mod compile;
mod charclass;

pub mod result;
pub mod regexp;

fn main() {
}
