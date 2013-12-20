use state::ParseState;
use charclass::CharClass;
use error::ParseError::*;

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

#[inline]
fn parse_escape(t: &mut ~str, ps: &mut ParseState) -> ParseCode {

  let mut cc = CharClass::new();

  if (ps.remainder() > 0) {
    let esc = t.char_at(ps.ptr());

    match esc {
      'd' | 'D' => {
        check_ok!(cc.addRange('0', '9'));

        ps.incr(esc.len_utf8_bytes());
      }
      'w' | 'W' => {
        check_ok!(cc.addRange('a', 'z'));
        check_ok!(cc.addRange('A', 'Z'));
        check_ok!(cc.addRange('_', '_'));

        ps.incr(esc.len_utf8_bytes());
      }
      's' | 'S' => {
        check_ok!(cc.addRange('\n', '\n'));
        check_ok!(cc.addRange('\t', '\t'));
        check_ok!(cc.addRange('\r', '\r'));

        ps.incr(esc.len_utf8_bytes());
      }
      _ => return parse_escape_char(t, ps) 
    }

    if (esc.is_uppercase()) {
      cc.negate();
    }

    ps.pushCharClass(cc);

    ParseOk
  } else {
    ParseIncompleteEscapeSeq
  }
}

#[inline]
fn parse_escape_char(t: &mut ~str, ps: &mut ParseState) -> ParseCode {

  match t.char_at(ps.ptr()) {
    c => {
      ps.pushLiteral(c.to_str());
      ps.incr(c.len_utf8_bytes());
    }
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
  let negate = match t.char_at(ps.ptr()) {
    '^' => {
      ps.incr(1);
      true
    },
    _ => false
  };

  while (ps.remainder() > 0) {

    match t.char_at(ps.ptr()) {
      '[' => {
        nbracket += 1;
        ps.incr(1);
      },
      ']' => {
        ps.incr(1);
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
        ps.incr(c.len_utf8_bytes());
        
        // check to see if its this is part of a 
        // range
        if (ps.remainder() > 1) {
          match t.char_at(ps.ptr()) {
            '-' => {
              if (t.char_at(ps.ptr() + 1) != ']') {
                match cc.addRange(c, t.char_at(ps.ptr() + 1)) {
                  ParseOk => { },
                  e => return e,
                }
                ps.incr(c.len_utf8_bytes() + 1);
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
      if (len == ps.remainder()) {
        return ParseNotRepetition
      }
    )
  )

  while (len < ps.remainder() && t.char_at(ps.ptr() + len).is_digit()) {
    buf.push_char(t.char_at(ps.ptr() + len));
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

  match t.char_at(ps.ptr() + len) {
    ',' => { 
      len += 1;
    }
    '}' => {
      ps.incr(len + 1);
      return ps.doBoundedRepetition(start, start);
    }
    _ => return ParseNotRepetition
  }

  check_bounds!();

  // if the next character is a }, unbounded repetition
  match t.char_at(ps.ptr() + len) {
    '}' => {
      ps.incr(len + 1);
      return ps.doUnboundedRepetition(start);
    }
    x if x.is_digit() => { } // continue if x is a digit
    _ => return ParseNotRepetition
  }

  // this should be the ending digit
  while (len < ps.remainder() && t.char_at(ps.ptr() + len).is_digit()) {
    buf.push_char(t.char_at(ps.ptr() + len));
    len += 1;
  }

  let end = from_str::<uint>(buf).unwrap();

  check_bounds!();

  match t.char_at(ps.ptr() + len) {
    '}' => {
      ps.incr(len + 1);
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
  while (ps.remainder() > 0) {

    match t.char_at(ps.ptr()) {
      '(' => {
        ps.doConcatenation();
        ps.pushLeftParen();

        ps.incr(1);

        // check for ?: (non capturing group)
        let noncapturing = {
          if (ps.remainder() > 1) {
            t.char_at(ps.ptr()) == '?' && t.char_at(ps.ptr() + 1) == ':'
          } else {
            false
          }
        };
        
        // adjust
        if (noncapturing) {
          ps.incr(2);
        }

        check_ok!(parse_recursive(t, ps));
        ps.doLeftParen(noncapturing);
        ps.incr(1);
      }
      ')' => {
        if (ps.hasUnmatchedParens() && ps.remainder() > 0) {
          break;
        }
        return ParseUnexpectedClosingParen;
      }

      '|' => {
        ps.doConcatenation();
        ps.pushAlternation();

        ps.incr(1);
        check_ok!(parse_recursive(t, ps));
        ps.doAlternation();
        
        if (ps.hasUnmatchedParens()) {
          break;
        }
      }

      '*' => {
        ps.incr(1);
        ps.doKleine();
      }
      '?' => {
        ps.incr(1);
        ps.doZeroOrOne();
      }
      '+' => {
        ps.incr(1);
        ps.doOneOrMore();
      }

      '{' => {
        ps.incr(1);

        match parse_repetition(t, ps) {
          ParseOk => { },
          ParseNotRepetition => ps.pushLiteral("{"),
          e => return e
        }
      }

      '.' => {
        ps.incr(1);
        ps.pushDotAll();
      }

      '^' => {
        ps.incr(1);
        ps.pushLineStart();
      }
      '$' => {
        ps.incr(1);
        ps.pushLineEnd();
      }

      '[' => {
        ps.incr(1);
        check_ok!(parse_charclass(t, ps));
      }
      '\\' => {
        ps.incr(1);
        check_ok!(parse_escape(t, ps));
      }
      c => {
        ps.pushLiteral(c.to_str());
        ps.incr(c.len_utf8_bytes());
      }
    }

    //ps.trace();

  }

  ps.doConcatenation();

  if (ps.hasUnmatchedParens() && ps.remainder() == 0) {
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

  macro_rules! test_parse(
    ($input: expr, $expect: pat) => (
      {
        let mut ps = ParseState::new($input); 
        let ok = match parse_recursive(&mut ~$input, &mut ps) {
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
    test_parse!("a{10}", ParseOk);
  }

  #[test]
  fn parse_unbounded_repetition_ok() {
    test_parse!("b{10,}", ParseOk);
  }

  #[test]
  fn parse_bounded_range_repetition_ok() {
    test_parse!("c{10,12}", ParseOk);
  }

  #[test]
  fn parse_bad_range_repetition_ok() {
    test_parse!("c{10,x}", ParseOk);
  }

  #[test]
  fn parse_empty_range_err() {
    test_parse!("d{12,10}", ParseEmptyRepetitionRange);
  }

  #[test]
  fn parse_negative_range_ok() {
    test_parse!("e{-11}", ParseOk);
  }

  #[test]
  fn parse_no_comma_range_ok() {
    test_parse!("f{10 11}", ParseOk);
  }

  #[test]
  fn parse_no_closing_curly_brace_ok() {
    test_parse!("g{10", ParseOk);
  }

  #[test]
  fn parse_no_repetition_target_specified_err() {
    test_parse!("{10,}", ParseEmptyRepetition); 
  }

  #[test]
  fn parse_backward_slash_err() {
    test_parse!("\\", ParseIncompleteEscapeSeq);
  }
}
