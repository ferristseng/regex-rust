use std::ptr;
use parse::parse_recursive;
use state::ParseState;
use compile::{Instruction, compile_recursive};

mod parse;
mod state;
mod compile;

struct Regexp {
  input: ~str
}

impl Regexp {
  pub fn new(s: &str) -> Regexp {
    Regexp { input: s.clone().to_owned() }
  }
}

impl Regexp { 
  // we should hide the underlying parsing algorithm
  fn parse(&mut self) -> Result<state::Regexp, &'static str> {
    let mut ps = ParseState::new();
    match parse_recursive(&mut self.input, ptr::to_mut_unsafe_ptr(&mut ps)) {
      Ok(s) => {
        ps = s;
        ps.pop()
      }
      Err(e) => Err(e)
    }
  }
  fn compile(&mut self) {
    let mut stack: ~[Instruction] = ~[];
    match self.parse() {
      Ok(ref re) => {
        compile_recursive(re, &mut stack);
      }
      Err(e) => {
        println(e);
      }
    };
  }
}

fn main() {
  println("--Case 0--");
  Regexp::new("abc").compile();

  println("--Case 1--");
  Regexp::new("a|b").compile();

  println("--Case 2--");
  //Regexp::new("a|b|c").parse();

  println("--Case 3--");
  //Regexp::new("a|Bcf|dez").parse();

  println("--Case 4--");
  //Regexp::new("abc*|d").parse();

  println("--Case 5--");
  //Regexp::new("io(ab|c)*zz|(bcd)*").parse();

  println("--Case 6--");
  //Regexp::new("あ(ab(cd|d)|e)|f").parse();

  println("--Case 7--");
  //Regexp::new("[[A-Z]0-9(fgh)]]]|[abc]").parse();
}
