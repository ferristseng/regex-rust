use parse::{Expr, CharClass, CharClassStatic};
use std::char::{from_u32, MAX};
use std::cmp::{Less, Greater};

pub type Range = (char, char);

/// Static Character Classes
pub static NumericClass: Expr = CharClassStatic([
  ('0', '9')
]);
pub static AlphaClass: Expr = CharClassStatic([
  ('a', 'z'), 
  ('A', 'Z'), 
  ('_', '_')
]);
pub static WhitespaceClass: Expr = CharClassStatic([
  (' ', ' '), 
  ('\t', '\t'), 
  ('\u000b', '\u000b'), 
  ('\u000c', '\u000c'), 
  ('\n', '\n'), 
  ('\r', '\r')
]);
pub static NegatedNumericClass: Expr = CharClassStatic([
  ('\u0000', '\u002F'), ('\u003A', MAX)
]);
pub static NegatedAlphaClass: Expr = CharClassStatic([
  ('\u0000', '\u0040'), 
  ('\u005B', '\u005E'), 
  ('\u0060', '\u0060'), 
  ('\u007B', MAX)
]);
pub static NegatedWhitespaceClass: Expr = CharClassStatic([
  ('\u0000', '\u0008'), 
  ('\u000e', '\u001f'), 
  ('\u0021', MAX)
]);

/// Try to get the prev character in sequence from 
/// the given one.
fn prev_char(c: char) -> Option<char> {
  match from_u32(c as u32 - 1) {
    None => None,
    r    => r   
  }
}

/// Try to get the next character in sequence from 
/// the given one.
fn next_char(c: char) -> Option<char> {
  match from_u32(c as u32 + 1) {
    None => None,
    r    => r
  }
}

/// Order character ranges.
///
/// Character ranges with a greater end are preferred when 
/// equal.
fn order_ranges(ranges: &mut ~[Range]) {
  ranges.sort_by(|range1, range2| {
    let &(start1, end1) = range1;
    let &(start2, end2) = range2;

    if (start1 < start2) {
      Less 
    } else if (start1 == start2) {
      if (end1 > end2) {
        Less
      } else {
        Greater
      }
    } else {
      Greater
    }
  })
}

/// Construct a CharClass with a set of ranges. Remove 
/// overlapping ranges preferring larger ranges (ex. Given [A-DA-C], 
/// collapse to [A-D]).
pub fn new_charclass(ranges: ~[Range]) -> Expr {
  let mut ranges = ranges;

  order_ranges(&mut ranges);

  let mut new_ranges = ~[];

  for &(start, end) in ranges.iter() {
    match new_ranges.pop_opt() {
      Some(range) => {
        let (s, e): (char, char) = range;
        if (start > e) {
          new_ranges.push((s, e));
          if (start <= end) {
            new_ranges.push((start, end))
          }
        } else if (start < e && end > e) {
          new_ranges.push((s, end))
        } else if (start < e && end < e) {
          new_ranges.push((s, e))
        } else {
          new_ranges.push((s, e))
        }
      }
      None => {
        if (start <= end) {  
          new_ranges.push((start, end))
        }
      }
    }
  }
  
  CharClass(new_ranges)
}

/// Construct a CharClass with a set of ranges, and negate them.
pub fn new_negated_charclass(ranges: ~[Range]) -> Expr {
  let mut ranges = ranges;

  order_ranges(&mut ranges);

  let mut min: char = '\U00000000'; 

  let mut new_ranges = ~[];

  for &(start, end) in ranges.iter() {
    match prev_char(start) {
      Some(e) => {
        if (min <= e && end >= start) {
          new_ranges.push((min, e));
        }
      },
      None => () 
    };
    if (min <= end) {
      min = match next_char(end) {
        Some(c) => c,
        None => end
      };
    }
  }

  // Patch the end
  if (min != MAX) {
    new_ranges.push((min, MAX));
  }
  
  CharClass(new_ranges)
}

#[cfg(test)]
mod char_class_tests {
  use std::char::MAX;
  use charclass::*;
  use parse::{Expr, CharClass};

  fn unravel_cc(cc: Expr) -> ~[Range] {
    match cc {
      CharClass(ranges) => ranges,
      _ => fail!()
    }
  }

  #[test]
  fn char_class_good() {
    let cc = new_charclass(~[('A', 'Z'), ('F', 'F'), ('A', 'あ')]);
    assert_eq!(unravel_cc(cc), ~[('A', 'あ')]); 
  }

  #[test]
  fn char_class_empty() {
    let cc = new_charclass(~[('Z', 'A')]);
    assert_eq!(unravel_cc(cc), ~[]);
  }

  #[test]
  fn char_class_negate() {
    let cc = new_negated_charclass(~[('A', '\uFA08')]);
    assert_eq!(unravel_cc(cc), ~[('\u0000', '@'), ('\uFA09', MAX)]);
  }

  #[test]
  fn char_class_negate_multiple() {
    let cc = new_negated_charclass(~[('們', '我'), ('A', 'Z')]);
    assert_eq!(unravel_cc(cc), ~[('\u0000', '@'), ('[', '\u5010'), 
               ('\u6212', MAX)]);
  }

  #[test]
  fn char_class_negate_overlap() {
    let cc = new_negated_charclass(~[('a', 'c'), ('c', 'c')]);
    assert_eq!(unravel_cc(cc), ~[('\u0000', '`'), ('d', MAX)]);
  }

  #[test]
  fn char_class_negate_bounds() {
    let cc = new_negated_charclass(~[('\u0000', MAX)]);
    assert_eq!(unravel_cc(cc), ~[]);
  }

  #[test]
  fn char_class_overlapping_ranges() {
    let cc = new_charclass(~[('A', 'D'), ('B', 'C')]);
    assert_eq!(unravel_cc(cc), ~[('A', 'D')]);
  }

  #[test]
  fn char_class_repeated_ranges() {
    let cc = new_charclass(~[('A', 'D'), ('A', 'D')]);
    assert_eq!(unravel_cc(cc), ~[('A', 'D')]);
  }

  #[test]
  fn char_class_overlapping_ranges2() {
    let cc = new_charclass(~[('A', 'D'), ('B', 'E')]);
    assert_eq!(unravel_cc(cc), ~[('A', 'E')]);
  }

  fn char_class_negate_sequential() {
    let cc = new_charclass(~[('a', 'a'), ('b', 'b'), ('c', 'c')]);
    assert_eq!(unravel_cc(cc), ~[('\u0000', '`'), ('d', MAX)]);
  }
}
