use error::ParseError::*;
use extra::sort::merge_sort;
use std::char::{from_u32, MAX};

// try to get the previous unicode character 
// in sequence
fn prev_char(c: char) -> Option<char> {
  match from_u32(c as u32 - 1) {
    None => None,
    r    => r   
  }
}

// try to get the next unicode character 
// in sequence
fn next_char(c: char) -> Option<char> {
  match from_u32(c as u32 + 1) {
    None => None,
    r    => r
  }
}

// RegexpCharClass
// represents a character class (i.e '[a-z123]')
pub struct CharClass {
  ranges: ~[(char, char)] 
}

impl CharClass {
  pub fn new() -> CharClass {
    CharClass { ranges: ~[] }
  }
}

impl CharClass {
  pub fn negate(&mut self) -> ParseCode {
    let ordered = merge_sort(self.ranges, |range1, range2| {
      let &(start1, _) = range1;
      let &(start2, _) = range2;
      start1 <= start2
    });

    let mut min: char = '\U00000000'; 
    let max: char = MAX;

    self.ranges = ~[];

    for &(start, end) in ordered.iter() {
      match prev_char(start) {
        Some(e) => {
          if (min < e) {
            self.addRange(min, e); 
          }
        },
        None => { } // continue
      };
      if (min < end) {
        min = match next_char(end) {
          Some(c) => c,
          None => end
        };
      }
    }

    // patch the end
    if (min != max) {
      self.addRange(min, max);
    }

    if self.ranges.len() == 0 {
      ParseEmptyCharClassRange
    } else {
      ParseOk
    }
  }
  pub fn containsChar(&mut self, c: char) -> bool {
    let mut covered = false;

    for &(start, end) in self.ranges.iter() {
      if (c >= start && c <= end) {
        covered = true;
        break
      }
    }

    covered
  }
  pub fn addRange(&mut self, s: char, e: char) -> ParseCode {
    if (s <= e) {
      self.ranges.push((s, e));
    } else {
      return ParseEmptyCharClassRange;
    }

    ParseOk
  }
}

// tests

#[cfg(test)]
mod char_class_tests {
  use std::char::MAX;
  use charclass::*;
  use error::ParseError::*;

  macro_rules! create_cc(
    ([ $(($start: expr, $end: expr)),+ ]) => (
      {
        let mut cc = CharClass::new();
        $(
        cc.addRange($start, $end);
        )+
        cc
      }
    )
  )

  macro_rules! expect_code(
    ($f: expr, $code: pat) => (
      {
        let res = match $f {
          $code => true,
          _ => false
        };
        assert!(res);
      }
    )
  )
  
  #[test]
  fn char_class_good() {
    let cc = create_cc!([('A', 'Z'), ('F', 'F'), ('A', 'あ')]);
    assert_eq!(cc.ranges, ~[('A', 'Z'), ('F', 'F'), ('A', 'あ')]); 
  }

  #[test]
  fn char_class_empty() {
    let mut cc = CharClass::new();
    expect_code!(cc.addRange('Z', 'A'), ParseEmptyCharClassRange);
  }

  #[test]
  fn char_class_negate() {
    let mut cc = create_cc!([('A', '\U0000FA08')]);
    cc.negate();
    assert_eq!(cc.ranges, ~[('\U00000000', '@'), ('\U0000FA09', MAX)]);
  }

  #[test]
  fn char_class_negate_multiple() {
    let mut cc = create_cc!([('們', '我'), ('A', 'Z')]);
    cc.negate();
    assert_eq!(cc.ranges, ~[('\U00000000', '@'), ('[', '\U00005010'), 
               ('\U00006212', MAX)])
  }

  #[test]
  fn char_class_negate_overlap() {
    let mut cc = create_cc!([('a', 'c'), ('c', 'c')]);
    cc.negate();
    assert_eq!(cc.ranges, ~[('\U00000000', '`'), ('d', MAX)]);
  }

  #[test]
  fn char_class_negate_bounds() {
    let mut cc = create_cc!([('\U00000000', MAX)]);
    expect_code!(cc.negate(), ParseEmptyCharClassRange);
  }
}

