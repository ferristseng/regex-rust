use exec::Prog;
use result::Match;
use parse::{parse, Expr};
use compile::{Instruction, compile_recursive};
use error::ParseError::*;

// CompiledRegexp

pub struct CompiledRegexp {
  input: ~str,
  prog: Prog
}

impl CompiledRegexp {
  pub fn new(s: &str) -> Result<CompiledRegexp, ParseCode> {
    match UncompiledRegexp::new(s).compile() {
      Ok(re) => Ok(re), 
      Err(e) => Err(e)
    }
  }
  fn new_with_prog(prog: Prog, s: &str) -> CompiledRegexp {
    CompiledRegexp {
      prog: prog,
      input: s.to_owned()
    }
  }
}

impl CompiledRegexp {
  // the same thing as re.match() in python, 
  // but can't make match a function name in rust
  pub fn exec(&self, input: &str) -> Option<Match> {
    self.prog.run(input, 0)
  }
  
  // should only find the first
  pub fn search(&self, input: &str) -> Option<Match> {
    let len = input.len();

    for start in range(0, len + 1) {
      match self.prog.run(input, start) {
        Some(m) => return Some(m),
        None => { }
      }
    }

    None
  }
}

// uncompiled regular expression
// not parsed until compile is called...
// compile returns a CompiledRegexp
pub struct UncompiledRegexp {
  input: ~str
}

impl UncompiledRegexp {
  pub fn new(s: &str) -> UncompiledRegexp {
    UncompiledRegexp { input: s.to_owned() }
  }
}

impl UncompiledRegexp { 
  fn parse(&mut self) -> Result<Expr, ParseCode> {
    parse(self.input)
  }
  pub fn compile(&mut self) -> Result<CompiledRegexp, ParseCode> {
    let mut stack: ~[Instruction] = ~[];
    match self.parse() {
      Ok(ref expr) => {
        compile_recursive(expr, &mut stack);
        let prog = Prog::new(stack, 0);
        Ok(CompiledRegexp::new_with_prog(prog, self.input))
      }
      Err(e) => Err(e)
    }
  }
  pub fn exec(&mut self, input: &str) -> Result<Option<Match>, ParseCode> {
    match self.compile() {
      Ok(ref mut re) => Ok(re.exec(input)),
      Err(e) => Err(e)
    }
  }
  pub fn search(&mut self, input: &str) -> Result<Option<Match>, ParseCode> {
    match self.compile() {
      Ok(ref mut re) => Ok(re.search(input)),
      Err(e) => Err(e)
    }
  }
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
