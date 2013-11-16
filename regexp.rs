use std::ptr;
use parse::*;

mod parse;

struct Regexp {
  input: ~str
}

impl Regexp {
  fn new(s: &str) -> Regexp {
    Regexp { input: s.clone().to_owned() }
  }
  fn parse(&mut self) {
    let mut ps = ParseState::new();
    parse_recursive(&mut self.input, Some(ptr::to_mut_unsafe_ptr(&mut ps))); 
  }
}

pub fn parse_charclass(t: &mut ~str, ps: *mut ParseState) -> Result<(), &'static str> {
  Ok(())
}

pub fn parse_recursive(t: &mut ~str, s: Option<*mut ParseState>) -> Result<(), &'static str> {
  let mut ps = match s {
    Some(st) => { 
      unsafe { ptr::read_and_zero_ptr(st) } 
    },
    None => ParseState::new()
  };

  // cases for
  // parsing different characters
  // in the input string
  while (t.len() > 0) {

    match t.char_at(0) {
      '(' => {
        ps.doConcatenation();
        ps.pushLeftParen();

        t.shift_char();
        parse_recursive(t, Some(ptr::to_mut_unsafe_ptr(&mut ps)));
        ps.doLeftParen();
      },
      ')' => {
        t.shift_char();
        if (ps.hasUnmatchedParens()) {
          break;
        }
        return Err("Unmatched ')'")
      }

      '|' => {
        ps.doConcatenation();
        ps.pushAlternation();

        t.shift_char();
        parse_recursive(t, Some(ptr::to_mut_unsafe_ptr(&mut ps)));
        ps.doAlternation();
        
        if (ps.hasUnmatchedParens()) {
          break;
        }
      },

      '*' => {
        t.shift_char();
        ps.doKleine();
      },
      '?' => {
        t.shift_char();
        ps.doZeroOrOne();
      },
      '+' => {
        t.shift_char();
        ps.doOneOrMore();
      }
      c => {
        ps.pushLiteral(c.to_str());
        t.shift_char();
      }
    }

    ps.trace();

  }

  ps.doConcatenation();

  // replace the content at
  // the old pointer, if a state was passed in
  match s {
    Some(st) => {
      unsafe { ptr::replace_ptr(st, ps); }
    },
    _ => { }
  }

  Ok(())
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
}
