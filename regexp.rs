use parse::parse_recursive;
use state::ParseState;
use compile::{Instruction, compile_recursive};
use error::ParseError::*;

mod parse;
mod state;
mod compile;
mod error;

static PARSE_ERR: &'static str = "Parse Error: ";

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
  fn parse(&mut self) -> Result<state::Regexp, ParseCode> {
    let mut ps = ParseState::new();
    match parse_recursive(&mut self.input, &mut ps) {
      ParseOk => {
        ps.pop()
      }
      e => Err(e)
    }
  }
  fn compile(&mut self) {
    let mut stack: ~[Instruction] = ~[];
    match self.parse() {
      Ok(ref re) => {
        compile_recursive(re, &mut stack);
      }
      Err(e) => {
        println(Regexp::parse_err_to_str(e));
      }
    };
  }
}

impl Regexp {
  fn parse_err_to_str(code: ParseCode) -> ~str {
    match code {
      ParseOk => ~"Ok",
      _ => PARSE_ERR + "Error"
    }
  }
}

fn main() {
  println("--Case 0--");
  Regexp::new("abc").compile();

  println("--Case 1--");
  Regexp::new("a|b").compile();

  println("--Case 2--");
  Regexp::new("a|b|c").compile();

  println("--Case 3--");
  Regexp::new("a|Bcf|dez").compile();

  println("--Case 4--");
  //Regexp::new("abc*|d").parse();

  println("--Case 5--");
  //Regexp::new("io(ab|c)*zz|(bcd)*").parse();

  println("--Case 6--");
  //Regexp::new("あ(ab(cd|d)|e)|f").parse();

  println("--Case 7--");
  //Regexp::new("[[A-Z]0-9(fgh)]]]|[abc]").parse();
}
