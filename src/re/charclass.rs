use parse::{Expr, CharClass, RangeExpr, RangeTable, NegatedRangeTable};
use std::char::{from_u32, MAX};
use std::cmp::{Less, Greater};

pub type Range = (char, char);

/// Static Character Classes
pub mod perl {
  pub fn get_escape_table(escape_char: char) -> Option<&'static [(char, char)]> {
    match escape_char {
      'd' | 'D' => Some(digit_table),
      'w' | 'W' => Some(word_table),
      's' | 'S' => Some(whitespace_table),
      _ => None
    }
  }

  pub static digit_table : &'static [(char, char)] = &[
    ('0', '9')
  ];

  pub static word_table : &'static [(char, char)] = &[
    ('a', 'z'),
    ('A', 'Z'),
    ('_', '_')
  ];

  pub static whitespace_table : &'static [(char, char)] = &[
    (' ', ' '),
    ('\t', '\t'),
    ('\u000b', '\u000b'),
    ('\u000c', '\u000c'),
    ('\n', '\n'),
    ('\r', '\r')
  ];
}

pub mod ascii {
  pub fn get_prop_table(prop: &str) -> Option<&'static [(char,char)]> {
    match prop {
      &"alnum" => Some(alnum_table),
      &"alpha" => Some(alpha_table),
      &"ascii" => Some(ascii_table),
      &"blank" => Some(blank_table),
      &"cntrl" => Some(cntrl_table),
      &"digit" => Some(digit_table),
      &"graph" => Some(graph_table),
      &"lower" => Some(lower_table),
      &"print" => Some(print_table),
      &"punct" => Some(punct_table),
      &"space" => Some(space_table),
      &"upper" => Some(upper_table),
      &"word" => Some(word_table),
      &"xdigit" => Some(xdigit_table),
      _ => None
    }
  }

  pub static alnum_table : &'static [(char,char)] = &[
    ('0', '9'), ('A', 'Z'), ('a', 'z')
  ];

  pub static alpha_table : &'static [(char,char)] = &[
    ('A', 'Z'), ('a', 'z')
  ];

  pub static ascii_table : &'static [(char,char)] = &[
    ('\x00', '\x7f')
  ];

  pub static blank_table : &'static [(char,char)] = &[
    ('\t', '\t'), (' ', ' ')
  ];

  pub static cntrl_table : &'static [(char,char)] = &[
    ('\x00', '\x1f'), ('\x7f', '\x7f')
  ];

  pub static digit_table : &'static [(char,char)] = &[
    ('0', '9')
  ];

  pub static graph_table : &'static [(char,char)] = &[
    ('\x21', '\x7E')
  ];

  pub static lower_table : &'static [(char,char)] = &[
    ('a', 'z')
  ];

  pub static print_table : &'static [(char,char)] = &[
    ('\x20', '\x7E')
  ];

  pub static punct_table : &'static [(char,char)] = &[
    ('\x21', '\x2F'), ('\x3a', '\x40'),
    ('\x5b', '\x60'), ('\x7b', '\x7e')
  ];

  pub static space_table : &'static [(char,char)] = &[
    ('\x09', '\x0D')
  ];

  pub static upper_table : &'static [(char,char)] = &[
    ('A', 'Z')
  ];

  pub static word_table : &'static [(char,char)] = &[
    ('0', '9'), ('A', 'Z'),
    ('a', 'z'), ('_', '_')
  ];

  pub static xdigit_table : &'static [(char,char)] = &[
    ('0', '9'), ('A', 'Z'),
    ('a', 'z')
  ];
}

/// Try to get the prev character in sequence from
/// the given one.
pub fn prev_char(c: char) -> Option<char> {
  match from_u32(c as u32 - 1) {
    None => None,
    r    => r
  }
}

/// Try to get the next character in sequence from
/// the given one.
pub fn next_char(c: char) -> Option<char> {
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

    if start1 < start2 {
      Less
    } else if start1 == start2 {
      if end1 > end2 {
        Less
      } else {
        Greater
      }
    } else {
      Greater
    }
  })
}

// /// Construct a CharClass with a set of ranges. Remove
// /// overlapping ranges preferring larger ranges (ex. Given [A-DA-C],
// /// collapse to [A-D]).
// pub fn new_charclass(ranges: ~[Range]) -> ~[Range] {
//   let mut ranges = ranges;
//
//   order_ranges(&mut ranges);
//
//   let mut new_ranges = ~[];
//
//   for &(start, end) in ranges.iter() {
//     match new_ranges.pop() {
//       Some(range) => {
//         let (s, e): (char, char) = range;
//         if start > e {
//           new_ranges.push((s, e));
//           if start <= end {
//             new_ranges.push((start, end))
//           }
//         } else if start < e && end > e {
//           new_ranges.push((s, end))
//         } else if start < e && end < e {
//           new_ranges.push((s, e))
//         } else {
//           new_ranges.push((s, e))
//         }
//       }
//       None => {
//         if start <= end {
//           new_ranges.push((start, end))
//         }
//       }
//     }
//   }
//
//   new_ranges
// }

/// Construct a CharClass with a set of ranges, and negate them.
pub fn new_negated_charclass(exprs: ~[Expr]) -> ~[Expr] {
  let mut ranges = ~[];

  for expr in exprs.iter() {
    match expr {
      &RangeExpr(start, end) => {
        ranges.push((start, end));
      }
      &RangeTable(ref table) => {
        for &(start, end) in table.iter() {
          ranges.push((start, end));
        }
      }
      &NegatedRangeTable(ref table) => {
        let mut cur_start = '\x00';
        if table.len() > 0 {
          for &(start, end) in table.iter() {
            match prev_char(start) {
              Some(c) => {
                ranges.push((cur_start, c));
              }
              None => ()
            };

            match next_char(end) {
              Some(c) => cur_start = c,
              None => break
            };
          }
          if cur_start <= MAX {
            ranges.push((cur_start, MAX));
          }
        }
      }
      _ => ()
    }
  }

  order_ranges(&mut ranges);

  let mut min: char = '\U00000000';

  let mut new_ranges = ~[];

  for &(start, end) in ranges.iter() {
    match prev_char(start) {
      Some(e) => {
        if min <= e && end >= start {
          new_ranges.push(RangeExpr(min, e));
        }
      },
      None => ()
    };
    if min <= end {
      min = match next_char(end) {
        Some(c) => c,
        None => end
      };
    }
  }

  // Patch the end
  if min != MAX {
    new_ranges.push(RangeExpr(min, MAX));
  }

  new_ranges
}

#[cfg(test)]
mod char_class_tests {
  use std::char::MAX;
  use charclass::*;
  use parse::{Expr, CharClass, RangeExpr};

  fn unravel_cc(cc: Expr) -> ~[Expr] {
    match cc {
      CharClass(ranges) => ranges,
      _ => fail!()
    }
  }

  #[test]
  // fn char_class_good() {
  //   let cc = CharClass(new_charclass(~[('A', 'Z'), ('F', 'F'), ('A', 'あ')]));
  //   assert_eq!(unravel_cc(cc), ~[('A', 'あ')]);
  // }
  //
  // #[test]
  // fn char_class_empty() {
  //   let cc = CharClass(new_charclass(~[('Z', 'A')]));
  //   assert_eq!(unravel_cc(cc), ~[]);
  // }

  #[test]
  fn char_class_negate() {
    let cc = CharClass(new_negated_charclass(~[RangeExpr('A', '\uFA08')]));
    assert_eq!(unravel_cc(cc), ~[RangeExpr('\u0000', '@'), RangeExpr('\uFA09', MAX)]);
  }

  #[test]
  fn char_class_negate_multiple() {
    let cc = CharClass(new_negated_charclass(~[RangeExpr('們', '我'), RangeExpr('A', 'Z')]));
    assert_eq!(unravel_cc(cc), ~[RangeExpr('\u0000', '@'), RangeExpr('[', '\u5010'),
               RangeExpr('\u6212', MAX)]);
  }

  #[test]
  fn char_class_negate_overlap() {
    let cc = CharClass(new_negated_charclass(~[RangeExpr('a', 'c'), RangeExpr('c', 'c')]));
    assert_eq!(unravel_cc(cc), ~[RangeExpr('\u0000', '`'), RangeExpr('d', MAX)]);
  }

  #[test]
  fn char_class_negate_bounds() {
    let cc = CharClass(new_negated_charclass(~[RangeExpr('\u0000', MAX)]));
    assert_eq!(unravel_cc(cc), ~[]);
  }

  // #[test]
  // fn char_class_overlapping_ranges() {
  //   let cc = CharClass(new_charclass(~[('A', 'D'), ('B', 'C')]));
  //   assert_eq!(unravel_cc(cc), ~[('A', 'D')]);
  // }
  //
  // #[test]
  // fn char_class_repeated_ranges() {
  //   let cc = CharClass(new_charclass(~[('A', 'D'), ('A', 'D')]));
  //   assert_eq!(unravel_cc(cc), ~[('A', 'D')]);
  // }
  //
  // #[test]
  // fn char_class_overlapping_ranges2() {
  //   let cc = CharClass(new_charclass(~[('A', 'D'), ('B', 'E')]));
  //   assert_eq!(unravel_cc(cc), ~[('A', 'E')]);
  // }
  //
  // fn char_class_negate_sequential() {
  //   let cc = CharClass(new_charclass(~[('a', 'a'), ('b', 'b'), ('c', 'c')]));
  //   assert_eq!(unravel_cc(cc), ~[('\u0000', '`'), ('d', MAX)]);
  // }
}
