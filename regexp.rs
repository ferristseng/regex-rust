extern mod extra;

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
  // from the user
  fn parse(&mut self) -> Result<state::Regexp, ParseCode> {
    let mut ps = ParseState::new();
    match parse_recursive(&mut self.input, &mut ps) {
      ParseOk => {
        ps.pop()
      }
      e => {
        println(Regexp::parse_err_to_str(e));
        Err(e)
      }
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
      ParseOk                     => ~"Ok",
      ParseEmptyAlternate         => PARSE_ERR + "Nothing to alternate",
      ParseEmptyConcatenate       => PARSE_ERR + "Nothing to concatenate",
      ParseRepeatedRepetition     => PARSE_ERR + "Multiple repeat operations",
      ParseEmptyRepetition        => PARSE_ERR + "Nothing to repeat",
      ParseEmptyRepetitionRange   => PARSE_ERR + "Repeat range is empty",
      ParseExpectedClosingParen   => PARSE_ERR + "Expected ')'",
      ParseExpectedClosingBracket => PARSE_ERR + "Expected ']'",
      ParseExpectedClosingBrace   => PARSE_ERR + "Expected '}'",
      ParseExpectedComma          => PARSE_ERR + "Expected ','",
      ParseExpectedAlpha          => PARSE_ERR + "Expected alpha character",
      ParseExpectedNumeric        => PARSE_ERR + "Expected number",
      ParseExpectedOperand        => PARSE_ERR + "Expected an operand on the stack",
      ParseUnexpectedOperand      => PARSE_ERR + "Unexpected operand was on the stack",
      ParseUnexpectedCharacter    => PARSE_ERR + "Unexpected character in input",
      ParseEmptyCharClassRange    => PARSE_ERR + "Empty character class",
      ParseInternalError |
      ParseUnknownError           => PARSE_ERR + "Unknown error (probably a bug)",
      ParseEmptyStack             => PARSE_ERR + "Nothing on the stack"
    }
  }
}

fn main() {
  // println("--Case 0--");
  // Regexp::new("abc").compile();

  // println("--Case 1--");
  // Regexp::new("a|b").compile();

  // println("--Case 2--");
  // Regexp::new("a|b|c").compile();

  // println("--Case 3--");
  // Regexp::new("a|(Bcf)|dez").compile();

  // println("--Case 4--");
  // //Regexp::new("abc*|d").parse();

  // println("--Case 5--");
  // //Regexp::new("io(ab|c)*zz|(bcd)*").parse();

  // println("--Case 6--");
  // //Regexp::new("あ(ab(cd|d)|e)|f").parse();

  println("--Case 7--");
  //Regexp::new("[[A-Z]0-9(fgh)]]]|[abc]").parse();
  Regexp::new("[1-]]").parse();

  println("--Case 8--");
  Regexp::new("(abc){1,}").parse();

  println("--Case 9--");
  Regexp::new("abc{3,4}?").parse();
  
  println("--Case 10--");
  Regexp::new("a|b{3}").parse();

  println("--Case 11--");
  Regexp::new("a{4,3}?").parse();

  println("--Case 12--");
  Regexp::new("[C[e-h]arlemange]|bs|c").compile();

  println("--Case 13--");
  Regexp::new("[^aA-ZA]").parse();

  println("--Case 14--");
  Regexp::new("[^\U00000000-\U0010FFFF]").parse();
}
