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

mod error;
mod compile;
mod parse;
mod state;
mod test;
mod charclass;
mod exec;

pub mod result;
pub mod regexp;

// this is test code
fn main() {
  let mut re = UncompiledRegexp::new("(?:http(s)?:\\/\\/)?(www\\.)?([a-zA-Z0-9_.]+)\\.(com|org|net|edu)\\/?");
  let ma = re.exec("http://ferristseng.comuASDAFASFASBVZKXJVBKZXBVKJZBXVKBZXV");

  match ma {
    Ok(result) => {
      match result {
        Some(matched) => {
          println("Found Match");
          println(matched.matched());
          for i in range(0, matched.groups.len()) {
            println(matched.group(i));
          } 
        }
        None => { }
      }
    }
    Err(e) => println(e.to_str())
  }

  let mut re = UncompiledRegexp::new("a{5,8}");
  let ma = re.exec("aaaa");

  match ma {
    Ok(result) => {
      match result {
        Some(matched) => {
          println("Found Match");
          println(matched.matched());
          for i in range(0, matched.groups.len()) {
            println(matched.group(i));
          } 
        }
        None => { }
      }
    }
    Err(e) => println(e.to_str())
  }

  let mut re = UncompiledRegexp::new("①②③");
  let ma = re.exec("①②③");

  match ma {
    Ok(result) => {
      match result {
        Some(matched) => {
          println("Found Match");
          println(matched.matched());
          for i in range(0, matched.groups.len()) {
            println(matched.group(i));
          } 
        }
        None => { }
      }
    }
    Err(e) => println(e.to_str())
  }
}
