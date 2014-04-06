use exec::{ExecStrategy, PikeVM};
use compile::Instruction;
use result::Match;
use parse::parse;
use compile::compile_recursive;
use error::ParseError::*;

/// Uncompiled regular expression. 
pub struct UncompiledRegexp {
  prog: ~[Instruction]
}

/// Constructors
impl UncompiledRegexp {
  pub fn new(s: &str) -> Result<UncompiledRegexp, ParseCode> {
    match parse(s) {
      Ok(ref expr) => {
        let prog = compile_recursive(expr);
        Ok(UncompiledRegexp { prog: prog })
      }
      Err(e) => Err(e)
    }
  }
}

/// TODO:
/// The API needs some work.
/// Allow for other implementations to be used?
impl UncompiledRegexp { 
  /// Checks if the beginning of the input string 
  /// contains a match, and returns it.
  pub fn exec(&self, input: &str) -> Option<Match> {
    let strat = PikeVM::new(self.prog, 0);
    match strat.run(input, 0) {
      Some(t) => {
        Some(Match::new(0, t.end, input, t.captures))
      }
      None => None
    }
  }
  /// Finds the first occurrence of the pattern in the 
  /// input string and returns it.
  pub fn search(&self, input: &str) -> Option<Match> {
    let len = input.len();
    let strat = PikeVM::new(self.prog, 0);

    for start in range(0, len + 1) {
      match strat.run(input, start) {
        Some(t) => {
          return Some(Match::new(start, t.end, input, t.captures))
        }
        None => ()
      }
    }

    None
  }

  // pub fn split(&self, input: &str) -> Option<Match> {
  //
  // }

  pub fn find_all(&self, input: &str) -> ~[Match] {
    let mut matches : ~[Match] = ~[];

    let len = input.len();
    let strat = PikeVM::new(self.prog, 0); 

    let mut start = 0;
    for _ in range(0, len + 1) {  // Run starting at each character
        match strat.run(input, start) { // run only matches one thing...
          Some(t) => {
            let nextPos = t.end;
            matches.push(Match::new(start, t.end, input, t.captures));
            start = nextPos;
          }
          None => {
            start += 1;
          }
        }
    }

    return matches;
  }

  // pub fn find_iter(&self, input: &str) -> Option<Match> {
  // 
  // }

  pub fn replace(&self, input: &str, replaceWith: &str) -> ~str {
    let len = input.len();
    let strat = PikeVM::new(self.prog, 0); 
    let mut replaced = input.to_owned();
    let mut start = 0;
    let emptyPatternAdd = if self.prog.len()==1 {1} else {0};

    while len != 0{
      match strat.run(replaced, start) {
        Some(t) => {
          replaced = format!("{:s}{:s}{:s}", replaced.slice_to(start), replaceWith, replaced.slice_from(t.end));
          start += replaceWith.len() + emptyPatternAdd;
        }
        None => {
          start += 1;
        }
      }
      if start > replaced.len() {break}
    }

    replaced
  }

  pub fn replacen(&self, input: &str, replaceWith: &str) -> (~str, uint) {
    let len = input.len();
    let strat = PikeVM::new(self.prog, 0); 
    let mut replaced = input.to_owned();
    let mut start = 0;
    let emptyPatternAdd = if self.prog.len()==1 {1} else {0};
    let mut repCount = 0;

    while len != 0{
      match strat.run(replaced, start) {
        Some(t) => {
          replaced = format!("{:s}{:s}{:s}", replaced.slice_to(start), replaceWith, replaced.slice_from(t.end));
          start += replaceWith.len() + emptyPatternAdd;
          repCount += 1;
        }
        None => {
          start += 1;
        }
      }
      if start > replaced.len() {break}
    }

    (replaced, repCount)
  }

}

#[cfg(test)]
mod library_functions_test {
  use super::*;

  macro_rules! test_replace(
    ($re: expr, $input: expr, $replaceWith: expr, $expect: expr) => (
      {
        let re = match UncompiledRegexp::new($re) {
          Ok(regex) => regex,
          Err(e) => fail!(e)
        };
        let result = re.replace($input, $replaceWith);
        if result != ~$expect {
          fail!(format!("Replacing {:s} in {:s} with {:s} yielded {:s}, not expected result of {:s}\n", $re, $input, $replaceWith, result, $expect));
        }
      }
    );
  )

  macro_rules! test_replacen(
    ($re: expr, $input: expr, $replaceWith: expr, $expect: expr, $expectCount: expr) => (
      {
        let re = match UncompiledRegexp::new($re) {
          Ok(regex) => regex,
          Err(e) => fail!(e)
        };
        let result = re.replacen($input, $replaceWith);
        match result {
          (answer, repCount) => {
            if answer != ~$expect || repCount != $expectCount {
              fail!(format!("Replacing {:s} in {:s} with {:s} yielded {:s} with {:u} replaces, not expected result of {:s} with {:d} replaces\n", 
                $re, $input, $replaceWith, answer, repCount, $expect, $expectCount));
            }
		  }
        }
      }
    );
  )

  macro_rules! test_find_all(
    ($re: expr, $input: expr, $expect: expr) => (
      {
        let re = match UncompiledRegexp::new($re) {
          Ok(regex) => regex,
          Err(e) => fail!(e)
        };
        let result = re.find_all($input);
        let mut i = 0;
        for &item in $expect.iter() {
          if i >= result.len() {
            fail!(format!("Results list only has {:u} elements, expected to have {:u}\n", i, $expect.len()));
          }
          let res = result[i].input.slice(result[i].start, result[i].end);
          if res != item {
            fail!(format!("Find-all on regexp '{:s}' yielded '{:s}' at element {:u} of results list, not expected result of '{:s}'\n", $re, res, i, item.clone()));
          }
          i = i + 1;
        }
      }
    );
  )

  #[test]
  fn test_replace_01() {
    test_replace!("a*ba*", "abaaacaabaaaccdab", "", "cccd");
  }

  #[test]
  fn test_replace_02() {
    test_replace!("a*ba{1,}", "abaaacaabaaacca", "", "ccca");
  }

  #[test]
  fn test_replace_03() {
    test_replace!("a*ba{1,}", "abaaacaabaaacca", "aba", "abacabacca");
  }

  #[test]
  fn test_replace_04() {
    test_replace!("a", "aaaaaaaaaaaa", "b", "bbbbbbbbbbbb");
  }

  #[test]
  fn test_replace_05() {
    test_replace!("a{1,}", "aaaaaaaaaaaa", "b", "b");
  }
  
  #[test]
  fn test_replace_06() {
    test_replace!("a{1,}", "aaaaaaaaaaaa", "", "");
  }
  
  #[test]
  fn test_replace_07() {
    test_replace!("", "aaaa", "b", "babababab");
  }

  #[test]
  fn test_replace_08() {
    test_replace!("a?bab", "abababab", "c", "cc");
  }

  #[test]
  fn test_replace_09() {
    test_replace!("a", "aa", "ccc", "cccccc");
  }

  #[test]
  fn test_replace_10() {
    test_replace!("b", "aa", "ccc", "aa");
  }

  #[test]
  fn test_replacen_01() {
    test_replacen!("a*ba*", "abaaacaabaaaccdab", "", "cccd", 3);
  }

  #[test]
  fn test_replacen_02() {
    test_replacen!("a*ba{1,}", "abaaacaabaaacca", "", "ccca", 2);
  }

  #[test]
  fn test_replacen_03() {
    test_replacen!("a*ba{1,}", "abaaacaabaaacca", "aba", "abacabacca", 2);
  }

  #[test]
  fn test_replacen_04() {
    test_replacen!("a", "aaaaaaaaaaaa", "b", "bbbbbbbbbbbb", 12);
  }

  #[test]
  fn test_replacen_05() {
    test_replacen!("a{1,}", "aaaaaaaaaaaa", "b", "b", 1);
  }
  
  #[test]
  fn test_replacen_06() {
    test_replacen!("a{1,}", "aaaaaaaaaaaa", "", "", 1);
  }
  
  #[test]
  fn test_replacen_07() {
    test_replacen!("", "aaaa", "b", "babababab", 5);
  }

  #[test]
  fn test_replacen_08() {
    test_replacen!("a?bab", "abababab", "c", "cc", 2);
  }

  #[test]
  fn test_replacen_09() {
    test_replacen!("a", "aa", "ccc", "cccccc", 2);
  }

  #[test]
  fn test_replacen_10() {
    test_replacen!("b", "aa", "ccc", "aa", 0);
  }

  #[test]
  fn test_find_all_01() {
    test_find_all!("a*ba*", "abaaacaabaaaccdab", &["abaaa", "aabaaa", "ab"]);
  }

  #[test]
  fn test_find_all_02() {
    test_find_all!("a*ba{1,}", "abaaacaabaaaccab", &["abaaa", "aabaaa"]);
  }

  #[test]
  fn test_find_all_03() {
    test_find_all!("a*ba{1,}", "abaaacaabaaaccab", &["abaaa", "aabaaa"]);
  }

  #[test]
  fn test_find_all_04() {
    test_find_all!("a", "aaaaaaaaaaaa", &["a", "a", "a", "a", "a", "a", "a", 
      "a", "a", "a", "a", "a"]);
  }

  #[test]
  fn test_find_all_05() {
    test_find_all!("a{1,}", "aaaaaaaaaaaa", &["aaaaaaaaaaaa"]);
  }
  
  #[test]
  fn test_find_all_06() {
    test_find_all!("a{1,}", "aaabaaaabaaa", &["aaa", "aaaa", "aaa"]);
  }
  
  #[test]
  fn test_find_all_07() {
    test_find_all!("", "aaaa", &["", "", "", ""]);
  }

  #[test]
  fn test_find_all_08() {
    test_find_all!("a?bab", "ababababbab", &["abab", "abab", "bab"]);
  }

  #[test]
  fn test_find_all_09() {
    test_find_all!("a", "aa", &["a", "a"]);
  }

  #[test]
  fn test_find_all_10() {
    test_find_all!("a*b*c*d*", "abcdbabcdabcbababcbdabcbdaabbbccccddddd", &["abcd", 
      "b", "abcd", "abc", "b", "ab", "abc", "bd", "abc", "bd", "aabbbccccddddd"]);
  }
}


#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn parse_alternation_ok_test() {
    assert!(UncompiledRegexp::new("a|b").is_ok());
  }

  #[test]
  fn parse_concatenation_ok_test() {
    assert!(UncompiledRegexp::new("a(bc)d").is_ok());
  }

  #[test]
  fn parse_char_class_ok_test() {
    assert!(UncompiledRegexp::new("[a-zABC!@#]]]").is_ok());
  }

  #[test]
  fn parse_capture_ok_test() {
    assert!(UncompiledRegexp::new("(hel(ABC)ok)").is_ok());
  }

  #[test]
  fn parse_capture_fail_test() {
    assert!(UncompiledRegexp::new("(hel(ABC)ok").is_err());
  }

  #[test]
  fn search_group_fetch() {
    match UncompiledRegexp::new("(?P<hello>d)") {
      Ok(regex) => {
        match regex.search("dhfs") {
          Some(m) => {
            match m.group_by_name("hello") {
              Some(result) => {
                assert_eq!(result, ~"d");
              }
              None => {
                fail!("Failed to find a group with a match");
              }
            }
          }
          None => {
            fail!("Didn't match a group when expected");
          }
        }        
      }
      Err(error) => {
        fail!(error.to_str());
      }
    }
  }
}
