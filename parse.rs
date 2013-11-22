use std::ptr;
use state::ParseState;
use state::CharClass;
use error::ParseError::*;

// parse functions
//
// these take in a pointer to a ParseState and an input string,
// and finish / modify the ParseState

pub fn parse_charclass(t: &mut ~str, ps: &mut ParseState) -> ParseCode {
 
  // check to see if the first char following
  // '[' is a '^', if so, it is a negated char 
  // class
  let mut cc = CharClass::new();

  // we need to keep track of any [, ( in
  // the input, because we can just ignore 
  // them
  let mut nbracket: uint = 0;

  match t.char_at(0) {
    '^' => {
      t.shift_char();
      cc.negate();
    },
    _ => { }
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
                t.shift_char();
                t.shift_char();
              }
            }
            _ => { 
              cc.addChar(c);
            }
          }
        } else {
          cc.addChar(c); 
        }
      }
    }

  }

  ParseExpectedClosingBracket
}

// parse an input string recursively
// ideally, we wouldn't parse an input string recursively
// because rust does not optimize tail end
// recursive calls, but...
// this way is pretty
pub fn parse_recursive(t: &mut ~str, ps: &mut ParseState) -> ParseCode {
  
  // check for an err,
  macro_rules! check_ok(
    ($f: expr) => (
      match $f {
        ParseOk => { }
        e => return e
      }
    );
  )

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
      },
      ')' => {
        t.shift_char();
        if (ps.hasUnmatchedParens()) {
          break;
        }
        return ParseExpectedClosingParen;
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

      '[' => {
        t.shift_char();
        check_ok!(parse_charclass(t, ps));
      }

      c => {
        ps.pushLiteral(c.to_str());
        t.shift_char();
      }
    }

    ps.trace();

  }

  ps.doConcatenation();

  ParseOk
}

