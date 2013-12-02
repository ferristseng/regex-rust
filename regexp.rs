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
  fn search(&self, input: &str) {
    let len = input.len();

    for start in range(0, len) {
      for end in range(start, len) {
        self.prog.run(input.slice(start, end));
      }
    }
  }
  fn replace(&self, input: &str) {

  }
  fn findall(&self, input: &str) {

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
  fn parse(&mut self) -> Result<state::Regexp, ParseCode> {
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
  fn compile(&mut self) -> Result<CompiledRegexp, ParseCode> {
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
  fn run(&mut self, input: &str) {
    match self.compile() {
      Ok(ref mut re) => re.run(input),
      Err(e) => println(e.to_str())
    }
  }
  fn search(&mut self, input: &str) {
    match self.compile() {
      Ok(ref mut re) => re.search(input),
      Err(e) => println(e.to_str())
    }
  }
  fn replace(&mut self, input: &str) {
    match self.compile() {
      Ok(ref mut re) => re.replace(input),
      Err(e) => println(e.to_str())
    }
  }
  fn findall(&mut self, input: &str) {
    match self.compile() {
      Ok(ref mut re) => re.findall(input),
      Err(e) => println(e.to_str())
    }
  }
  fn split(&mut self, input: &str) {
    match self.compile() {
      Ok(ref mut re) => re.split(input),
      Err(e) => println(e.to_str())
    }
  }
}

fn main() {
  println("--Case 0--");
  let mut re = UncompiledRegexp::new("[a-z]d|abc");
  re.run("abc");

  println("--Case 1--");
  let mut re = UncompiledRegexp::new("(?:http(s)?://)?(www.)?([a-zA-Z0-9_.]+).(com|org|net|edu)/?");
  re.run("http://ferristseng.comuASDAFASFASBVZKXJVBKZXBVKJZBXVKBZXV");
  re.run("http://reddit.com/");
  re.run("https://google.com/");
  //re.run("NOT A WEBSITE");
  re.run("http://virginia.edu");
  re.run("www.cnn.com");

  println("--Case 2--");
  let mut re = UncompiledRegexp::new("[^a-zA-Z0-9]*");
  re.run("我是曾繁睿");

  println("--Case 2 (NonGreedy)--");
  let mut re = UncompiledRegexp::new("[^a-zA-Z0-9]*?");
  re.run("我是曾繁睿");

  println("--Case 3--");
  let mut re = UncompiledRegexp::new("(a+?)*");
  re.run("aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa");

  println("--Case 4--");
  let mut re = UncompiledRegexp::new("<([^>]+)>");
  re.run("<html><head></head><div></div></html>");

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

  /*
  println("--Case 7--");
  //Regexp::new("[[A-Z]0-9(fgh)]]]|[abc]").parse();
  Regexp::new("1\\d2").parse();

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
  */

  println("OK");
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
