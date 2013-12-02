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

// check for an err
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
fn parse_escape(t: &mut ~str, ps: &mut ParseState) -> ParseCode {
  let mut cc = CharClass::new();
  match t.char_at(0) {
    'd' => {
      match cc.addRange('0', '9') {
        ParseOk => { 
          ps.pushCharClass(cc); 
          t.shift_char();
        }
        e => return e
      }
    } 
    _ => { }
  }
  ParseOk
}

#[inline]
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

#[inline]
fn parse_repetition(t: &mut ~str, ps: &mut ParseState) -> ParseCode {

  let mut buf = ~"";
  let mut len = 0;

  // check to make sure there are still
  // characters in the string
  macro_rules! check_bounds(
    () => (
      if (len == t.len()) {
        return ParseNotRepetition
      }
    )
  )

  while (len < t.len() && t.char_at(len).is_digit()) {
    buf.push_char(t.char_at(len));
    len += 1;
  }

  if (len == 0) {
    return ParseNotRepetition // no digits or end of string
  }

  // this is guaranteed to be a digit because 
  // we only append it to the buffer if the char
  // is a digit
  let start = from_str::<uint>(buf).unwrap(); 

  buf.clear();

  check_bounds!();

  match t.char_at(len) {
    ',' => { 
      len += 1;
    }
    '}' => {
      t.shiftn_char(len + 1);
      return ps.doBoundedRepetition(start, start);
    }
    _ => return ParseNotRepetition
  }

  check_bounds!();

  // if the next character is a }, unbounded repetition
  match t.char_at(len) {
    '}' => {
      t.shiftn_char(len + 1);
      return ps.doUnboundedRepetition(start);
    }
    x if x.is_digit() => { } // continue if x is a digit
    _ => return ParseNotRepetition
  }

  // this should be the ending digit
  while (len < t.len() && t.char_at(len).is_digit()) {
    buf.push_char(t.char_at(len));
    len += 1;
  }

  let end = from_str::<uint>(buf).unwrap();

  check_bounds!();

  match t.char_at(len) {
    '}' => {
      t.shiftn_char(len + 1);
      return ps.doBoundedRepetition(start, end);
    }
    _ => return ParseNotRepetition
  }
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

        // check for ?: (non capturing group)
        let noncapturing = {
          if (t.len() > 1) {
            t.char_at(0) == '?' && t.char_at(1) == ':'
          } else {
            false
          }
        };
        
        // adjust
        if (noncapturing) {
          t.shiftn_char(2);
        }

        check_ok!(parse_recursive(t, ps));
        ps.doLeftParen(noncapturing);
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

        match parse_repetition(t, ps) {
          ParseOk => { },
          ParseNotRepetition => ps.pushLiteral("{"),
          e => return e
        }
      }

      '[' => {
        t.shift_char();
        check_ok!(parse_charclass(t, ps));
      }
      '\\' => {
        t.shift_char();
        check_ok!(parse_escape(t, ps));
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

#[cfg(test)]
mod parse_tests {
  use super::*;
  use state::*;
  use error::ParseError::*;

  macro_rules! test_repetition(
    ($input: expr, $expect: pat) => (
      {
        let mut ps = ParseState::new(); 
        let ok = match parse_recursive(&mut $input, &mut ps) {
          $expect => true,
          _ => false
        };
        //ps.trace();
        assert!(ok); 
      }
    );
  )

  #[test]
  fn parse_bounded_repetition_ok() {
    test_repetition!(~"a{10}", ParseOk);
  }

  #[test]
  fn parse_unbounded_repetition_ok() {
    test_repetition!(~"b{10,}", ParseOk);
  }

  #[test]
  fn parse_bounded_range_repetition_ok() {
    test_repetition!(~"c{10,12}", ParseOk);
  }

  #[test]
  fn parse_bad_range_repetition_ok() {
    test_repetition!(~"c{10,x}", ParseOk);
  }

  #[test]
  fn parse_empty_range_err() {
    test_repetition!(~"d{12,10}", ParseEmptyRepetitionRange);
  }

  #[test]
  fn parse_negative_range_ok() {
    test_repetition!(~"e{-11}", ParseOk);
  }

  #[test]
  fn parse_no_comma_range_ok() {
    test_repetition!(~"f{10 11}", ParseOk);
  }

  #[test]
  fn parse_no_closing_curly_brace_ok() {
    test_repetition!(~"g{10", ParseOk);
  }

  #[test]
  fn parse_no_repetition_target_specified_err() {
    test_repetition!(~"{10,}", ParseEmptyRepetition); 
  }
}
