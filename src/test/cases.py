# -*- coding: utf-8 -*-

# Add cases below
# These are used to generated a file of unit tests
# in Rust
#
# As of writing this, there's no way of using the #[test]
# attribute in a macro (Rust 0.8), so I'm doing it this way.

MATCH = 1
NOMATCH = 0
PARSEERR = -1
NONE = "NONE"

# These are the tests we generate functions for
# (re, input, matched_str, expected, ..[groups])
TESTS = [
  # 0
  ("[^^]+", "abc", "", "abc", MATCH),
  ("[^^]+", "^", "", "", NOMATCH),
  ("[^al-obc]+", "kpd", "", "kpd",  MATCH),
  ("[^al-obc]+", "abc", "", "", NOMATCH),
  ("[al-obc]+", "almocb", "", "almocb", MATCH),
  ("[al-obc]+", "defzx", "", "", NOMATCH),
  ("a(?:b|c|d)(.)", "ace", "", "ace", MATCH, ["e"]),
  ("a(?:b|c|d)*(.)", "ace", "", "ace", MATCH, ["e"]),
  ("a(?:b|c|d)+?(.)", "ace", "", "ace", MATCH, ["e"]),
  ("[-+]?[0-9]*\\.?[0-9]+", "3.14", "", "3.14", MATCH),
  ("<TAG\\b[^>]*>(.*?)</TAG>", "one<TAG>two</TAG>three", "", "<TAG>two</TAG>", MATCH, ["two"]),
  # These tests cause the compiler to fail (See https://github.com/mozilla/rust/issues/4780)
  #("①②③", "①②③", "", "①②③", MATCH),
  #("①②③", "①②③④⑤", "", "①②③", MATCH),
  #("①(②)③", "①②③", "", "①②③", MATCH, ["②"]),
  #("[①②③]*", "①②③", "", "①②③", MATCH),
  #("[^④⑤]*", "①②③", "", "①②③", MATCH),
  #--
  # INSERT SKIPPED TESTS HERE
  #--
  (")", "", "", "", PARSEERR),
  ("", "", "", "", MATCH),
  ("abc", "abc", "", "abc", MATCH),
  ("abc", "xbc", "", "", NOMATCH),
  ("abc", "axc", "", "", NOMATCH),
  ("abc", "xabcy", "", "abc", MATCH),
  ("abc", "ababc", "", "abc", MATCH),
  ("ab*c", "abc", "", "abc", MATCH),
  ("ab*bc", "abbc", "", "abbc", MATCH),
  ("ab*bc", "abbbbc", "", "abbbbc", MATCH),
  ("ab{0,}bc", "abbbbc", "", "abbbbc", MATCH),
  ("ab+bc", "abbc", "", "abbc", MATCH),
  ("ab+bc", "abc", "", "", NOMATCH),
  ("ab+bc", "abq", "", "", NOMATCH),
  ("ab{1,}bc", "abq", "", "", NOMATCH),
  ("ab+bc", "abbbbc", "", "abbbbc", MATCH),
  ("ab{1,}bc", "abbbbc", "", "abbbbc", MATCH),
  ("ab{1,3}bc", "abbbbc", "", "abbbbc", MATCH),
  ("ab{3,4}bc", "abbbbc", "", "abbbbc", MATCH),
  ("ab{4,5}bc", "abbbbc", "", "abbbbc", NOMATCH),
  ("ab?bc", "abbc", "", "abbc", MATCH),
  ("ab?bc", "abc", "", "abc", MATCH),
  ("ab{0,1}bc", "abc", "", "abc", MATCH),
  ("ab?bc", "abbbbc", "", "", NOMATCH),
  ("ab?c", "abc", "", "abc", MATCH),
  ("ab{0,1}c", "abc", "", "abc", MATCH),
  ("^abc$", "abc", "", "abc", MATCH),
  ("^abc$", "abcc", "", "", NOMATCH),
  ("^abc", "abcc", "", "abc", MATCH),
  ("^abc$", "aabc", "", "", NOMATCH),
  ("abc$", "abcc", "", "", NOMATCH),
  ("^", "abc", "", "", MATCH),
  ("$", "abc", "", "", MATCH),
  ("a.c", "abc", "", "abc", MATCH),
  ("a.c", "axc", "", "axc", MATCH),
  ("a.*c", "axyzc", "", "axyzc", MATCH),
  ("a.*c", "axyzd", "", "", NOMATCH),
  ("a[bc]d", "abc", "", "", NOMATCH),
  ("a[bc]d", "abd", "", "abd", MATCH),
  ("a[b-d]e", "abd", "", "", NOMATCH),
  ("a[b-d]e", "ace", "", "ace", MATCH),
  ("a[b-d]", "aac", "", "ac", MATCH),
  ("a[-b]", "a-", "", "a-", MATCH),
  ("a[\\-b]", "a-", "", "a-", MATCH),
  ("a[]b", "-", "", "", PARSEERR),
  ("a[", "-", "", "", PARSEERR),
  ("a\\", "-", "", "", PARSEERR),
  ("abc)", "-", "", "", PARSEERR),
  ("(abc", "-", "", "", PARSEERR),
  ("a]", "a]", "", "a]", MATCH),
  ("a[]]b", "a]b", "", "a]b", MATCH),
  ("a[\\]]b", "a]b", "", "a]b", MATCH),
  ("a[^bc]d", "aed", "", "aed", MATCH),
  ("a[^bc]d", "abd", "", "", NOMATCH),
  ("a[^-b]c", "adc", "", "adc", MATCH),
  ("a[^-b]c", "a-c", "", "", NOMATCH),
  ("a[^]b]c", "a]c", "", "", NOMATCH),
  ("a[^]b]c", "adc", "", "adc", MATCH),
  ("\\ba\\b", "a-", "", "a", MATCH),
  ("\\ba\\b", "-a", "", "a", MATCH),
  ("\\ba\\b", "-a-", "", "a", MATCH),
  ("\\by\\b", "xy", "", "", NOMATCH),
  ("\\by\\b", "yz", "", "", NOMATCH),
  ("\\by\\b", "xyz", "", "", NOMATCH),
  ("x\\b", "xyz", "", "", NOMATCH),
  ("x\\B", "xyz", "", "x", MATCH),
  ("\\Ba\\B", "a-", "", "", NOMATCH),
  ("\\Ba\\B", "-a", "", "", NOMATCH),
  ("\\Ba\\B", "-a-", "", "", NOMATCH),
  ("\\By\\B", "xy", "", "", NOMATCH),
  ("\\By\\B", "yz", "", "", NOMATCH),
  ("\\By\\b", "xy", "", "y", MATCH),
  ("\\by\\B", "yz", "", "y", MATCH),
  ("\\By\\B", "xyz", "", "y", MATCH),
  ("ab|cd", "abc", "", "ab", MATCH),
  ("ab|cd", "abcd", "", "ab", MATCH),
  ("()ef", "def", "", "ef", MATCH, [""]), # Check this
  ("$b", "b", "", "", NOMATCH),
  ("a\\(b", "a(b", "", "a(b", MATCH),
  ("a\\(*b", "ab", "", "ab", MATCH),
  ("a\\(*b", "a((b", "", "a((b", MATCH),
  ("a\\\\b", "a\\\\b", "", "a\\\\b", MATCH),
  ("((a))", "abc", "", "a", MATCH, ["a", "a"]),
  ("(a)b(c)", "abc", "", "abc", MATCH, ["a", "c"]),
  ("a+b+c", "aabbabc", "", "abc", MATCH),
  ("(a+|b)*", "ab", "", "ab", MATCH, ["b"]),
  ("(a+|b)+", "ab", "", "ab", MATCH, ["b"]),
  ("(a+|b)?", "ab", "", "a", MATCH, ["a"]),
  (")(", "-", "", "", PARSEERR),
  ("[^ab]*", "cde", "", "cde", MATCH),
  ("abc", "", "", "", NOMATCH),
  ("a*", "", "", "", MATCH),
  ("a|b|c|d|e", "e", "", "e", MATCH),
  ("(a|b|c|d|e)f", "ef", "", "ef", MATCH, ["e"]),
  ("abcd*efg", "abcdefg", "", "abcdefg", MATCH),
  ("ab*", "xabyabbbz", "", "ab", MATCH),
  ("ab*", "xayabbbz", "", "a", MATCH),
  ("(ab|cd)e", "abcde", "", "cde", MATCH, ["cd"]),
  ("[abhgefdc]ij", "hij", "", "hij", MATCH),
  ("^(ab|cd)e", "abcde", "", "", NOMATCH),
  ("(abc|)ef", "abcdef", "", "ef", MATCH, [""]), # Check this
  ("(a|b)c*d", "abcd", "", "bcd", MATCH, ["b"]),
  ("(ab|ab*)bc", "abc", "", "abc", MATCH, ["a"]),
  ("a([bc]*)c*", "abc", "", "abc", MATCH, ["bc"]),
  ("a([bc]*)(c*d)", "abcd", "", "abcd", MATCH, ["bc", "d"]),
  ("a([bc]+)(c*d)", "abcd", "", "abcd", MATCH, ["bc", "d"]),
  ("a([bc]*)(c+d)", "abcd", "", "abcd", MATCH, ["b", "cd"]),
  ("a[bcd]*dcdcde", "adcdcde", "", "adcdcde", MATCH),
  ("a[bcd]+dcdcde", "adcdcde", "", "", NOMATCH),
  ("(ab|a)b*c", "abc", "", "abc", MATCH, ["ab"]),
  ("((a)(b)c)(d)", "abcd", "", "abcd", MATCH, ["abc", "a", "b", "d"]),
  ("[a-zA-Z_][a-zA-Z0-9_]*", "alpha", "", "alpha", MATCH),
  ("^a(bc+|b[eh])g|.h$", "abh", "", "bh", MATCH, [NONE]),
  ("(bc+d$|ef*g.|h?i(j|k))", "effgz", "", "effgz", MATCH, ["effgz", NONE]),
  ("(bc+d$|ef*g.|h?i(j|k))", "ij", "", "ij", MATCH, ["ij", "j"]),
  ("(bc+d$|ef*g.|h?i(j|k))", "effg", "", "", NOMATCH),
  ("(bc+d$|ef*g.|h?i(j|k))", "bcdd", "", "", NOMATCH),
  ("(bc+d$|ef*g.|h?i(j|k))", "reffgz", "", "effgz", MATCH, ["effgz", NONE]),
  ("(((((((((a)))))))))", "a", "", "a", MATCH, ["a", "a", "a", "a", "a", "a", "a", "a", "a"]),
  ("multiple words of text", "uh-uh", "", "", NOMATCH),
  ("multiple words", "multiple words, yeah", "", "multiple words", MATCH),
  ("(.*)c(.*)", "abcde", "", "abcde", MATCH, ["ab", "de"]),
  ("\\((.*), (.*)\\)", "(a, b)", "", "(a, b)", MATCH, ["a", "b"]),
  ("[k]", "ab", "", "", NOMATCH),
  ("a[-]?c", "ac", "", "ac", MATCH),
  #("(abc)\\1", "abcabc", "", "abcabc", MATCH),
  #("([a-c]*)\\1", "abcabc", "", "abcabc", MATCH),
  ("^(.+)?B", "AB", "", "AB", MATCH, ["A"]),
  #("(a+).\\1$", "aaaaa", "", "aaaaa", MATCH),
  #("^(a+).\\1$", "aaaa", "", "", NOMATCH),
  #--
  # Custom Tests
  #--
  ("a{5}", "aaaaa", "", "aaaaa", MATCH),
  ("a{5,}", "aaaaaaa", "", "aaaaaaa", MATCH),
  ("a{5,7}", "aaaaaa", "", "aaaaaa", MATCH),
  ("a{5,}", "aaaa", "", "", NOMATCH),

  # Nested character class tests
  ("[a-e[g]]", "d]", "", "d]", MATCH),
  ("[a-e[g]]", "g]", "", "g]", MATCH),
  ("[a-e[g]]", "[]", "", "[]", MATCH),
  ("[a-e[g]]", "]]", "", "]]", NOMATCH),
  ("[[g-p][a-d]]", "[c]", "", "[c]", MATCH),
  ("[(a-d)]", "c", "", "c", MATCH),
  ("[(a-d)]", "(", "", "(", MATCH),

  # Unicode character class tests
  ("\\p{Nd}", '\u06f0', "", '\u06f0', MATCH),
  ("\\p{Nd}", "\U000104af", "", "", NOMATCH),
  ("\\P{Nd}", "\U000104af", "", "\U000104af", MATCH),
  ("\\P{Nd}", "\u06f0", "", "", NOMATCH),
  ("\\p{Greek}", "\U00010181", "", "\U00010181", MATCH),
  ("\\p{Greek}", "\u0374", "", "", NOMATCH),
  ("\\P{Greek}", "\U00010181", "", "", NOMATCH),
  ("\\P{Greek}", "\u0374", "", "\u0374", MATCH),

  # Dotall flag test
  ("a.b", "a\\nb", "", "", NOMATCH),
  ("a.b", "a\\nb", "s", "a\\nb", MATCH),
  (".", "\\n", "s", "\\n", MATCH),

  # Multiline flag test
  ("^a$", "a\\nb\\nc", "", "", NOMATCH),
  ("^a$", "a\\nb\\nc", "m", "a", MATCH),
  ("^b$", "a\\nb\\nc", "", "", NOMATCH),
  ("^b$", "a\\nb\\nc", "m", "b", MATCH),
  ("^c$", "a\\nb\\nc", "", "", NOMATCH),
  ("^c$", "a\\nb\\nc", "m", "c", MATCH),

  # Ungreedy flag test
  ("a*", "aaaa", "", "aaaa", MATCH),
  ("a*", "aaaa", "U", "", MATCH),
  ("a*?", "aaaa", "", "", MATCH),
  ("a*?", "aaaa", "U", "aaaa", MATCH),
  ("a{1,3}", "aaaa", "", "aaa", MATCH),
  ("a{1,3}", "aaaa", "U", "a", MATCH),
  ("a{1,3}?", "aaaa", "", "a", MATCH),
  ("a{1,3}?", "aaaa", "U", "aaa", MATCH),

  # Case insensitive Unicode character classes
  ("\\p{Lu}", "a", "", "", NOMATCH),
  ("\\p{Lu}", "a", "i", "a", MATCH),
  ("\\p{Lu}", "A", "", "A", MATCH),
  ("\\p{Lu}", "A", "i", "A", MATCH),
  ("\\p{Lu}", "0", "i", "", NOMATCH),
  ("\\p{Ll}", "A", "", "", NOMATCH),
  ("\\p{Ll}", "A", "i", "A", MATCH),
  ("\\p{Ll}", "a", "", "a", MATCH),
  ("\\p{Ll}", "a", "i", "a", MATCH),
  ("\\p{Ll}", "0", "i", "", NOMATCH),
  ("\\P{Lu}", "a", "", "a", MATCH),
  ("\\P{Lu}", "a", "i", "a", MATCH),
  ("\\P{Lu}", "A", "", "", NOMATCH),
  ("\\P{Lu}", "A", "i", "A", MATCH),
  ("\\P{Lu}", "0", "i", "0", MATCH),
  ("\\P{Ll}", "A", "", "A", MATCH),
  ("\\P{Ll}", "A", "i", "A", MATCH),
  ("\\P{Ll}", "a", "", "", NOMATCH),
  ("\\P{Ll}", "a", "i", "a", MATCH),
  ("\\P{Ll}", "0", "i", "0", MATCH),

  # Case insensitive ASCII character classes
  ("[:upper:]", "a", "", "", NOMATCH),
  ("[:upper:]", "a", "i", "a", MATCH),
  ("[:upper:]", "A", "", "A", MATCH),
  ("[:upper:]", "A", "i", "A", MATCH),
  ("[:upper:]", "0", "i", "", NOMATCH),
  ("[:lower:]", "A", "", "", NOMATCH),
  ("[:lower:]", "A", "i", "A", MATCH),
  ("[:lower:]", "a", "", "a", MATCH),
  ("[:lower:]", "a", "i", "a", MATCH),
  ("[:lower:]", "0", "i", "", NOMATCH),
  ("[:^upper:]", "a", "", "a", MATCH),
  ("[:^upper:]", "a", "i", "a", MATCH),
  ("[:^upper:]", "A", "", "", NOMATCH),
  ("[:^upper:]", "A", "i", "A", MATCH),
  ("[:^upper:]", "0", "i", "0", MATCH),
  ("[:^lower:]", "A", "", "A", MATCH),
  ("[:^lower:]", "A", "i", "A", MATCH),
  ("[:^lower:]", "a", "", "", NOMATCH),
  ("[:^lower:]", "a", "i", "a", MATCH),
  ("[:^lower:]", "0", "i", "0", MATCH),

  # Case insensitive character literals
  ("abc", "AbC", "", "", NOMATCH),
  ("abc", "AbC", "i", "AbC", MATCH),

  # Case insensitive escape characters
  ("\\e", "sdfE", "", "", NOMATCH),
  ("\\e", "sdfE", "i", "E", MATCH),
  ("\\e", "sdfe", "", "e", MATCH),
  ("\\e", "sdfe", "i", "e", MATCH),
  ("\\E", "sdfe", "", "", NOMATCH),
  ("\\E", "sdfe", "i", "e", MATCH),
  ("\\E", "sdfE", "", "E", MATCH),
  ("\\E", "sdfE", "i", "E", MATCH),

  # Character class insensitive escape characters
  ("[a-ce]", "B", "", "", NOMATCH),
  ("[a-ce]", "B", "i", "B", MATCH),
  ("[a-ce]", "b", "", "b", MATCH),
  ("[a-ce]", "b", "i", "b", MATCH),
  ("[a-ce]", "d", "i", "", NOMATCH),
  ("[^a-ce]", "B", "", "B", MATCH),
  ("[^a-ce]", "B", "i", "B", MATCH),
  ("[^a-ce]", "b", "", "", NOMATCH),
  ("[^a-ce]", "b", "i", "b", MATCH),
  ("[^a-ce]", "d", "i", "d", MATCH),

  # Mixing character ranges with built-in character classes in negated character class (bug)
  ("[^a-f\\d]", "e", "", "", NOMATCH),
  ("[^a-f\\d]", "3", "", "", NOMATCH),
  ("[^0-3\\D]", "2", "", "", NOMATCH),
  ("[^0-3\\D]", "4", "", "4", MATCH),
  ("[^a-f\\p{Greek}]", "\u03c3", "", "", NOMATCH),
  ("[^a-f\\p{Greek}]", "3", "", "3", MATCH),
  ("[^a-f\\P{Greek}]", "\u03c3", "", "\u03c3", MATCH),
  ("[^a-f\\P{Greek}]", "c", "", "", NOMATCH),

  # Parse flags in expressions
  ("(?i:a)a", "AA", "", "", NOMATCH),
  ("(?i:a)a", "Aa", "", "Aa", MATCH),
  ("(?i:a)a", "aa", "", "aa", MATCH),
  ("(?i:a)a", "aA", "", "", NOMATCH),
  ("(?m:^)a$", "\\na\\n", "", "", NOMATCH),
  ("(?m:^)a$", "\\na", "", "a", MATCH),
  ("(?m:^)a$", "a\\n", "", "", NOMATCH),
  ("(?m:^)a$", "a", "", "a", MATCH),
  ("(?s:.).", "\\na", "", "\\na", MATCH),
  ("(?s:.).", "\\n\\n", "", "", NOMATCH),
  ("(?U:a{1,3})", "aaa", "", "a", MATCH),
  ("((?i)a)a", "AA", "", "", NOMATCH),
  ("((?i)a)a", "Aa", "", "Aa", MATCH),
  ("((?i)a)a", "aa", "", "aa", MATCH),
  ("((?i)a)a", "aA", "", "", NOMATCH),
  ("((?m)^)a$", "\\na\\n", "", "", NOMATCH),
  ("((?m)^)a$", "\\na", "", "a", MATCH),
  ("((?m)^)a$", "a\\n", "", "", NOMATCH),
  ("((?m)^)a$", "a", "", "a", MATCH),
  ("((?s).).", "\\na", "", "\\na", MATCH),
  ("((?s).).", "\\n\\n", "", "", NOMATCH),
  ("((?U)a{1,3})", "aaa", "", "a", MATCH),

  # Hex character code escape tests
  ("\\x54", 'T', "", 'T', MATCH),
  ("\\x79", '\x79', "", '\x79', MATCH),
  ("\\x00", '7', "", '', NOMATCH),
  ("\\x2B", '+', "", '+', MATCH),
  ("\\x2b", '+', "", '+', MATCH),
  ("\\x4g", 'Test', "", '', PARSEERR),
  ("\\x32\\x45+\\x30*", '\x32\x45\x45\x45', "", '\x32\x45\x45\x45', MATCH),
  ("\\x{54}", 'T', "", 'T', MATCH),
  ("\\x{DbB0}", '\u06f0', "", '\u06f0', MATCH),
  ("\\x{54}\\x{DbB0}+\\x{36}*", 'T\u06f0\u06f0\u06f0', "", 'T\u06f0\u06f0\u06f0', MATCH),
  ("\\x{}", 'Test', "", '', PARSEERR),
  ("\\x{000}", 'Test', "", '', PARSEERR),
  ("\\x{00000000}", 'Test', "", '', PARSEERR),

  # Octal character code escape tests
  ("\\61", '1', "", '1', MATCH),
  ("\\061", '1', "", '1', MATCH),
  ("\\175", '}', "", '}', MATCH),
  ("\\615", '15', "", '15', MATCH),
  ("\\615", '1', "", '1', NOMATCH),
  ("\\77\\123+\\111*", '?SSSSS', "", '?SSSSS', MATCH),

  # Special character escape tests
  ("\\v", '\v', "", '\v', MATCH),
  ("\\f", '\f', "", '\f', MATCH),
  ("\\n", '\n', "", '\n', MATCH),
  ("\\t", '\t', "", '\t', MATCH),
  ("\\r", '\r', "", '\r', MATCH),
  ("\\v\\f*\\n\\t+\\r", '\v\f\f\n\t\t\r', "", '\v\f\f\n\t\t\r', MATCH),
  ("\\T", '\t', "", '\t', NOMATCH),

  # Literal string escape tests
  ("\\QThis is the string!\\E", 'This is the string!', "", 'This is the string!', MATCH),
  ("\\Q((a)*)*\\E", '((a)*)*', "", '((a)*)*', MATCH),
  ("(\\Q({[\\E)*", '({[({[({[({[({[', "", '({[({[({[({[({[', MATCH),
  ("\\Q\\E", '', "", '', MATCH),

  # Single byte escape tests
  ("\\C", 'a', '', 'a', MATCH),
  ("\\C\\C", '\u06f0', '', '\u06f0', MATCH),

  # These tests are mostly for find_all
  ("a*b", "abaabaaab", "", "ab", MATCH), # Should match 9.
  ("(ab)+", "abbbbbbbab", "", "ab", MATCH), # Should match 2.

  # Tests from standard regextest basic.dat, Glenn Fowler
("abracadabra$", "abracadabracadabra", "", "abracadabra", MATCH),
("a...b", "abababbb", "", "ababb", MATCH),
("XXXXXX", "..XXXXXX", "", "XXXXXX", MATCH),
("\)", "()", "", ")", MATCH),
("a]", "a]a", "", "a]", MATCH),
("}", "}", "", "}", MATCH),
("\}", "}", "", "}", MATCH),
("\]", "]", "", "]", MATCH),
("]", "]", "", "]", MATCH),
("{", "{", "", "{", MATCH),
("}", "}", "", "}", MATCH),
("^a", "ax", "", "a", MATCH),
("\^a", "a^a", "", "^a", MATCH),
("a\^", "a^", "", "a^", MATCH),
("a$", "aa", "", "a", MATCH),
("a\$", "a$", "", "a$", MATCH),
("^$", "", "", "", NOMATCH),
("$^", "", "", "", NOMATCH),
("a($)", "aa", "", "a", MATCH),
("a*(^a)", "aa", "", "a", MATCH),
("(..)*(...)*", "a", "", "", NOMATCH),
("(..)*(...)*", "abcd", "", "abcd", MATCH),
("(ab|a)(bc|c)", "abc", "", "abc", MATCH),
("(ab)c|abc", "abc", "", "abc", MATCH),
("a{0}b", "ab", "", "b", MATCH),
("(a*)(b?)(b+)b{3}", "aaabbbbbbb", "", "aaabbbbbbb", MATCH),
("(a*)(b{0,1})(b{1,})b{3}", "aaabbbbbbb", "", "aaabbbbbbb", MATCH),
# E a{9876543210} NULL  BADBR
("((a|a)|a)", "a", "", "a", MATCH),
("(a*)(a|aa)", "aaaa", "", "aaaa", MATCH),
("a*(a.|aa)", "aaaa", "", "aaaa", MATCH),
("a(b)|c(d)|a(e)f", "aef", "", "aef", MATCH),
("(a|b)?.*", "b", "", "b", MATCH),
("(a|b)c|a(b|c)", "ac", "", "ac", MATCH),
("(a|b)c|a(b|c)", "ab", "", "ab", MATCH),
("(a|b)*c|(a|ab)*c", "abc", "", "abc", MATCH),
("(a|b)*c|(a|ab)*c", "xc", "", "c", MATCH),
("(.a|.b).*|.*(.a|.b)", "xa", "", "xa", MATCH),
("a?(ab|ba)ab", "abab", "", "abab", MATCH),
("a?(ac{0}b|ba)ab", "abab", "", "abab", MATCH),
("ab|abab", "abbabab", "", "ab", MATCH),
("aba|bab|bba", "baaabbbaba", "", "bba", MATCH),
("aba|bab", "baaabbbaba", "", "bab", MATCH),
("(aa|aaa)*|(a|aaaaa)", "aa", "", "aa", MATCH),
("(a.|.a.)*|(a|.a...)", "aa", "", "aa", MATCH),
("ab|a", "xabc", "", "ab", MATCH),
("ab|a", "xxabc", "", "ab", MATCH),
("(Ab|cD)*", "aBcD", "", "aBcD", MATCH),
("[^-]", "--a", "", "a", MATCH),
("[a-]*", "--a", "", "--a", MATCH),
("[a-m-]*", "--amoma--", "", "--am", MATCH),
(":::1:::0:|:::1:1:0:", ":::0:::1:::1:::0:", "", ":::1:::0:", MATCH),
(":::1:::0:|:::1:1:1:", ":::0:::1:::1:::0:", "", ":::1:::0:", MATCH),
# {E  [[:upper:]]   A   (0,1) [[<element>]] not supported
("[[:lower:]]+", "`az{", "", "az", MATCH),
("[[:upper:]]+", "@AZ[", "", "AZ", MATCH),
("[[-]]", "[[-]]", "", "-]", MATCH),
# [[.NIL.]] NULL  ECOLLATE
# BE  [[=aleph=]] NULL  ECOLLATE
# }
("\n", "\n", "", "\n", MATCH),
("[^a]", "\n", "", "\n", MATCH),
("\na", "\na", "", "\na", MATCH),
("(a)(b)(c)", "abc", "", "abc", MATCH),
("xxx", "xxx", "", "xxx", MATCH),
("(^|[ (,;])((([Ff]eb[^ ]* *|0*2/|\* */?)0*[6-7]))([^0-9]|$)", "feb 6,", "", "feb 6,", MATCH),
("(^|[ (,;])((([Ff]eb[^ ]* *|0*2/|\* */?)0*[6-7]))([^0-9]|$)", "2/7", "", "2/7", MATCH),
("(^|[ (,;])((([Ff]eb[^ ]* *|0*2/|\* */?)0*[6-7]))([^0-9]|$)", "feb 1,Feb 6", "", ",Feb 6", MATCH),
("((((((((((((((((((((((((((((((x))))))))))))))))))))))))))))))", "x", "", "x", MATCH),
("((((((((((((((((((((((((((((((x))))))))))))))))))))))))))))))*", "xx", "", "xx", MATCH),
("a?(ab|ba)*", "ababababababababababababababababababababababababababababababababababababababababa", "", "ababababababababababababababababababababababababababababababababababababababababa", MATCH),
("abaa|abbaa|abbbaa|abbbbaa", "ababbabbbabbbabbbbabbbbaa", "", "abbbbaa", MATCH),
("abaa|abbaa|abbbaa|abbbbaa", "ababbabbbabbbabbbbabaa", "", "abaa", MATCH),
("aaac|aabc|abac|abbc|baac|babc|bbac|bbbc", "baaabbbabac", "", "abac", MATCH),
# (".*", "\x01\xff", "", "\x01\xff", MATCH)   # not UTF-8
("aaaa|bbbb|cccc|ddddd|eeeeee|fffffff|gggg|hhhh|iiiii|jjjjj|kkkkk|llll", "XaaaXbbbXcccXdddXeeeXfffXgggXhhhXiiiXjjjXkkkXlllXcbaXaaaa", "", "aaaa", MATCH),
("aaaa\nbbbb\ncccc\nddddd\neeeeee\nfffffff\ngggg\nhhhh\niiiii\njjjjj\nkkkkk\nllll", "XaaaXbbbXcccXdddXeeeXfffXgggXhhhXiiiXjjjXkkkXlllXcbaXaaaa", "", "", NOMATCH),
("a*a*a*a*a*b", "aaaaaaaaab", "", "aaaaaaaaab", MATCH),
("^", "", "", "", NOMATCH),
("$", "", "", "", NOMATCH),
("^a$", "a", "", "a", MATCH),
("abc", "abc", "", "abc", MATCH),
("abc", "xabcy", "", "abc", MATCH),
("ab*bc", "abc", "", "abc", MATCH),
("abc$", "aabc", "", "abc", MATCH),
("a[b-]", "a-", "", "a-", MATCH),
# E a\(b      a(b   (0,3)
# E a\(*b     ab    (0,2)
# E a\(*b     a((b    (0,4)
# E ((a))     abc   (0,1)(0,1)(0,1)
# E (a)b(c)     abc   (0,3)(0,1)(2,3)
# E a+b+c     aabbabc   (4,7)
# E a*      aaa   (0,3)
# E (a*)*     -   (0,0)(0,0)
# E (a*)+     -   (0,0)(0,0)
# E (a*|b)*     -   (0,0)(0,0)
# E (a+|b)*     ab    (0,2)(1,2)
# E (a+|b)+     ab    (0,2)(1,2)
# E (a+|b)?     ab    (0,1)(0,1)
# BE  [^ab]*      cde   (0,3)
# E (^)*      -   (0,0)(0,0)
# BE  a*      NULL    (0,0)
# E ([abc])*d   abbbcd    (0,6)(4,5)
# E ([abc])*bcd   abcd    (0,4)(0,1)
# E a|b|c|d|e   e   (0,1)
# E (a|b|c|d|e)f    ef    (0,2)(0,1)
# E ((a*|b))*   -   (0,0)(0,0)(0,0)
# BE  abcd*efg    abcdefg   (0,7)
# BE  ab*     xabyabbbz (1,3)
# BE  ab*     xayabbbz  (1,2)
# BE  [abhgefdc]ij    hij   (0,3)
# E (a|b)c*d    abcd    (1,4)(1,2)
# E (ab|ab*)bc    abc   (0,3)(0,1)
# E a([bc]*)c*    abc   (0,3)(1,3)
# E a([bc]*)(c*d)   abcd    (0,4)(1,3)(3,4)
# E a([bc]+)(c*d)   abcd    (0,4)(1,3)(3,4)
# E a([bc]*)(c+d)   abcd    (0,4)(1,2)(2,4)
# E a[bcd]*dcdcde   adcdcde   (0,7)
# E (ab|a)b*c   abc   (0,3)(0,2)
# E ((a)(b)c)(d)    abcd    (0,4)(0,3)(0,1)(1,2)(3,4)
# BE  [A-Za-z_][A-Za-z0-9_]*  alpha   (0,5)
# E ^a(bc+|b[eh])g|.h$  abh   (1,3)
# E (bc+d$|ef*g.|h?i(j|k))  effgz   (0,5)(0,5)
# E (bc+d$|ef*g.|h?i(j|k))  ij    (0,2)(0,2)(1,2)
# E (bc+d$|ef*g.|h?i(j|k))  reffgz    (1,6)(1,6)
# E (((((((((a))))))))) a   (0,1)(0,1)(0,1)(0,1)(0,1)(0,1)(0,1)(0,1)(0,1)(0,1)
# BE  multiple words    multiple words yeah (0,14)
# E (.*)c(.*)   abcde   (0,5)(0,2)(3,5)
# BE  abcd      abcd    (0,4)
# E a(bc)d      abcd    (0,4)(1,3)
# E a[-]?c    ac   (0,3)
# E M[ou]'?am+[ae]r .*([AEae]l[- ])?[GKQ]h?[aeu]+([dtz][dhz]?)+af[iy] Muammar Qaddafi (0,15)(?,?)(10,12)
# E M[ou]'?am+[ae]r .*([AEae]l[- ])?[GKQ]h?[aeu]+([dtz][dhz]?)+af[iy] Mo'ammar Gadhafi  (0,16)(?,?)(11,13)
# E M[ou]'?am+[ae]r .*([AEae]l[- ])?[GKQ]h?[aeu]+([dtz][dhz]?)+af[iy] Muammar Kaddafi (0,15)(?,?)(10,12)
# E M[ou]'?am+[ae]r .*([AEae]l[- ])?[GKQ]h?[aeu]+([dtz][dhz]?)+af[iy] Muammar Qadhafi (0,15)(?,?)(10,12)
# E M[ou]'?am+[ae]r .*([AEae]l[- ])?[GKQ]h?[aeu]+([dtz][dhz]?)+af[iy] Muammar Gadafi  (0,14)(?,?)(10,11)
# E M[ou]'?am+[ae]r .*([AEae]l[- ])?[GKQ]h?[aeu]+([dtz][dhz]?)+af[iy] Mu'ammar Qadafi (0,15)(?,?)(11,12)
# E M[ou]'?am+[ae]r .*([AEae]l[- ])?[GKQ]h?[aeu]+([dtz][dhz]?)+af[iy] Moamar Gaddafi  (0,14)(?,?)(9,11)
# E M[ou]'?am+[ae]r .*([AEae]l[- ])?[GKQ]h?[aeu]+([dtz][dhz]?)+af[iy] Mu'ammar Qadhdhafi  (0,18)(?,?)(13,15)
# E M[ou]'?am+[ae]r .*([AEae]l[- ])?[GKQ]h?[aeu]+([dtz][dhz]?)+af[iy] Muammar Khaddafi  (0,16)(?,?)(11,13)
# E M[ou]'?am+[ae]r .*([AEae]l[- ])?[GKQ]h?[aeu]+([dtz][dhz]?)+af[iy] Muammar Ghaddafy  (0,16)(?,?)(11,13)
# E M[ou]'?am+[ae]r .*([AEae]l[- ])?[GKQ]h?[aeu]+([dtz][dhz]?)+af[iy] Muammar Ghadafi (0,15)(?,?)(11,12)
# E M[ou]'?am+[ae]r .*([AEae]l[- ])?[GKQ]h?[aeu]+([dtz][dhz]?)+af[iy] Muammar Ghaddafi  (0,16)(?,?)(11,13)
# E M[ou]'?am+[ae]r .*([AEae]l[- ])?[GKQ]h?[aeu]+([dtz][dhz]?)+af[iy] Muamar Kaddafi  (0,14)(?,?)(9,11)
# E M[ou]'?am+[ae]r .*([AEae]l[- ])?[GKQ]h?[aeu]+([dtz][dhz]?)+af[iy] Muammar Quathafi  (0,16)(?,?)(11,13)
# E M[ou]'?am+[ae]r .*([AEae]l[- ])?[GKQ]h?[aeu]+([dtz][dhz]?)+af[iy] Muammar Gheddafi  (0,16)(?,?)(11,13)
# E M[ou]'?am+[ae]r .*([AEae]l[- ])?[GKQ]h?[aeu]+([dtz][dhz]?)+af[iy] Moammar Khadafy (0,15)(?,?)(11,12)
# E M[ou]'?am+[ae]r .*([AEae]l[- ])?[GKQ]h?[aeu]+([dtz][dhz]?)+af[iy] Moammar Qudhafi (0,15)(?,?)(10,12)
# E a+(b|c)*d+    aabcdd      (0,6)(3,4)
# E ^.+$      vivi      (0,4)
# E ^(.+)$      vivi      (0,4)(0,4)
# E ^([^!.]+).att.com!(.+)$ gryphon.att.com!eby (0,19)(0,7)(16,19)
# E ^([^!]+!)?([^!]+)$  bas     (0,3)(?,?)(0,3)
# E ^([^!]+!)?([^!]+)$  bar!bas     (0,7)(0,4)(4,7)
# E ^([^!]+!)?([^!]+)$  foo!bas     (0,7)(0,4)(4,7)
# E ^.+!([^!]+!)([^!]+)$  foo!bar!bas   (0,11)(4,8)(8,11)
# E ((foo)|(bar))!bas bar!bas     (0,7)(0,3)(?,?)(0,3)
# E ((foo)|(bar))!bas foo!bar!bas   (4,11)(4,7)(?,?)(4,7)
# E ((foo)|(bar))!bas foo!bas     (0,7)(0,3)(0,3)
# E ((foo)|bar)!bas   bar!bas     (0,7)(0,3)
# E ((foo)|bar)!bas   foo!bar!bas   (4,11)(4,7)
# E ((foo)|bar)!bas   foo!bas     (0,7)(0,3)(0,3)
# E (foo|(bar))!bas   bar!bas     (0,7)(0,3)(0,3)
# E (foo|(bar))!bas   foo!bar!bas   (4,11)(4,7)(4,7)
# E (foo|(bar))!bas   foo!bas     (0,7)(0,3)
# E (foo|bar)!bas   bar!bas     (0,7)(0,3)
# E (foo|bar)!bas   foo!bar!bas   (4,11)(4,7)
# E (foo|bar)!bas   foo!bas     (0,7)(0,3)
# E ^(([^!]+!)?([^!]+)|.+!([^!]+!)([^!]+))$ foo!bar!bas (0,11)(0,11)(?,?)(?,?)(4,8)(8,11)
# E ^([^!]+!)?([^!]+)$|^.+!([^!]+!)([^!]+)$ bas   (0,3)(?,?)(0,3)
# E ^([^!]+!)?([^!]+)$|^.+!([^!]+!)([^!]+)$ bar!bas   (0,7)(0,4)(4,7)
# E ^([^!]+!)?([^!]+)$|^.+!([^!]+!)([^!]+)$ foo!bar!bas (0,11)(?,?)(?,?)(4,8)(8,11)
# E ^([^!]+!)?([^!]+)$|^.+!([^!]+!)([^!]+)$ foo!bas   (0,7)(0,4)(4,7)
# E ^(([^!]+!)?([^!]+)|.+!([^!]+!)([^!]+))$ bas   (0,3)(0,3)(?,?)(0,3)
# E ^(([^!]+!)?([^!]+)|.+!([^!]+!)([^!]+))$ bar!bas   (0,7)(0,7)(0,4)(4,7)
# E ^(([^!]+!)?([^!]+)|.+!([^!]+!)([^!]+))$ foo!bar!bas (0,11)(0,11)(?,?)(?,?)(4,8)(8,11)
# E ^(([^!]+!)?([^!]+)|.+!([^!]+!)([^!]+))$ foo!bas   (0,7)(0,7)(0,4)(4,7)
# E .*(/XXX).*      /XXX      (0,4)(0,4)
# E .*(\\XXX).*     \XXX      (0,4)(0,4)
# E \\XXX       \XXX      (0,4)
# E .*(/000).*      /000      (0,4)(0,4)
# E .*(\\000).*     \000      (0,4)(0,4)
# E \\000       \000      (0,4)




]
