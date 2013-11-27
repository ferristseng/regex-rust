extern mod extra;

use exec::Prog;
use parse::parse_recursive;
use state::ParseState;
use compile::{Instruction, compile_recursive};
use error::ParseError::*;

mod parse;
mod state;
mod compile;
mod error;
mod exec;

struct UncompiledRegexp {
  input: ~str
}

struct CompiledRegexp;

impl UncompiledRegexp {
  pub fn new(s: &str) -> UncompiledRegexp {
    UncompiledRegexp { input: s.clone().to_owned() }
  }
}

impl UncompiledRegexp { 
  // we should hide the underlying parsing algorithm
  // from the user
  fn parse(&mut self) -> Result<state::Regexp, ParseCode> {
    let mut ps = ParseState::new();
    match parse_recursive(&mut self.input, &mut ps) {
      ParseOk => {
        ps.pop()
      }
      e => {
        println(e.to_str());
        Err(e)
      }
    }
  }
  fn compile(&mut self) {
    let mut stack: ~[Instruction] = ~[];
    match self.parse() {
      Ok(ref re) => {
        compile_recursive(re, &mut stack);
        Prog::new(stack, "abcdefghijklmnopaaa").run(); 
      }
      Err(e) => {
        println(e.to_str());
      }
    };
  }
}

fn main() {
  println("--Case 0--");
  UncompiledRegexp::new("abc").compile();

  println("--Case 1--");
  UncompiledRegexp::new("a|b").compile();

  // println("--Case 2--");
  // UncompiledRegexp::new("a|b|c").compile();

  // println("--Case 3--");
  // UncompiledRegexp::new("a|(Bcf)|dez").compile();

  // println("--Case 4--");
  // //UncompiledRegexp::new("abc*|d").parse();

  // println("--Case 5--");
  // //UncompiledRegexp::new("io(ab|c)*zz|(bcd)*").parse();

  // println("--Case 6--");
  // //UncompiledRegexp::new("あ(ab(cd|d)|e)|f").parse();

  println("--Case 7--");
  //UncompiledRegexp::new("[[A-Z]0-9(fgh)]]]|[abc]").parse();
  UncompiledRegexp::new("[1-]]").parse();

  println("--Case 8--");
  UncompiledRegexp::new("(abc){1,}").parse();

  println("--Case 9--");
  UncompiledRegexp::new("abc{3,4}?").parse();
  
  println("--Case 10--");
  UncompiledRegexp::new("a|b{3}").parse();

  println("--Case 11--");
  UncompiledRegexp::new("a{4,3}?").parse();

  println("--Case 12--");
  UncompiledRegexp::new("[C[e-h]arlemange]|bs|c").compile();

  println("--Case 13--");
  UncompiledRegexp::new("[^aA-ZA]").compile();

  println("--Case 14--");
  UncompiledRegexp::new("[^\U00000000-\U0010FFFF]").parse();

  println("--Case 15--");
  UncompiledRegexp::new("[^a-f]").parse();
  
  println("--Case 16--");
  UncompiledRegexp::new("a?").compile();

  println("--Case 17--");
  UncompiledRegexp::new("(ABC)+").compile();

  println("--Case 18--");
  UncompiledRegexp::new("(A|B)*").compile();
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn parse_alternation_ok_test() {
    assert!(UncompiledRegexp::new("a|b").parse().is_ok());
  }

  #[test]
  fn parse_concatenation_ok_test() {
    assert!(UncompiledRegexp::new("a(bc)d").parse().is_ok());
  }

  #[test]
  fn parse_char_class_ok_test() {
    assert!(UncompiledRegexp::new("[a-zABC!@#]]]").parse().is_ok());
  }

  #[test]
  fn parse_capture_ok_test() {
    assert!(UncompiledRegexp::new("(hel(ABC)ok)").parse().is_ok());
  }

  #[test]
  fn parse_capture_fail_test() {
    assert!(UncompiledRegexp::new("(hel(ABC)ok").parse().is_err());
  }

}
