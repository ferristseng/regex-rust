# -*- coding: utf-8 -*-

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
  ("①②③", "①②③", "①②③", MATCH),
  ("①②③", "①②③④⑤", "①②③", MATCH),
  ("①(②)③", "①②③", "①②③", MATCH),
  ("[①②③]*", "①②③", "①②③", MATCH),
  ("[^④⑤]*", "①②③", "①②③", MATCH),
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
  ("ab{0,1}c", "abc", "abc", MATCH),
  ("^abc$", "abc", "abc", MATCH),
  ("^abc$", "abcc", "", NOMATCH),
  ("^abc", "abcc", "abc", MATCH),
  ("^abc$", "aabc", "", NOMATCH),
  ("abc$", "abcc", "", NOMATCH),
  ("^", "abc", "", MATCH),
  ("$", "abc", "", MATCH),
  ("a.c", "abc", "abc", MATCH),
  ("a.c", "axc", "axc", MATCH),
  ("a.*c", "axyzc", "axyzc", MATCH),
  ("a.*c", "axyzd", "", NOMATCH),
  ("a[bc]d", "abc", "", NOMATCH),
  ("a[bc]d", "abd", "abd", MATCH),
  ("a[b-d]e", "abd", "", NOMATCH),
  ("a[b-d]e", "ace", "ace", MATCH),
  ("a[b-d]", "aac", "ac", MATCH),
  ("a[-b]", "a-", "a-", MATCH),
  ("a[\\-b]", "a-", "a-", MATCH),
  ("a[]b", "-", "", PARSEERR),
  ("a[", "-", "", PARSEERR),
  ("a\\", "-", "", PARSEERR),
  ("abc)", "-", "", PARSEERR),
  ("(abc", "-", "", PARSEERR),
  ("a]", "a]", "a]", MATCH),
  ("a[]]b", "a]b", "a]b", MATCH),
  ("a[\\]]b", "a]b", "a]b", MATCH),
  ("a[^bc]d", "aed", "aed", MATCH),
  ("a[^bc]d", "abd", "", NOMATCH),
  ("a[^-b]c", "adc", "adc", MATCH),
  ("a[^-b]c", "a-c", "", NOMATCH),
  ("a[^]b]c", "a]c", "", NOMATCH),
  ("a[^]b]c", "adc", "", MATCH),

  #--
  # Custom Tests
  #--
  ("", "", "", MATCH),
  ("a{5}", "aaaaa", "aaaaa", MATCH),
  ("a{5,}", "aaaaaaa", "aaaaaaa", MATCH),
  ("a{5,7}", "aaaaaa", "aaaaaa", MATCH),
  ("a{5,}", "aaaa", "", NOMATCH)
]

