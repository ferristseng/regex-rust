use std::char::to_digit;
use state::ParseState;
use state::CharClass;
use error::ParseError::*;

// add functionality to
// build in ~str to shift a number
// of characters from the beginning

trait RegexInputStr {
  fn shiftn_char(&mut self, times: uint);
}

impl RegexInputStr for ~str {
  fn shiftn_char(&mut self, times: uint) {
    for _ in range(0, times) {
      self.shift_char();
    }
  }
}

// check for an err,
macro_rules! check_ok(
  ($f: expr) => (
    match $f {
      ParseOk => { }
      e => return e
    }
  );
)

// parse functions
//
// these take in a pointer to a ParseState and an input string,
// and finish / modify the ParseState

fn parse_charclass(t: &mut ~str, ps: &mut ParseState) -> ParseCode {
 
  let mut cc = CharClass::new();

  // we need to keep track of any [, ( in
  // the input, because we can just ignore 
  // them
  let mut nbracket: uint = 0;

  // check to see if the first char following
  // '[' is a '^', if so, it is a negated char 
  // class
  let negate = match t.char_at(0) {
    '^' => {
      t.shift_char();
      true
    },
    _ => false
  };

  while (t.len() > 0) {

    match t.char_at(0) {
      '[' => {
        nbracket += 1;
        t.shift_char();
      },
      ']' => {
        t.shift_char();
        if (nbracket > 0) {
          nbracket -= 1;
        } else {
          if (negate) {
            check_ok!(cc.negate());
          }
          ps.pushCharClass(cc);
          return ParseOk;
        }
      }
      c => {
        t.shift_char();
        
        // check to see if its this is part of a 
        // range
        if (t.len() > 1) {
          match t.char_at(0) {
            '-' => {
              if (t.char_at(1) != ']') {
                match cc.addRange(c, t.char_at(1)) {
                  ParseOk => { },
                  e => return e,
                }
                t.shiftn_char(2);
              } else {
                cc.addRange(c, c);
              }
            }
            _ => { 
              cc.addRange(c, c);
            }
          }
        } else {
          cc.addRange(c, c); 
        }
      }
    }

  }

  ParseExpectedClosingBracket
}

// tries to determine if there
// is a repetition operation
// and parses it
// multiple cases:
// {a,b}: from a to be inclusive
// {a,}:  a unbounded
// {a}:   exactly a
fn parse_repetition(t: &mut ~str, ps: &mut ParseState) -> ParseCode {

  let buf = ~"";

  if (t.len() > 1) {
    let s = match t.char_at(0) {
      x if x > '0' && x < '9' => to_digit(x, 10).unwrap(), 
      _ => return ParseOk // not parsable
    };
    match t.char_at(1) {
      ',' => { } // continue
      '}' => {
        t.shiftn_char(2); 
        return ps.doBoundedRepetition(s, s);
      }
      _ => return ParseOk // not parsable
    }

    if (t.len() > 2) {
      let e = match t.char_at(2) {
        '}' => {
          t.shiftn_char(3);
          return ps.doUnboundedRepetition(s);
        }
        x if x > '0' && x < '9' => to_digit(x, 10).unwrap(), 
        _ => return ParseOk // not parsable
      };
      if (t.len() > 3) {
        match t.char_at(3) {
          '}' => {
            t.shiftn_char(4);
            return ps.doBoundedRepetition(s, e);
          },
          _ => return ParseOk // not parsable
        } 
      }
    }
  }

  ParseOk
}

// parse an input string recursively
// ideally, we wouldn't parse an input string recursively
// because rust does not optimize tail end
// recursive calls, but...
// this way is pretty
pub fn parse_recursive(t: &mut ~str, ps: &mut ParseState) -> ParseCode {
  
  // cases for
  // parsing different characters
  // in the input string
  while (t.len() > 0) {

    match t.char_at(0) {
      '(' => {
        ps.doConcatenation();
        ps.pushLeftParen();

        t.shift_char();
        check_ok!(parse_recursive(t, ps));
        ps.doLeftParen();
        t.shift_char();
      }
      ')' => {
        if (ps.hasUnmatchedParens() && t.len() > 0) {
          break;
        }
        return ParseUnexpectedClosingParen;
      }

      '|' => {
        ps.doConcatenation();
        ps.pushAlternation();

        t.shift_char();
        check_ok!(parse_recursive(t, ps));
        ps.doAlternation();
        
        if (ps.hasUnmatchedParens()) {
          break;
        }
      }

      '*' => {
        t.shift_char();
        ps.doKleine();
      }
      '?' => {
        t.shift_char();
        ps.doZeroOrOne();
      }
      '+' => {
        t.shift_char();
        ps.doOneOrMore();
      }

      '{' => {
        t.shift_char();
        check_ok!(parse_repetition(t, ps));
      }

      '[' => {
        t.shift_char();
        check_ok!(parse_charclass(t, ps));
      }

      c => {
        ps.pushLiteral(c.to_str());
        t.shift_char();
      }
    }

    //ps.trace();

  }

  ps.doConcatenation();

  if (ps.hasUnmatchedParens() && t.len() == 0) {
    ParseExpectedClosingParen
  } else {
    ParseOk
  }
}
