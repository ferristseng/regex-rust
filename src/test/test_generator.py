# Test generator 
import re
from cases import * 
from datetime import datetime

FILE = open('src/re/test.rs', 'w')

OUTPUT = """
// This is an auto-generated test file
// Generated by src/test/test_generator.py
//
// Last Modified: %s

macro_rules! run_tests(
  ($re: expr, $input: expr, $matched: expr, $ident: expr, 
   $expect: pat, $groups: expr) => (
    {
      let re = match UncompiledRegexp::new($re) {
        Ok(regex) => regex,
        Err(e) => fail!(e)
      };
      let res = re.search($input);
      let expect_test = match res {
        $expect => true, 
        _ => {
          println(format!("Failed with test {:s}: <Re: '{:s}'> | <Input: '{:s}'>", 
                  $ident, $re, $input));
          false
        }
      };
      if (!expect_test) {
        assert!(expect_test);
        return
      }
      if (res.is_some()) {
        match res {
          Some(ma) => {
            assert_eq!(ma.matched(), $matched)
            
            let groups: &'static[&'static str] = $groups;
            let mut i = 0;

            for g in groups.iter() {
              assert_eq!(ma.group(i), g.to_str());

              i += 1;
            }
          }
          _ => () 
        }
      }
    }
  )
)

#[cfg(test)]
mod python_tests {
  use regexp::UncompiledRegexp;

  // Tests start here
  %s
}
"""

TEST_FN = \
"""
  fn test_case_ident_%s() {
    run_tests!(\"%s\", \"%s\", ~\"%s\", \"%s\", %s, &'static [%s])
  }"""

SUCCESS_FN = \
"""
  #[test]%s
""" % TEST_FN

FAIL_FN = \
"""
  #[test]
  #[should_fail]%s
""" % TEST_FN

def generate_test_num(num, digits):
  ret = str(num)
  for i in range(0, digits - len(ret)):
    ret = "0" + ret
  return ret

def generate_test_case(ident, regexp, input_str, 
    matched_str, expected, groups):
  if expected == NOMATCH:
    match = "None"
  elif expected == PARSEERR:
    match = "None"
  elif expected == MATCH:
    match = "Some(_)"

  regexp = re.sub("\\\\", "\\\\\\\\", regexp)
  input_str = re.sub("\\\\", "\\\\\\\\", input_str)
  matched_str = re.sub("\\\\", "\\\\\\\\", matched_str)

  if (len(groups) > 0):
    groups_str  = "\"" + "\", \"".join(groups) + "\""
  else:
    groups_str = ""

  test = FAIL_FN if expected == PARSEERR else SUCCESS_FN

  return test % (ident, regexp, input_str, matched_str, ident, 
      match, groups_str)

if __name__ == "__main__":
  date = datetime.today().strftime("%B %d %Y %I:%M%p")
  buf = ""

  for (i, test) in enumerate(TESTS):
    ident = generate_test_num(i, len(str(len(TESTS))))
    if (len(test) == 5):
      groups = test[4]
    else:
      groups = []
    buf += \
      generate_test_case(ident, test[0], test[1], test[2], test[3], 
                         groups)

  FILE.write(OUTPUT % (date, buf))

  print("Successfully generated test file: src/re/test.rs")
