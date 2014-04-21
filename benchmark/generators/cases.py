# -*- coding: utf-8 -*-

# Add cases below
# These are used to generated a file of benchmarks used across multiple testing languages
#

NO_LOOPS = 10000

# These are the tests we generate functions for
# (re, input)
TEST_GEN = [
  ("^abc", "abcc"),
  # ("\\P{Greek}", "\u0374", "\u0374", MATCH)
  ("^abc", "hslc"),
  ("[^^]+", "abc"),
  ("[^^]+", "^"),
  ("[^al-obc]+", "kpd"),
  ("[^al-obc]+", "abc"),
  ("[al-obc]+", "almocb"),
  ("[al-obc]+", "defzx"),
  ("a(?:b|c|d)(.)", "ace"),
  ("a(?:b|c|d)*(.)", "ace"),
  ("a(?:b|c|d)+?(.)", "ace"),
  #("[-+]?[0-9]*\\.?[0-9]+", "3.14"),
  #("<TAG\\b[^>]*>(.*?)</TAG>", "one<TAG>two</TAG>three"),
  # (")", ""),
  # ("", ""),
  ("abc", "abc"),
  ("abc", "xbc"),
  ("abc", "axc"),
  ("abc", "xabcy"),
  ("abc", "ababc"),
  ("ab*c", "abc")
]

SRCH_REG = "^abc"
TEST_SRCH = [
  ("abcc"),
  ("defzx"),
  ("almocb")
]
