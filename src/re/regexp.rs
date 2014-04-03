use exec::{ExecStrategy, PikeVM};
use compile::Instruction;
use result::Match;
use parse::parse;
use compile::compile_recursive;
use error::ParseError::*;

/// Uncompiled regular expression. 
pub struct UncompiledRegexp {
  prog: ~[Instruction]
}

/// Constructors
impl UncompiledRegexp {
  pub fn new(s: &str) -> Result<UncompiledRegexp, ParseCode> {
    match parse(s) {
      Ok(ref expr) => {
        let prog = compile_recursive(expr);
        Ok(UncompiledRegexp { prog: prog })
      }
      Err(e) => Err(e)
    }
  }
}

/// TODO:
/// The API needs some work.
/// Allow for other implementations to be used?
impl UncompiledRegexp { 
  /// Checks if the beginning of the input string 
  /// contains a match, and returns it.
  pub fn exec(&self, input: &str) -> Option<Match> {
    let strat = PikeVM::new(self.prog, 0);
    match strat.run(input, 0) {
      Some(t) => {
        Some(Match::new(0, t.end, input, t.captures))
      }
      None => None
    }
  }
  /// Finds the first occurrence of the pattern in the 
  /// input string and returns it.
  pub fn search(&self, input: &str) -> Option<Match> {
    let len = input.len();
    let strat = PikeVM::new(self.prog, 0);


    // IGNORE THIS BELOW. It's very dumb fake testing for 
    // find_all until we actually implement a better test suite.

    // let blah = self.find_all(input);
    // match blah {
    //   Some(x) => { 
    //     print!("Matched {:u} elements: ", x.len());
    //     for elem in x.iter() {
    //       print!("{:s}, ", elem.to_str());
    //     }
    //     println!("");
    //   }
    //   None => { println!("Matched 0 elements"); }
    // }


    for start in range(0, len + 1) {
      match strat.run(input, start) {
        Some(t) => {
          return Some(Match::new(start, t.end, input, t.captures))
        }
        None => ()
      }
    }

    None
  }

  // pub fn split(&self, input: &str) -> Option<Match> {
  //
  // }

  pub fn find_all(&self, input: &str) -> Option<~[Match]> {
    let mut matches : ~[Match] = ~[];

    let len = input.len();
    let strat = PikeVM::new(self.prog, 0); 

    for start in range(0, len + 1) {
      match strat.run(input, start) {
        Some(t) => {
          matches.push(Match::new(start, t.end, input, t.captures))
        }
        None => ()
      }
    }

    if (matches.len() != 0) {
      return Some(matches);
    }
    else {
      return None;
    }
  }

  // pub fn find_iter(&self, input: &str) -> Option<Match> {
  // 
  // }

  pub fn replace(&self, input: &str, replaceWith: &str) -> ~str {
    let len = input.len();
    let strat = PikeVM::new(self.prog, 0); 
    let mut replaced = input.to_owned();

    for start in range(0, len + 1) {
      match strat.run(replaced, start) {
        Some(t) => {
          replaced = format!("{:s}{:s}{:s}", replaced.slice_to(start), replaceWith, replaced.slice_from(t.end));
        }
        None => ()
      }
    }

    replaced
  }

  // pub fn replacen(&self, input: &str) -> Option<Match> {
  // 
  // }

}

#[cfg(test)]
mod library_functions_test {
  use super::*;
  use error::ParseError::*;

  macro_rules! test_replace(
    ($input: expr, $re: expr, $replaceWith: expr, $expect: expr) => (
      {
        let re = match UncompiledRegexp::new($re) {
          Ok(regex) => regex,
          Err(e) => fail!(e)
        };
        let result = re.replace($input, $replaceWith);
        let ok = if result == ~$expect {true} else {false};
        assert!(ok); 
      }
    );
  )

  #[test]
  fn test_replace_1() {
    test_replace!("abaaacaabaaaccdab", "a*ba*", "", "cccd");
  }

  #[test]
  fn test_replace_2() {
    test_replace!("abaaacaabaaacca", "a*ba{1,}", "", "ccca");
  }
}


#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn parse_alternation_ok_test() {
    assert!(UncompiledRegexp::new("a|b").is_ok());
  }

  #[test]
  fn parse_concatenation_ok_test() {
    assert!(UncompiledRegexp::new("a(bc)d").is_ok());
  }

  #[test]
  fn parse_char_class_ok_test() {
    assert!(UncompiledRegexp::new("[a-zABC!@#]]]").is_ok());
  }

  #[test]
  fn parse_capture_ok_test() {
    assert!(UncompiledRegexp::new("(hel(ABC)ok)").is_ok());
  }

  #[test]
  fn parse_capture_fail_test() {
    assert!(UncompiledRegexp::new("(hel(ABC)ok").is_err());
  }

  #[test]
  fn search_group_fetch() {
    match UncompiledRegexp::new("(?P<hello>d)") {
      Ok(regex) => {
        match regex.search("dhfs") {
          Some(m) => {
            match m.group_by_name("hello") {
              Some(result) => {
                assert_eq!(result, ~"d");
              }
              None => {
                fail!("Failed to find a group with a match");
              }
            }
          }
          None => {
            fail!("Didn't match a group when expected");
          }
        }        
      }
      Err(error) => {
        fail!(error.to_str());
      }
    }
  }
}
