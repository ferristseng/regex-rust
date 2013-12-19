# Add cases below
# These are used to generated a file of unit tests
# in Rust

MATCH = 1
NOMATCH = 0
PARSEERR = -1

# These are the tests we generate functions for
# (re, input, matched_str, expected, groups)
TESTS = [
  # 0
  ("[^^]+", "abc", "abc", MATCH),
  ("[^^]+", "^", "", NOMATCH),
  ("[^al-obc]+", "kpd", "kpd",  MATCH),
  ("[^al-obc]+", "abc", "", NOMATCH),
  ("[al-obc]+", "almocb", "almocb", MATCH),
  ("[al-obc]+", "defzx", "", NOMATCH),
  ("a(?:b|c|d)(.)", "ace", "ace", MATCH),
  ("a(?:b|c|d)*(.)", "ace", "ace", MATCH),
  ("a(?:b|c|d)+?(.)", "ace", "ace", MATCH),
  ("[-+]?[0-9]*\\.?[0-9]+", "3.14", "3.14", MATCH),
#  ("<TAG\\b[^>]*>(.*?)</TAG>", "one<TAG>two</TAG>three", "<TAG>two</TAG>", MATCH),
  #("①②③", "①②③", "①②③", MATCH),
  #("①②③", "①②③④⑤", "①②③", MATCH),
  #("①(②)③", "①②③", "①②③", MATCH),
  #("[①②③]*", "①②③", "①②③", MATCH),
  #("[^④⑤]*", "①②③", "①②③", MATCH),
  #--
  # INSERT SKIPPED TESTS HERE
  #--
  ("abc", "abc", "abc", MATCH),
  ("abc", "xbc", "", NOMATCH),
  ("abc", "axc", "", NOMATCH),
  ("abc", "xabcy", "abc", MATCH),
  ("abc", "ababc", "abc", MATCH),
  ("ab*c", "abc", "abc", MATCH),
  ("ab*bc", "abbc", "abbc", MATCH),
  ("ab*bc", "abbbbc", "abbbbc", MATCH),
  ("ab{0,}bc", "abbbbc", "abbbbc", MATCH),
  ("ab+bc", "abbc", "abbc", MATCH),
  ("ab+bc", "abc", "", NOMATCH),
  ("ab+bc", "abq", "", NOMATCH),
  ("ab{1,}bc", "abq", "", NOMATCH),
  ("ab+bc", "abbbbc", "abbbbc", MATCH),
  ("ab{1,}bc", "abbbbc", "abbbbc", MATCH),
  ("ab{1,3}bc", "abbbbc", "abbbbc", MATCH),
  ("ab{3,4}bc", "abbbbc", "abbbbc", MATCH),
  ("ab{4,5}bc", "abbbbc", "abbbbc", NOMATCH),
  ("ab?bc", "abbc", "abbc", MATCH),
  ("ab?bc", "abc", "abc", MATCH),
  ("ab{0,1}bc", "abc", "abc", MATCH),
  ("ab?bc", "abbbbc", "", NOMATCH),
  ("ab?c", "abc", "abc", MATCH),

  #--
  # Custom Tests
  #--
  ("a{5}", "aaaaa", "aaaaa", MATCH),
  ("a{5,}", "aaaaaaa", "aaaaaaa", MATCH),
  ("a{5,7}", "aaaaaa", "aaaaaa", MATCH),
  ("a{5,}", "aaaa", "", NOMATCH)
]

