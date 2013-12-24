use parsable::Parsable;
use state::ParseState;
use charclass::CharClass;
use error::ParseError::*;

// check for an err
macro_rules! check_ok(
  ($f: expr) => (
    match $f {
      ParseOk => (), 
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
  let current = p.current();

  let cc = match current {
    Some('d') => CharClass::new([('0', '9')]),
    Some('D') => CharClass::new_negated([('0', '9')]),
    Some('w') => CharClass::new([('a', 'z'), ('A', 'Z'), ('_', '_')]),
    Some('W') => CharClass::new_negated([('a', 'z'), ('A', 'Z'), ('_', '_')]),
    Some('s') => CharClass::new([('\n', '\n'), ('\t', '\t'), ('\r', '\r')]),
    Some('S') => CharClass::new_negated([('\n', '\n'), ('\t', '\t'), ('\r', '\r')]),
    Some('b') => {
      ps.pushNonWordBoundary();
      p.next();

      return ParseOk
    }
    Some('B') => {
      ps.pushWordBoundary();
      p.next();

      return ParseOk
    }
    Some(_) => return parse_escape_char(p, ps),
    None => return ParseIncompleteEscapeSeq
  };

  p.next();

  ps.pushCharClass(cc);

  ParseOk
}

#[inline]
fn parse_escape_char(p: &mut Parsable, ps: &mut ParseState) -> ParseCode {
  match p.current() {
    Some(c) => {
      ps.pushLiteral(c.to_str());
      p.next();
      ParseOk
    }
    None => ParseIncompleteEscapeSeq 
  }
}

#[inline]
fn parse_group(p: &mut Parsable, ps: &mut ParseState) -> ParseCode {
  let mut noncapturing = false;
  let mut name: Option<~str> = None;

  ps.doConcatenation();
  ps.pushLeftParen();

  // Check for an extension denoted by a ?
  // 
  // Currently supporting:
  //
  //  * `?:` = No Capture
  //  * `?#` = Comment
  //  * `?P<name> = Named Capturing Group
  match p.current() {
    Some('?') => {
      match p.peek() {
        Some(':') => {
          p.consume(2);

          noncapturing = true;
        }
        // A Comment. Everything in the parenthases is 
        // ignored
        Some('#') => {
          p.consume(2);
          
          loop {
            match p.current() {
              Some(')') => return ParseOk,
              Some('\\') if p.len() == 0 => break,  
              None => break,
              _ => p.next()
            }
          }
        }
        Some('P') => {
          

          p.consume(2);
        }
        _ => () 
      }
    }
    _ => ()
  }

  check_ok!(_parse_recursive(p, ps));
  ps.doLeftParen(noncapturing);
  p.next();

  ParseOk
}

#[inline]
fn parse_charclass(p: &mut Parsable, ps: &mut ParseState) -> ParseCode {
  // we need to keep track of any [, ( in
  // the input, because we can just ignore 
  // them
  let mut nbracket: uint = 0;
  let mut ranges = ~[];

  // check to see if the first char following
  // '[' is a '^', if so, it is a negated char 
  // class 
  let negate = match p.current() {
    Some('^') => {
      p.next();
      true
    }
    _ => false
  };

  // if the first character in a char class is ], 
  // it is treated as a literal
  match p.current() {
    Some(']') => {
      ranges.push((']', ']'));
      p.next();
    }
    _ => { }
  }

  loop {
    match p.current() {
      Some('[') => {
        nbracket += 1;
        p.next();
      },
      Some(']') => {
        p.next();
        if (nbracket > 0) {
          nbracket -= 1;
        } else {
          let cc = if (negate) {
            CharClass::new_negated(ranges)
          } else {
            CharClass::new(ranges)
          };
          if (cc.empty()) {
            return ParseEmptyCharClassRange
          }
          ps.pushCharClass(cc);
          return ParseOk;
        }
      }
      Some('\\') => {
        p.next();
        match p.current() {
          Some(c) => {
            ranges.push((c, c));
          }
          None => return ParseIncompleteEscapeSeq
        }
        p.next();
      }
      Some(c) => {
        p.next();
        
        // check to see if its this is part of a 
        // range
        match p.current() {
          Some('-') => {
            match p.peek() {
              // Not a range...something like [a-]
              Some(']') => {
                ranges.push((c, c));
              }
              // A range...something like [a-b]
              Some(e) => {
                ranges.push((c, e));
                p.consume(2);
              }
              // End of string
              None => break
            }
          }
          // A single character...something like [a]
          Some(_) | None => {
            ranges.push((c, c));
          }
        }
      }
      None => break
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

  loop {
    match p.peekn(len) {
      Some(d) if d.is_digit() => {
        buf.push_char(d);
        len += 1;
      }
      _ => break
    }
  }

  if (len == 0) {
    return ParseNotRepetition // no digits or end of string
  }

  // this is guaranteed to be a digit because 
  // we only append it to the buffer if the char
  // is a digit
  let start = from_str::<uint>(buf).unwrap(); 

  buf.clear();

  // check for a ',' or a '}'
  // if there is a ',', then there either 
  // is or isn't a bound
  // if there is a '}', there is an
  // exact repetition
  match p.peekn(len) {
    Some(',') => { 
      len += 1;
    }
    Some('}') => {
      p.consume(len + 1);
      return ps.doBoundedRepetition(start, start);
    }
    _ => return ParseNotRepetition
  }

  // if the next character is a }, unbounded repetition
  match p.peekn(len) {
    Some('}') => {
      p.consume(len + 1);
      return ps.doUnboundedRepetition(start);
    }
    Some(x) if x.is_digit() => { } // continue if x is a digit
    _ => return ParseNotRepetition
  }

  // this should be the ending digit
  loop {
    match p.peekn(len) {
      Some(d) if d.is_digit() => {
        buf.push_char(d);
        len += 1;
      }
      _ => break
    }
  }

  let end = from_str::<uint>(buf).unwrap();

  match p.peekn(len) {
    Some('}') => {
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
  loop {
    match p.current() {
      Some('(') => {
        p.next();
        check_ok!(parse_group(p, ps));
      } 
      Some(')') => {
        if (ps.hasUnmatchedParens() && !p.isEnd()) {
          break;
        }
        return ParseUnexpectedClosingParen;
      }

      Some('|') => {
        ps.doConcatenation();
        ps.pushAlternation();

        p.next();
        check_ok!(_parse_recursive(p, ps));
        ps.doAlternation();
        
        if (ps.hasUnmatchedParens()) {
          break;
        }
      }

      Some('*') => {
        p.next();
        check_ok!(ps.doKleine());
      }
      Some('?') => {
        p.next();
        check_ok!(ps.doZeroOrOne());
      }
      Some('+') => {
        p.next();
        check_ok!(ps.doOneOrMore());
      }

      Some('{') => {
        p.next();

        match parse_repetition(p, ps) {
          ParseOk => { },
          ParseNotRepetition => ps.pushLiteral(~"{"),
          e => return e
        }
      }

      Some('.') => {
        p.next();
        ps.pushDotAll();
      }

      Some('^') => {
        p.next();
        ps.pushAssertStart();
      }
      Some('$') => {
        p.next();
        ps.pushAssertEnd();
      }

      Some('[') => {
        p.next();
        check_ok!(parse_charclass(p, ps));
      }
      Some('\\') => {
        p.next();
        check_ok!(parse_escape(p, ps));
      }
      Some(c) => {
        p.next();
        ps.pushLiteral(c.to_str());
      }
      None => break // end of string
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
