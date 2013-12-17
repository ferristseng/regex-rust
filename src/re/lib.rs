#[link(
  name = "re",
  package_id = "re",
  vers = "0.1",
  url = "https://github.com/ferristseng/regex-rust/tree/master"
 )];

#[license = "MIT"];
#[comment = "Regular Expression Engine in Rust"];

extern mod extra;

pub use regexp::UncompiledRegexp;

mod error;
mod exec;
mod compile;
mod parse;
mod state;

pub mod regexp;

// this is test code
fn main() {
  let mut re = UncompiledRegexp::new("(?:http(s)?:\\/\\/)?(www\\.)?([a-zA-Z0-9_.]+)\\.(com|org|net|edu)\\/?");
  let ma = re.run("http://ferristseng.comuASDAFASFASBVZKXJVBKZXBVKJZBXVKBZXV");

  match ma {
    Some(matched) => {
      for i in range(0, matched.groups.len()) {
        println(matched.group(i));
      } 
    }
    None => { }
  }

  let mut re = UncompiledRegexp::new("[al-obc]+");
  let ma = re.run("almocb");

  match ma {
    Some(matched) => {
      println("Found Match");
      for i in range(0, matched.groups.len()) {
        println(matched.group(i));
      } 
    }
    None => { }
  }
}
