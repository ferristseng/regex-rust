use extra::sort::merge_sort;
use std::char::{from_u32, MAX};


/**
 * Try to get the prev character in sequence from 
 * the given one.
 */
fn prev_char(c: char) -> Option<char> {
  match from_u32(c as u32 - 1) {
    None => None,
    r    => r   
  }
}

/**
 * Try to get the next character in sequence from 
 * the given one.
 */
fn next_char(c: char) -> Option<char> {
  match from_u32(c as u32 + 1) {
    None => None,
    r    => r
  }
}

/**
 * Order character ranges.
 *
 * Character ranges with a greater end are preferred when 
 * equal.
 */
fn order_ranges(ranges: &[(char, char)]) -> ~[(char, char)] {
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

/**
 * Represents a series of character ranges ([A-Za-z]).
 */
pub struct CharClass {
  ranges: ~[(char, char)] 
}

impl CharClass {
  /**
   * Construct a CharClass with a set of ranges. Remove 
   * overlapping ranges preferring larger ranges (ex. Given [A-DA-C], 
   * collapse to [A-D]).
   */
  pub fn new(ranges: &[(char, char)]) -> CharClass {
    let ordered = order_ranges(ranges);

    let mut last = '\U00000000';
    let mut ranges = ~[];

    for &(start, end) in ordered.iter() {
      if (start > last && end >= start) {
        ranges.push((start, end));
      }
      last = end;
    }
    
    CharClass { 
      ranges: ranges 
    }
  }
  /**
   * Construct a CharClass with a set of ranges, and negate them.
   */
  pub fn new_negated(ranges: &[(char, char)]) -> CharClass {
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
    
    CharClass {
      ranges: ranges
    }
  }
  /**
   * Check if length of ranges is 0.
   */
  pub fn empty(&self) -> bool {
    self.ranges.len() == 0
  }
}

impl ToStr for CharClass {
  fn to_str(&self) -> ~str {
    format!("<CharClass: {:s}>", self.ranges.to_str())
  }
}

#[cfg(test)]
mod char_class_tests {
  use std::char::MAX;
  use charclass::*;

  #[test]
  fn char_class_good() {
    let cc = CharClass::new([('A', 'Z'), ('F', 'F'), ('A', 'あ')]);
    assert_eq!(cc.ranges, ~[('A', 'あ')]); 
  }

  #[test]
  fn char_class_empty() {
    let cc = CharClass::new([('Z', 'A')]);
    assert!(cc.empty());
  }

  #[test]
  fn char_class_negate() {
    let cc = CharClass::new_negated([('A', '\U0000FA08')]);
    assert_eq!(cc.ranges, ~[('\U00000000', '@'), ('\U0000FA09', MAX)]);
  }

  #[test]
  fn char_class_negate_multiple() {
    let cc = CharClass::new_negated([('們', '我'), ('A', 'Z')]);
    assert_eq!(cc.ranges, ~[('\U00000000', '@'), ('[', '\U00005010'), 
               ('\U00006212', MAX)])
  }

  #[test]
  fn char_class_negate_overlap() {
    let cc = CharClass::new_negated([('a', 'c'), ('c', 'c')]);
    assert_eq!(cc.ranges, ~[('\U00000000', '`'), ('d', MAX)]);
  }

  #[test]
  fn char_class_negate_bounds() {
    let cc = CharClass::new_negated([('\U00000000', MAX)]);
    assert!(cc.empty());
  }
}
