use exec::Prog;
use result::Match;
use parse::parse;
use compile::compile_recursive;
use error::ParseError::*;

/**
 * Uncompiled regular expression. 
 */
pub struct UncompiledRegexp {
  prog: Prog
}

/**
 * Constructors
 */
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

impl UncompiledRegexp { 
  /**
   * Checks if the beginning of the input string 
   * contains a match, and returns it.
   */
  pub fn exec(&self, input: &str) -> Option<Match> {
    self.prog.run(input, 0)
  }
  /**
   * Finds the first occurrence of the pattern in the 
   * input string and returns it.
   */
  pub fn search(&self, input: &str) -> Option<Match> {
    let len = input.len();

    for start in range(0, len + 1) {
      match self.prog.run(input, start) {
        Some(m) => return Some(m),
        None => ()
      }
    }

    None
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
}
