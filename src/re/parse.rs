use state::State;
use std::char::MAX;
use std::str;
use error::ParseError::*;
use unicode::*;
use charclass::{Range, new_charclass, new_negated_charclass, AlphaClass,
  NumericClass, WhitespaceClass, NegatedAlphaClass, NegatedNumericClass,
  NegatedWhitespaceClass, ascii};

#[deriving(Show, Clone)]

pub enum QuantifierPrefix {
  Greedy,
  NonGreedy
}

#[deriving(Show, Clone)]

pub enum Expr {
  Empty,
  Literal(char),
  CharClass(~[Range]),
  CharClassStatic(&'static [Range]),
  CharClassTable(&'static [(char,char)]),
  NegatedCharClassTable(&'static [(char,char)]),
  Alternation(~Expr, ~Expr),
  Concatenation(~Expr, ~Expr),
  Repetition(~Expr, uint, Option<uint>, QuantifierPrefix),
  Capture(~Expr, uint, Option<~str>),
  AssertWordBoundary,
  AssertNonWordBoundary,
  AssertStart,
  AssertEnd
}

pub struct ParseFlags {
  i: bool,
  m: bool,
  s: bool,
  U: bool
}

impl ParseFlags {
  pub fn new() -> ParseFlags {
    ParseFlags {
      i: false,
      m: false,
      s: false,
      U: false
    }
  }
}

impl ParseFlags {
  pub fn setFlags(&mut self, s: ~str) {
    for c in s.chars() {
      match c {
        'i' => self.i = true,
        'm' => self.m = true,
        's' => self.s = true,
        'U' => self.U = true,
        _ => ()
      }
    }
  }

  pub fn clearFlags(&mut self, s: ~str) {
    for c in s.chars() {
      match c {
        'i' => self.i = false,
        'm' => self.m = false,
        's' => self.s = false,
        'U' => self.U = false,
        _ => ()
      }
    }
  }
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
/// * f - The current flags for parsing
pub fn parse(t: &str, f: &mut ParseFlags) -> Result<Expr, ParseCode> {
  let mut p = State::new(t);

  _parse_recursive(&mut p, f)
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
    Some('d') => NumericClass.clone(),
    Some('D') => NegatedNumericClass.clone(),
    Some('w') => AlphaClass.clone(),
    Some('W') => NegatedAlphaClass.clone(),
    Some('s') => WhitespaceClass.clone(),
    Some('S') => NegatedWhitespaceClass.clone(),
    Some('p') => {
      p.next();
      return parse_unicode_charclass(p, false);
    }
    Some('P') => {
      p.next();
      return parse_unicode_charclass(p, true);
    }
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

  println!("{:s}", cc.to_str());


  Ok(cc)
}

/// Parses a named Unicode character class
///
/// # Arguments
///
/// * p - The current state of parsing
/// * neg - Whether or not the character class should be negated
#[inline]
fn parse_unicode_charclass(p: &mut State, neg: bool) -> Result<Expr, ParseCode> {
  match p.current() {
    Some('{') => {    // Unicode character class with name longer than 1 character
      let mut prop_name_buf: ~[char] = ~[];
      loop {
        p.next();
        match p.current() {
          Some('}') => {
            p.next();
            if prop_name_buf.len() == 0 {
              return Err(ParseEmptyPropertyName)
            } else {
              let prop_name = str::from_chars(prop_name_buf);
              let prop_table = match general_category::get_prop_table(prop_name) {
                Some(table) => table,
                None => {
                  match script::get_prop_table(prop_name) {
                    Some(table) => table,
                    None => return Err(ParseInvalidUnicodeProperty)
                  }
                }
              };
              if neg {
                return Ok(NegatedCharClassTable(prop_table))
              } else {
                return Ok(CharClassTable(prop_table))
              }
            }
          }
          Some(c) => {
            prop_name_buf.push(c);
          }
          None => return Err(ParseExpectedClosingBrace)
        }
      }
    }
    Some(c) => {      // Unicode character class with 1 character name
      p.next();
      let prop_name = str::from_char(c);
      let prop_table = match general_category::get_prop_table(prop_name) {
        Some(table) => table,
        None => {
          match script::get_prop_table(prop_name) {
            Some(table) => table,
            None => return Err(ParseInvalidUnicodeProperty)
          }
        }
      };
      if neg {
        return Ok(NegatedCharClassTable(prop_table))
      } else {
        return Ok(CharClassTable(prop_table))
      }
    }
    None => return Err(ParseIncompleteEscapeSeq)
  }
}

/// Parses a named ASCII character class
///
/// #Arguments
///
/// * p - The current state of parsing
#[inline]
fn parse_ascii_charclass(p: &mut State) -> Result<Expr, ParseCode> {
  let neg = if p.current() == Some('^') {
    p.next();
    true
  } else {
    false
  };

  let mut prop_name_buf: ~[char] = ~[];
  loop {
    match p.current() {
      Some(':') if p.peek() == Some(']') => {
        p.consume(2);
        return match ascii::get_prop_table(str::from_chars(prop_name_buf)) {
          Some(t) => {
            if neg {
              Ok(NegatedCharClassTable(t))
            } else {
              Ok(CharClassTable(t))
            }
          }
          None => Err(ParseInvalidAsciiCharClass)
        }
      }
      Some(c) => {
        p.next();
        prop_name_buf.push(c);
      }
      None => {
        return Err(ParseExpectedAsciiCharClassClose)
      }
    }
  }
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
/// * f - The current parse flags
#[inline]
fn parse_group(p: &mut State, f: &mut ParseFlags) -> Result<Expr, ParseCode> {
  let mut capturing = true;
  let mut name: Option<~str> = None;
  let mut name_buf: ~[char] = ~[];

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
          match p.current() {
            Some('<') => {
              p.next();
              loop {
                match p.current() {
                  Some('>') if name_buf.len() == 0 => {
                    return Err(ParseEmptyGroupName);
                  }
                  Some('>') => {
                    name = Some(str::from_chars(name_buf));
                    p.next();
                    break;
                  }
                  // TODO: restrict this to [a-zA-Z0-9_]
                  Some(c) => {
                    if c.is_digit_radix(36) || c == '_' {
                      name_buf.push(c);
                      p.next();
                    } else if c == ')' {
                      return Err(ParseExpectedClosingAngleBracket);
                    } else {
                      return Err(ParseExpectedAlphaNumeric);
                    }
                  }
                  None => {
                    return Err(ParseExpectedClosingAngleBracket);
                  }
                }
              }
            }
            _ => {
              return Err(ParseExpectedOpeningAngleBracket);
            }
          }
        }
        _ => ()
      }
    }
    _ => ()
  }

  p.nparens += 1;

  let ncap = p.ncaptures;

  if capturing {
    p.ncaptures += 1;
  }

  let expr = match _parse_recursive(p, f) {
    Ok(re) => re,
    e => return e
  };

  p.next();

  p.nparens -= 1;

  if capturing {
    Ok(Capture(~expr, ncap, name))
  } else {
    Ok(expr)
  }
}

/// Parses a character class at a given state.
///
/// NOTE: Open square brackets within a character class have no
/// special meaning, they are treated as ordinary characters.
/// There is no such thing as nested character classes.
///
/// # Arguments
///
/// * p - The current state of parsing
#[inline]
fn parse_charclass(p: &mut State) -> Result<Expr, ParseCode> {
  let mut ranges = ~[];
  let mut other_exprs = ~[];

  // check to see if this is an ascii char class, not a general purpose one
  match p.current() {
    Some(':') => {
      p.next();
      return parse_ascii_charclass(p);
    }
    _ => { }
  };

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
      Some(']') => {
        p.next();
        let cc = if negate {
          new_negated_charclass(ranges)
        } else {
          new_charclass(ranges)
        };

        // return CharClass and/or any internal special character classes
        let other_exprs_result = new_multiple_alternation(other_exprs);
        let expr_exists = match (&cc, &other_exprs_result) {
          (&CharClass(ref r), &Some(_)) if r.len() > 0 => (true, true),
          (&CharClass(ref r), &None) if r.len() > 0 => (true, false),
          (_, &Some(_)) => (false, true),
          (_, &None) => (false, false)
        };

        return match expr_exists {
          (true, true) => Ok(Alternation(~cc, ~other_exprs_result.unwrap())),
          (true, false) => Ok(cc),
          (false, true) => Ok(other_exprs_result.unwrap()),
          (false, false) => Err(ParseEmptyCharClassRange)
        };
      }
      Some('\\') => {
        p.next();
        match parse_escape(p) {
          Ok(expr) => other_exprs.push(expr),
          err => return err
        }
      }
      Some('[') if p.peek() == Some(':') => {  // ASCII character class
        p.consume(2);
        match parse_ascii_charclass(p) {
          Ok(table) => {
            other_exprs.push(table);
          }
          Err(e) => return Err(e)
        }
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

#[inline]
fn new_multiple_alternation(components: &[Expr]) -> Option<Expr> {
  match components.len() {
    0 => None,
    _ => Some(new_alternation_recursive(components))
  }
}

fn new_alternation_recursive(components: &[Expr]) -> Expr {
  match components.len() {
    1 => components[0].clone(),
    2 => Alternation(~components[0].clone(), ~components[1].clone()),
    n => Alternation(~components[0].clone(), ~new_alternation_recursive(components.slice(1,n)))
  }
}

/// Parses repetitions using the *, +, and ? operators and pushes them on the
/// stack of parsed expressions
///
/// # Arguments
///
/// * p     - The current state of parsing
/// * f     - The current parse flags
/// * stack - The current stack of expressions parsed
/// * c     - The repetition operator being parsed; either *, +, or ?
#[inline]
fn parse_repetition_op(p: &mut State, f: &mut ParseFlags, stack: &mut ~[Expr], c: char) -> Result<Expr, ParseCode> {
  p.next();

  // Look for a quantifier
  let quantifier = match p.current() {
    Some('?') => {
      p.next();
      NonGreedy
    }
    _ => Greedy
  };

  match stack.pop() {
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
        '?' => return Ok(Repetition(~expr, 0, Some(1), quantifier)),
        '+' => return Ok(Repetition(~expr, 1, None, quantifier)),
        '*' => return Ok(Repetition(~expr, 0, None, quantifier)),
        _   => unreachable!()
      }
    }
  }
}

/// Finds the bounds on a bounded or unbounded repetition.
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
fn extract_repetition_bounds(p: &mut State) -> Option<(uint, Option<uint>)> {
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

  if len == 0 {
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

/// Determines if there is a repetition operator at a given state and
/// tries to parse it.
///
/// # Arguments
///
/// * p      - The current state of parsing
/// * stack  - The current stack of parsed expressions
///
/// # Syntax
///
/// * {a,b} - Bounded repetition
/// * {a} - Bounded repetition
/// * {a,} - Unbounded repetition
#[inline]
fn parse_bounded_repetition(p: &mut State, f: &mut ParseFlags, stack: &mut ~[Expr]) -> Result<Expr, ParseCode>{
  p.next();
  match extract_repetition_bounds(p) {
    Some(rep) => {
      let (start, end) = rep;

      match end {
        Some(e) if start > e => {
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

      match stack.pop() {
        Some(expr) => {
          return Ok(Repetition(~expr, start, end, quantifier));
        }
        None => {
          return Err(ParseEmptyRepetition)
        }
      }
    }
    None => return Ok(Empty)
  }
}

/// Parses a regular expression recursively.
///
/// # Arguments
///
/// * p - The current state of parsing
/// * f - The current parse flags
fn _parse_recursive(p: &mut State, f: &mut ParseFlags) -> Result<Expr, ParseCode> {
  let mut stack = ~[];

  loop {
    match p.current() {
      Some('(') => {
        p.next();
        do_concat(&mut stack);
        let expr = check_ok!(parse_group(p, f));
        stack.push(expr);
      }
      Some(')') => {
        if p.hasUnmatchedParens() {
          break;
        }
        return Err(ParseUnexpectedClosingParen);
      }

      Some('|') => {
        do_concat(&mut stack);

        p.next();

        match _parse_recursive(p, f) {
          Ok(expr) => {
            let alt = match stack.pop() {
              Some(ans) => ans,
              None => Empty //Should be unreachable
            };
            stack.push(Alternation(~alt, ~expr));
          }
          e => return e
        };

        if p.hasUnmatchedParens() {
          break;
        }
      }

      Some(c) if c == '*' || c == '?' || c == '+' => {
        match parse_repetition_op(p, f, &mut stack, c) {
          Ok(expr) => stack.push(expr),
          e => return e
        }
      }

      Some('{') => {
        match parse_bounded_repetition(p, f, &mut stack) {
          Ok(Empty) => (),
          Ok(expr) => stack.push(expr),
          e => return e
        }
      }

      Some('.') => {
        p.next();
        if f.s {
          stack.push(CharClass(~[('\0', MAX)]));
        } else {
          stack.push(CharClass(~[('\0', '\x09'), ('\x0B', MAX)]))
        }
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

  if p.hasUnmatchedParens() && p.isEnd() {
    Err(ParseExpectedClosingParen)
  } else {
    match stack.pop() {
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
  while stack.len() > 1 {
    let rgt = match stack.pop() { Some(ans) => ans, None => Empty };
    let lft = match stack.pop() { Some(ans) => ans, None => Empty };

    stack.push(Concatenation(~lft, ~rgt));
  }
}

/// Prints the items on the stack.
///
/// # Arguments
///
/// * stack - The stack to print
fn print_stack(stack: &mut ~[Expr]) {
  println!("--E-Stack--");
  for e in stack.iter() {
    println!("{:s}", e.to_str());
  }
}

#[cfg(test)]
mod parse_tests {
  use super::*;
  use error::ParseError::*;

  macro_rules! test_parse(
    ($input: expr, $expect: pat) => (
      {
        let result = parse($input, &mut ParseFlags::new());
        let ok = match result {
          $expect => true,
          _ => false
        };
        //ps.trace();
        assert!(ok, "Was not expecting {:s}", result.to_str());
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

  #[test]
  fn parse_named_group_ok() {
    test_parse!("(?P<My_nAm3>regex)", Ok(_));
  }

  #[test]
  fn parse_named_group_no_opening_angle_bracket_err() {
    test_parse!("(?Psdf)", Err(ParseExpectedOpeningAngleBracket));
  }

  #[test]
  fn parse_named_group_no_closing_angle_bracket_err() {
    test_parse!("(?P<sdf)", Err(ParseExpectedClosingAngleBracket));
  }

  #[test]
  fn parse_named_group_no_closing_angle_bracket_2_err() {
    test_parse!("(?P<sdf", Err(ParseExpectedClosingAngleBracket));
  }

  #[test]
  fn parse_named_group_empty_name_err() {
    test_parse!("(?P<>sdfkj)", Err(ParseEmptyGroupName));
  }

  #[test]
  fn parse_named_group_invalid_name_character_err() {
    test_parse!("(?P<sdd$f>)", Err(ParseExpectedAlphaNumeric));
  }

  // #[test]
  // fn parse_unicode_charclass_single_letter() {
  //   test_parse!("\\pN", Ok(CharClassTable(~"N")));
  // }

  #[test]
  fn parse_unicode_charclass_multiple_letter() {
    test_parse!("\\p{Greek}", Ok(CharClassTable(_)));
  }

  // #[test]
  // fn parse_unicode_charclass_single_letter_negated() {
  //   test_parse!("\\PL", Ok(NegatedCharClassTable(_)));
  // }

  #[test]
  fn parse_unicode_charclass_multiple_letter_negated() {
    test_parse!("\\P{Latin}", Ok(NegatedCharClassTable(_)));
  }

  #[test]
  fn parse_unicode_charclass_empty_single_letter() {
    test_parse!("\\p", Err(ParseIncompleteEscapeSeq));
  }

  #[test]
  fn parse_unicode_charclass_empty_multiple_letter() {
    test_parse!("\\p{}", Err(ParseEmptyPropertyName));
  }

  #[test]
  fn parse_unicode_charclass_empty_unterminated() {
    test_parse!("\\p{", Err(ParseExpectedClosingBrace));
  }

  #[test]
  fn parse_unicode_charclass_nonempty_unterminated() {
    test_parse!("\\p{Gre", Err(ParseExpectedClosingBrace));
  }

  #[test]
  fn parse_unicode_charclass_multiple_nonexistent() {
    test_parse!("\\pA", Err(ParseInvalidUnicodeProperty));
  }

  #[test]
  fn parse_unicode_charclass_single_nonexistent() {
    test_parse!("\\pA", Err(ParseInvalidUnicodeProperty));
  }

  #[test]
  fn parse_unicode_charclass_nested() {
    test_parse!("[sdkfj\\p{Latin}]", Ok(Alternation(~CharClass(_), ~CharClassTable(_))));
  }

  #[test]
  fn parse_ascii_charclass() {
    test_parse!("[:alpha:]", Ok(CharClassTable(_)));
  }

  #[test]
  fn parse_ascii_charclass_nonexistent() {
    test_parse!("[:alpsd:]", Err(ParseInvalidAsciiCharClass));
  }

  #[test]
  fn parse_ascii_charclass_unterminated() {
    test_parse!("[:alpha", Err(ParseExpectedAsciiCharClassClose));
  }

  #[test]
  fn parse_ascii_charclass_nested() {
    test_parse!("[dsf[:print:]]", Ok(Alternation(~CharClass(_), ~CharClassTable(_))));
  }
}
