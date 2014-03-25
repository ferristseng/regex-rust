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

  // pub fn find_all(&self, input: &str) -> Option<Match> {
  //
  // }

  // pub fn find_iter(&self, input: &str) -> Option<Match> {
  // 
  // }

  // pub fn replace(&self, input: &str) -> Option<Match> {
  // 
  // }

  // pub fn replacen(&self, input: &str) -> Option<Match> {
  // 
  // }

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
