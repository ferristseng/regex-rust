use parse::{Expr, CharClass};
use extra::sort::merge_sort;
use std::char::{from_u32, MAX};

pub type Range = (char, char);

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
fn order_ranges(ranges: &[Range]) -> ~[Range] {
  merge_sort(ranges, |range1, range2| {
    let &(start1, end1) = range1;
    let &(start2, end2) = range2;

    if (start1 < start2) {
      true
    } else if (start1 == start2) {
      end1 > end2
    } else {
      false
    }
  })
}

/// Construct a CharClass with a set of ranges. Remove 
/// overlapping ranges preferring larger ranges (ex. Given [A-DA-C], 
/// collapse to [A-D]).
pub fn new_charclass(ranges: &[Range]) -> Expr {
  let ordered = order_ranges(ranges);

  let mut ranges = ~[];

  for &(start, end) in ordered.iter() {
    match ranges.pop_opt() {
      Some(range) => {
        let (s, e): (char, char) = range;
        if (start > e) {
          ranges.push((s, e));
          if (start <= end) {
            ranges.push((start, end))
          }
        } else if (start < e && end > e) {
          ranges.push((s, end))
        } else if (start < e && end < e) {
          ranges.push((s, e))
        } else {
          ranges.push((s, e))
        }
      }
      None => {
        if (start <= end) {  
          ranges.push((start, end))
        }
      }
    }
  }
  
  CharClass(ranges)
}

/// Construct a CharClass with a set of ranges, and negate them.
pub fn new_negated_charclass(ranges: &[Range]) -> Expr {
  let ordered = order_ranges(ranges);

  let mut min: char = '\U00000000'; 
  let max: char = MAX;

  let mut ranges = ~[];

  for &(start, end) in ordered.iter() {
    match prev_char(start) {
      Some(e) => {
        if (min < e && end >= start) {
          ranges.push((min, e));
        }
      },
      None => () 
    };
    if (min < end) {
      min = match next_char(end) {
        Some(c) => c,
        None => end
      };
    }
  }

  // Patch the end
  if (min != max) {
    ranges.push((min, max));
  }
  
  CharClass(ranges)
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
    let cc = new_charclass([('A', 'Z'), ('F', 'F'), ('A', 'あ')]);
    assert_eq!(unravel_cc(cc), ~[('A', 'あ')]); 
  }

  #[test]
  fn char_class_empty() {
    let cc = new_charclass([('Z', 'A')]);
    assert_eq!(unravel_cc(cc), ~[]);
  }

  #[test]
  fn char_class_negate() {
    let cc = new_negated_charclass([('A', '\U0000FA08')]);
    assert_eq!(unravel_cc(cc), ~[('\U00000000', '@'), ('\U0000FA09', MAX)]);
  }

  #[test]
  fn char_class_negate_multiple() {
    let cc = new_negated_charclass([('們', '我'), ('A', 'Z')]);
    assert_eq!(unravel_cc(cc), ~[('\U00000000', '@'), ('[', '\U00005010'), 
               ('\U00006212', MAX)]);
  }

  #[test]
  fn char_class_negate_overlap() {
    let cc = new_negated_charclass([('a', 'c'), ('c', 'c')]);
    assert_eq!(unravel_cc(cc), ~[('\U00000000', '`'), ('d', MAX)]);
  }

  #[test]
  fn char_class_negate_bounds() {
    let cc = new_negated_charclass([('\U00000000', MAX)]);
    assert_eq!(unravel_cc(cc), ~[]);
  }

  #[test]
  fn char_class_overlapping_ranges() {
    let cc = new_charclass([('A', 'D'), ('B', 'C')]);
    assert_eq!(unravel_cc(cc), ~[('A', 'D')]);
  }

  #[test]
  fn char_class_repeated_ranges() {
    let cc = new_charclass([('A', 'D'), ('A', 'D')]);
    assert_eq!(unravel_cc(cc), ~[('A', 'D')]);
  }

  #[test]
  fn char_class_overlapping_ranges2() {
    let cc = new_charclass([('A', 'D'), ('B', 'E')]);
    assert_eq!(unravel_cc(cc), ~[('A', 'E')]);
  }
}
