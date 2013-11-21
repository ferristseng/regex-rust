use std::ptr;
use parse::parse_recursive;
use state::ParseState;

mod parse;
mod state;

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
  fn parse(&mut self) {
    let mut ps = ParseState::new();
    parse_recursive(&mut self.input, ptr::to_mut_unsafe_ptr(&mut ps)); 
  }
}

fn main() {
  println("--Case 1--");
  Regexp::new("a|b").parse();

  println("--Case 2--");
  Regexp::new("a|b|c").parse();

  println("--Case 3--");
  Regexp::new("a|Bcf|dez").parse();

  println("--Case 4--");
  Regexp::new("abc*|d").parse();

  println("--Case 5--");
  Regexp::new("io(ab|c)*zz|(bcd)*").parse();

  println("--Case 6--");
  Regexp::new("あ(ab(cd|d)|e)|f").parse();

  println("--Case 7--");
  Regexp::new("[[A-Z]0-9(fgh)]]]|[abc]").parse();
}
