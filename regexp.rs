use exec::Prog;
use parse::parse_recursive;
use state::{ParseState, Regexp};
use compile::{Instruction, compile_recursive};
use error::ParseError::*;

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
      input: s.to_owned().clone()
    }
  }
}

impl CompiledRegexp {
  // the same thing as re.match() in python, 
  // but can't make match a function name in rust
  fn run(&self, input: &str) {
    self.prog.run(input);
  }
  
  // should only find the first
  fn search(&self, input: &str) {
    let len = input.len();

    for start in range(0, len) {
      self.prog.run(input.slice(start, len));
    }
  }

  // not really working how replace should	
  fn replace(&self, input: &str, repstr: &str) {
    self.prog.replace(input, repstr);
  }
  
  // ugly, but functional?
  fn findall(&self, input: &str) {
    let mut start = 0;
    let mut buff = 0;
    let len = input.len();
    while start < len {
      match self.prog.run(input.slice(start, len)) {
        Some(t) => { 
          if t.sp == 0 { 
            start = start + 1;
          } else {
            start = t.sp + buff; 
          }
          buff = start;
        }
        None => { start = start + 1; }
      }
    }
  }

  fn split(&self, input: &str) {

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
    UncompiledRegexp { input: s.clone().to_owned() }
  }
}

impl UncompiledRegexp { 
  // we should hide the underlying parsing algorithm
  // from the user
  fn parse(&mut self) -> Result<Regexp, ParseCode> {
    let mut ps = ParseState::new();
    let mut input = self.input.clone();
    match parse_recursive(&mut input, &mut ps) {
      ParseOk => {
        ps.pop()
      }
      e => {
        Err(e)
      }
    }
  }
  pub fn compile(&mut self) -> Result<CompiledRegexp, ParseCode> {
    let mut stack: ~[Instruction] = ~[];
    match self.parse() {
      Ok(ref re) => {
        compile_recursive(re, &mut stack);
        let prog = Prog::new(stack);
        Ok(CompiledRegexp::new_with_prog(prog, self.input))
      }
      Err(e) => {
        Err(e)
      }
    }
  }
  // for these, just call compile, and
  // run the corresponding CompiledRegex
  // functions
  pub fn run(&mut self, input: &str) {
    match self.compile() {
      Ok(ref mut re) => re.run(input),
      Err(e) => println(e.to_str())
    }
  }
  pub fn search(&mut self, input: &str) {
    match self.compile() {
      Ok(ref mut re) => re.search(input),
      Err(e) => println(e.to_str())
    }
  }
  pub fn replace(&mut self, input: &str, repstr: &str) {
    match self.compile() {
      Ok(ref mut re) => re.replace(input, repstr),
      Err(e) => println(e.to_str())
    }
  }
  pub fn findall(&mut self, input: &str) {
    match self.compile() {
      Ok(ref mut re) => re.findall(input),
      Err(e) => println(e.to_str())
    }
  }
  pub fn split(&mut self, input: &str) {
    match self.compile() {
      Ok(ref mut re) => re.split(input),
      Err(e) => println(e.to_str())
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
