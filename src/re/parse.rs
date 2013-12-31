use state::State;
use std::char::MAX;
use error::ParseError::*;
use charclass::{Range, new_charclass, new_negated_charclass};

#[deriving(ToStr)]
pub enum QuantifierPrefix {
  Greedy,
  NonGreedy
}

#[deriving(ToStr)]
pub enum Expr {
  Empty,
  Literal(char),
  CharClass(~[Range]),
  Alternation(~Expr, ~Expr),
  Concatenation(~Expr, ~Expr),
  Repetition(~Expr, uint, Option<uint>, QuantifierPrefix),
  Capture(~Expr, uint, Option<~str>),
  AssertWordBoundary,
  AssertNonWordBoundary,
  AssertStart,
  AssertEnd
}

macro_rules! check_ok(
  ($f: expr) => ( 
    match $f {
      Ok(Empty) => continue,
      Ok(re) => re, 
      e => return e
    }
  );
)

/// The public parse function. Builds the initial `State` from the 
/// input string, and calls an underlying parse function.
///
/// # Arguments
///
/// * t - The regular expression string
pub fn parse(t: &str) -> Result<Expr, ParseCode> {
  let mut p = State::new(t);

  _parse_recursive(&mut p)
}

/// Parses an escaped value at a given state.
///
/// # Arguments
///
/// * p - The current state of parsing
#[inline]
fn parse_escape(p: &mut State) -> Result<Expr, ParseCode> {
  let current = p.current();

  // Replace these with static vectors
  let cc = match current {
    Some('d') => new_charclass([('0', '9')]),
    Some('D') => new_negated_charclass([('0', '9')]),
    Some('w') => new_charclass([('a', 'z'), ('A', 'Z'), ('_', '_')]),
    Some('W') => new_negated_charclass([('a', 'z'), ('A', 'Z'), ('_', '_')]),
    Some('s') => new_charclass([('\n', '\n'), ('\t', '\t'), ('\r', '\r')]),
    Some('S') => new_negated_charclass([('\n', '\n'), ('\t', '\t'), ('\r', '\r')]),
    Some('b') => {
      p.next();
      
      return Ok(AssertNonWordBoundary)
    }
    Some('B') => {
      p.next();

      return Ok(AssertWordBoundary)
    }
    Some(_) => return parse_escape_char(p),
    None => return Err(ParseIncompleteEscapeSeq)
  };

  p.next();

  Ok(cc)
}

/// Parses an escaped character at a given state.
///
/// # Arguments
///
/// * p - The current state of parsing
#[inline]
fn parse_escape_char(p: &mut State) -> Result<Expr, ParseCode> {
  match p.current() {
    Some(c) => {
      p.next();

      Ok(Literal(c))
    }
    None => Err(ParseIncompleteEscapeSeq) 
  }
}

/// Parses a capturing group.
///
/// # Arguments
///
/// * p - The current state of parsing
#[inline]
fn parse_group(p: &mut State) -> Result<Expr, ParseCode> {
  let mut capturing = true;
  let mut name: Option<~str> = None;

  // Check for an extension denoted by a ?
  // 
  // Currently supporting:
  //
  // * `?:` = No Capture
  // * `?#` = Comment
  // * `?P<name> = Named Capturing Group
  match p.current() {
    Some('?') => {
      match p.peek() {
        Some(':') => {
          p.consume(2);
          capturing = false;
        }
        // A Comment. Everything in the parenthases is 
        // ignored
        Some('#') => {
          p.consume(2);
          
          loop {
            match p.current() {
              Some(')') => {
                return Ok(Empty)
              }
              Some('\\') if p.len() == 0 => {
                return Err(ParseIncompleteEscapeSeq)
              }
              None => {
                return Err(ParseExpectedClosingParen)
              }
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

  p.nparens += 1;

  let ncap = p.ncaptures;
  
  if (capturing) {
    p.ncaptures += 1;
  }

  let expr = match _parse_recursive(p) {
    Ok(re) => re,
    e => return e
  };

  p.next();

  p.nparens -= 1;

  if (capturing) {
    Ok(Capture(~expr, ncap, name)) 
  } else {
    Ok(expr)
  }
}

/// Parses a character class at a given state.
///
/// # Arguments
///
/// * p - The current state of parsing
#[inline]
fn parse_charclass(p: &mut State) -> Result<Expr, ParseCode> {
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
      p.next();
      ranges.push((']', ']'));
    }
    _ => { }
  }

  loop {
    match p.current() {
      Some('[') => {
        p.next();
        nbracket += 1;
      },
      Some(']') => {
        p.next();
        if (nbracket > 0) {
          nbracket -= 1;
        } else {
          let cc = if (negate) {
            new_negated_charclass(ranges)
          } else {
            new_charclass(ranges)
          };
          // Check to see if the created char class is 
          // empty
          if (match cc {
            CharClass(ref r) => r.len() == 0,
            _ => unreachable!() 
          }) {
            return Err(ParseEmptyCharClassRange)
          }
          return Ok(cc)
        }
      }
      Some('\\') => {
        p.next();
        match p.current() {
          Some(c) => {
            ranges.push((c, c));
          }
          None => return Err(ParseIncompleteEscapeSeq)
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
          Some(_) | None => ranges.push((c, c))
        }
      }
      None => break
    }

  }

  Err(ParseExpectedClosingBracket)
}

/// Determines if there is a repetition operator at a given state and 
/// tries to parse it.
///
/// # Arguments
///
/// * p - The current state of parsing
///
/// # Syntax
///
/// * {a,b} - Bounded repetition
/// * {a} - Bounded repetition
/// * {a,} - Unbounded repetition
#[inline]
fn parse_repetition(p: &mut State) -> Option<(uint, Option<uint>)> {
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
    return None
  }

  // this is guaranteed to be a digit because 
  // we only append it to the buffer if the char
  // is a digit
  let start = from_str::<uint>(buf).unwrap(); 

  buf.clear();

  // Check for a ',' or a '}'
  // if there is a ',', then there either is or isn't a bound
  // if there is a '}', there is an exact repetition
  match p.peekn(len) {
    Some(',') => { 
      len += 1;
    }
    Some('}') => {
      p.consume(len + 1);
      return Some((start, Some(start)))
    }
    _ => return None 
  }

  // If the next character is a '}', unbounded repetition
  match p.peekn(len) {
    Some('}') => {
      p.consume(len + 1);
      return Some((start, None))
    }
    Some(x) if x.is_digit() => (), // Continue if x is a digit
    _ => return None
  }

  // This should be the ending digit
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
      return Some((start, Some(end)));
    }
    _ => return None
  }
}

/// Parses a regular expression recursively.
///
/// # Arguments
///
/// * p - The current state of parsing
fn _parse_recursive(p: &mut State) -> Result<Expr, ParseCode> {
  let mut stack = ~[];

  loop {
    match p.current() {
      Some('(') => {
        p.next();
        do_concat(&mut stack);
        let expr = check_ok!(parse_group(p));
        stack.push(expr);
      } 
      Some(')') => {
        if (p.hasUnmatchedParens()) {
          break;
        }
        return Err(ParseUnexpectedClosingParen);
      }

      Some('|') => {
        do_concat(&mut stack);

        p.next();

        match _parse_recursive(p) {
          Ok(expr) => {
            let alt = stack.pop();
            stack.push(Alternation(~alt, ~expr));
          }
          e => return e
        };

        if (p.hasUnmatchedParens()) {
          break;
        }
      }

      Some(c) if c == '*' || c == '?' || c == '+' => {
        p.next();

        // Look for a quantifier
        let quantifier = match p.current() {
          Some('?') => {
            p.next();
            NonGreedy
          }
          _ => Greedy
        };
        
        match stack.pop_opt() {
          None |
          Some(Repetition(..)) |
          Some(AssertStart) | 
          Some(AssertEnd) | 
          Some(AssertWordBoundary) |
          Some(AssertNonWordBoundary) => {
            return Err(ParseEmptyRepetition)
          }
          Some(expr) => {
            match c {
              '?' => stack.push(Repetition(~expr, 0, Some(1), quantifier)),
              '+' => stack.push(Repetition(~expr, 1, None, quantifier)),
              '*' => stack.push(Repetition(~expr, 0, None, quantifier)),
              _   => unreachable!()
            }
          }
        }
      }

      Some('{') => {
        p.next();
        match parse_repetition(p) {
          Some(rep) => {
            let (start, end) = rep;

            match end {
              Some(e) if (start > e) => {
                return Err(ParseEmptyRepetitionRange)
              }
              _ => ()
            }

            // Look for a quantifier
            let quantifier = match p.current() {
              Some('?') => {
                p.next();
                NonGreedy
              },
              _ => Greedy
            };

            match stack.pop_opt() {
              Some(expr) => {
                stack.push(Repetition(~expr, start, end, quantifier));
              }
              None => {
                return Err(ParseEmptyRepetition)
              }
            }
          }
          None => ()
        }
      }

      Some('.') => {
        p.next();
        stack.push(new_charclass([('\0', MAX)]));
      }

      Some('^') => {
        p.next();
        stack.push(AssertStart);
      }
      Some('$') => {
        p.next();
        stack.push(AssertEnd);
      }

      Some('[') => {
        p.next();
        stack.push(check_ok!(parse_charclass(p)));
      }
      Some('\\') => {
        p.next();
        stack.push(check_ok!(parse_escape(p)));
      }
      Some(c) => {
        p.next();
        stack.push(Literal(c));
      }
      None => break // end of string
    }

    //print_stack(&mut stack);
  }

  do_concat(&mut stack);

  if (p.hasUnmatchedParens() && p.isEnd()) {
    Err(ParseExpectedClosingParen)
  } else {
    match stack.pop_opt() {
      Some(expr)  => Ok(expr),
      None        => Ok(Empty)
    }
  }
}

/// Concatenates all itemes on the stack if there are more 
/// than two.
///
/// # Arguments
///
/// * stack - The stack with items to concatenate
fn do_concat(stack: &mut ~[Expr]) {
  while (stack.len() > 1) {
    let rgt = stack.pop();
    let lft = stack.pop();

    stack.push(Concatenation(~lft, ~rgt));
  }
}

/// Prints the items on the stack.
///
/// # Arguments
///
/// * stack - The stack to print
fn print_stack(stack: &mut ~[Expr]) {
  println("--E-Stack--");
  for e in stack.iter() {
    println(e.to_str());
  }
}

#[cfg(test)]
mod parse_tests {
  use super::*;
  use error::ParseError::*;

  macro_rules! test_parse(
    ($input: expr, $expect: pat) => (
      {
        let ok = match parse($input) {
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
    test_parse!("a{10}", Ok(_));
  }

  #[test]
  fn parse_unbounded_repetition_ok() {
    test_parse!("b{10,}", Ok(_));
  }

  #[test]
  fn parse_bounded_range_repetition_ok() {
    test_parse!("c{10,12}", Ok(_));
  }

  #[test]
  fn parse_bad_range_repetition_ok() {
    test_parse!("c{10,x}", Ok(_));
  }

  #[test]
  fn parse_empty_range_err() {
    test_parse!("d{12,10}", Err(ParseEmptyRepetitionRange));
  }

  #[test]
  fn parse_negative_range_ok() {
    test_parse!("e{-11}", Ok(_));
  }

  #[test]
  fn parse_no_comma_range_ok() {
    test_parse!("f{10 11}", Ok(_));
  }

  #[test]
  fn parse_no_closing_curly_brace_ok() {
    test_parse!("g{10", Ok(_));
  }

  #[test]
  fn parse_no_repetition_target_specified_err() {
    test_parse!("{10,}", Err(ParseEmptyRepetition)); 
  }

  #[test]
  fn parse_backward_slash_err() {
    test_parse!("\\", Err(ParseIncompleteEscapeSeq));
  }
}
