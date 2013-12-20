use parsable::Parsable;
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

pub fn parse(t: &str, ps: &mut ParseState) -> ParseCode {
  let mut p = Parsable::new(t);

  _parse_recursive(&mut p, ps)
}

#[inline]
fn parse_escape(p: &mut Parsable, ps: &mut ParseState) -> ParseCode {
  let mut cc = CharClass::new();

  if (!p.isEnd()) {
    let esc = p.current();

    match esc {
      'd' | 'D' => {
        check_ok!(cc.addRange('0', '9'));

        p.next();
      }
      'w' | 'W' => {
        check_ok!(cc.addRange('a', 'z'));
        check_ok!(cc.addRange('A', 'Z'));
        check_ok!(cc.addRange('_', '_'));

        p.next();
      }
      's' | 'S' => {
        check_ok!(cc.addRange('\n', '\n'));
        check_ok!(cc.addRange('\t', '\t'));
        check_ok!(cc.addRange('\r', '\r'));

        p.next();
      }
      _ => return parse_escape_char(p, ps) 
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
fn parse_escape_char(p: &mut Parsable, ps: &mut ParseState) -> ParseCode {
  match p.current() {
    c => {
      ps.pushLiteral(c.to_str());
      p.next();
    }
  }

  ParseOk
}

#[inline]
fn parse_charclass(p: &mut Parsable, ps: &mut ParseState) -> ParseCode {
  let mut cc = CharClass::new();

  // we need to keep track of any [, ( in
  // the input, because we can just ignore 
  // them
  let mut nbracket: uint = 0;

  // check to see if the first char following
  // '[' is a '^', if so, it is a negated char 
  // class 
  let negate = if (!p.isEnd() && p.current() == '^') {
    p.next();
    true
  } else {
    false 
  };

  while (!p.isEnd()) {
    match p.current() {
      '[' => {
        nbracket += 1;
        p.next();
      },
      ']' => {
        p.next();
        if (nbracket > 0) {
          nbracket -= 1;
        } else {
          if (negate) {
            check_ok!(cc.negate());
          }
          if (cc.empty()) {
            return ParseEmptyCharClassRange
          }
          ps.pushCharClass(cc);
          return ParseOk;
        }
      }
      '\\' => {
        p.next();
      }
      c => {
        p.next();
        
        // check to see if its this is part of a 
        // range
        if (p.len() > 1) {
          match p.current() {
            '-' => {
              if (p.peek() != ']') {
                check_ok!(cc.addRange(c, p.peek()));
                p.consume(2);
              } else {
                cc.addRange(c, c);
              }
            }
            _ => {
              cc.addRange(c, c);
            }
          }
        // single character case (no range)
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
fn parse_repetition(p: &mut Parsable, ps: &mut ParseState) -> ParseCode {
  // these help parse numbers with more than
  // 1 digit
  let mut buf = ~"";
  let mut len = 0;

  // check to make sure there are still
  // characters in the string
  macro_rules! check_bounds(
    () => (
      if (len == p.len()) {
        return ParseNotRepetition
      }
    )
  )

  while (len < p.len() && p.peekn(len).is_digit()) {
    buf.push_char(p.peekn(len));
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

  // check for a ',' or a '}'
  // if there is a ',', then there either 
  // is or isn't a bound
  // if there is a '}', there is an
  // exact repetition
  match p.peekn(len) {
    ',' => { 
      len += 1;
    }
    '}' => {
      p.consume(len + 1);
      return ps.doBoundedRepetition(start, start);
    }
    _ => return ParseNotRepetition
  }

  check_bounds!();

  // if the next character is a }, unbounded repetition
  match p.peekn(len) {
    '}' => {
      p.consume(len + 1);
      return ps.doUnboundedRepetition(start);
    }
    x if x.is_digit() => { } // continue if x is a digit
    _ => return ParseNotRepetition
  }

  // this should be the ending digit
  while (len < p.len()  && p.peekn(len).is_digit()) {
    buf.push_char(p.peekn(len));
    len += 1;
  }

  let end = from_str::<uint>(buf).unwrap();

  check_bounds!();

  match p.peekn(len) {
    '}' => {
      p.consume(len + 1);
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
fn _parse_recursive(p: &mut Parsable, ps: &mut ParseState) -> ParseCode {
  // cases for
  // parsing different characters
  // in the input string
  while (!p.isEnd()) {
    match p.current() {
      '(' => {
        ps.doConcatenation();
        ps.pushLeftParen();

        p.next();

        // check for ?: (non capturing group)
        let noncapturing = {
          if (p.len() > 1) {
            p.current() == '?' && p.peek() == ':'
          } else {
            false
          }
        };
        
        // adjust
        if (noncapturing) {
          p.consume(2);
        }

        check_ok!(_parse_recursive(p, ps));
        ps.doLeftParen(noncapturing);
        p.next();
      }
      ')' => {
        if (ps.hasUnmatchedParens() && !p.isEnd()) {
          break;
        }
        return ParseUnexpectedClosingParen;
      }

      '|' => {
        ps.doConcatenation();
        ps.pushAlternation();

        p.next();
        check_ok!(_parse_recursive(p, ps));
        ps.doAlternation();
        
        if (ps.hasUnmatchedParens()) {
          break;
        }
      }

      '*' => {
        p.next();
        ps.doKleine();
      }
      '?' => {
        p.next();
        ps.doZeroOrOne();
      }
      '+' => {
        p.next();
        ps.doOneOrMore();
      }

      '{' => {
        p.next();

        match parse_repetition(p, ps) {
          ParseOk => { },
          ParseNotRepetition => ps.pushLiteral("{"),
          e => return e
        }
      }

      '.' => {
        p.next();
        ps.pushDotAll();
      }

      '^' => {
        p.next();
        ps.pushAssertStart();
      }
      '$' => {
        p.next();
        ps.pushAssertEnd();
      }

      '[' => {
        p.next();
        check_ok!(parse_charclass(p, ps));
      }
      '\\' => {
        p.next();
        check_ok!(parse_escape(p, ps));
      }
      c => {
        p.next();
        ps.pushLiteral(c.to_str());
      }
    }

    //ps.trace();
  }

  ps.doConcatenation();

  if (ps.hasUnmatchedParens() && p.isEnd()) {
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
        let mut ps = ParseState::new(); 
        let ok = match parse($input, &mut ps) {
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
