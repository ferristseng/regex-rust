
// This is an auto-generated test file
// Generated by src/test/test_generator.py
//
// Last Modified: December 24 2013 12:43AM

macro_rules! run_tests(
  ($re: expr, $input: expr, $matched: expr, $ident: expr, 
   $expect: pat, $groups: expr) => (
    {
      let mut re = UncompiledRegexp::new($re);
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
      if (res.is_ok()) {
        match res.unwrap() {
          Some(ma) => {
            assert_eq!(ma.matched(), $matched)
            
            let groups: &'static[&'static str] = $groups;
            let mut i = 0;

            for g in groups.iter() {
              assert_eq!(ma.group(i), g.to_str());

              i += 1;
            }
          }
          _ => { }
        }
      }
    }
  )
)

#[cfg(test)]
mod python_tests {
  use regexp::UncompiledRegexp;

  // Tests start here
  
  #[test]
  fn test_case_ident_000() {
    run_tests!("[^^]+", "abc", ~"abc", "000", Ok(Some(_)), &'static [])
  }

  #[test]
  fn test_case_ident_001() {
    run_tests!("[^^]+", "^", ~"", "001", Ok(None), &'static [])
  }

  #[test]
  fn test_case_ident_002() {
    run_tests!("[^al-obc]+", "kpd", ~"kpd", "002", Ok(Some(_)), &'static [])
  }

  #[test]
  fn test_case_ident_003() {
    run_tests!("[^al-obc]+", "abc", ~"", "003", Ok(None), &'static [])
  }

  #[test]
  fn test_case_ident_004() {
    run_tests!("[al-obc]+", "almocb", ~"almocb", "004", Ok(Some(_)), &'static [])
  }

  #[test]
  fn test_case_ident_005() {
    run_tests!("[al-obc]+", "defzx", ~"", "005", Ok(None), &'static [])
  }

  #[test]
  fn test_case_ident_006() {
    run_tests!("a(?:b|c|d)(.)", "ace", ~"ace", "006", Ok(Some(_)), &'static ["e"])
  }

  #[test]
  fn test_case_ident_007() {
    run_tests!("a(?:b|c|d)*(.)", "ace", ~"ace", "007", Ok(Some(_)), &'static ["e"])
  }

  #[test]
  fn test_case_ident_008() {
    run_tests!("a(?:b|c|d)+?(.)", "ace", ~"ace", "008", Ok(Some(_)), &'static ["e"])
  }

  #[test]
  fn test_case_ident_009() {
    run_tests!("[-+]?[0-9]*\\.?[0-9]+", "3.14", ~"3.14", "009", Ok(Some(_)), &'static [])
  }

  #[test]
  fn test_case_ident_010() {
    run_tests!("<TAG\\b[^>]*>(.*?)</TAG>", "one<TAG>two</TAG>three", ~"<TAG>two</TAG>", "010", Ok(Some(_)), &'static ["two"])
  }

  #[test]
  fn test_case_ident_011() {
    run_tests!("①②③", "①②③", ~"①②③", "011", Ok(Some(_)), &'static [])
  }

  #[test]
  fn test_case_ident_012() {
    run_tests!("①②③", "①②③④⑤", ~"①②③", "012", Ok(Some(_)), &'static [])
  }

  #[test]
  fn test_case_ident_013() {
    run_tests!("①(②)③", "①②③", ~"①②③", "013", Ok(Some(_)), &'static ["②"])
  }

  #[test]
  fn test_case_ident_014() {
    run_tests!("[①②③]*", "①②③", ~"①②③", "014", Ok(Some(_)), &'static [])
  }

  #[test]
  fn test_case_ident_015() {
    run_tests!("[^④⑤]*", "①②③", ~"①②③", "015", Ok(Some(_)), &'static [])
  }

  #[test]
  fn test_case_ident_016() {
    run_tests!(")", "", ~"", "016", Err(_), &'static [])
  }

  #[test]
  fn test_case_ident_017() {
    run_tests!("", "", ~"", "017", Ok(Some(_)), &'static [])
  }

  #[test]
  fn test_case_ident_018() {
    run_tests!("abc", "abc", ~"abc", "018", Ok(Some(_)), &'static [])
  }

  #[test]
  fn test_case_ident_019() {
    run_tests!("abc", "xbc", ~"", "019", Ok(None), &'static [])
  }

  #[test]
  fn test_case_ident_020() {
    run_tests!("abc", "axc", ~"", "020", Ok(None), &'static [])
  }

  #[test]
  fn test_case_ident_021() {
    run_tests!("abc", "xabcy", ~"abc", "021", Ok(Some(_)), &'static [])
  }

  #[test]
  fn test_case_ident_022() {
    run_tests!("abc", "ababc", ~"abc", "022", Ok(Some(_)), &'static [])
  }

  #[test]
  fn test_case_ident_023() {
    run_tests!("ab*c", "abc", ~"abc", "023", Ok(Some(_)), &'static [])
  }

  #[test]
  fn test_case_ident_024() {
    run_tests!("ab*bc", "abbc", ~"abbc", "024", Ok(Some(_)), &'static [])
  }

  #[test]
  fn test_case_ident_025() {
    run_tests!("ab*bc", "abbbbc", ~"abbbbc", "025", Ok(Some(_)), &'static [])
  }

  #[test]
  fn test_case_ident_026() {
    run_tests!("ab{0,}bc", "abbbbc", ~"abbbbc", "026", Ok(Some(_)), &'static [])
  }

  #[test]
  fn test_case_ident_027() {
    run_tests!("ab+bc", "abbc", ~"abbc", "027", Ok(Some(_)), &'static [])
  }

  #[test]
  fn test_case_ident_028() {
    run_tests!("ab+bc", "abc", ~"", "028", Ok(None), &'static [])
  }

  #[test]
  fn test_case_ident_029() {
    run_tests!("ab+bc", "abq", ~"", "029", Ok(None), &'static [])
  }

  #[test]
  fn test_case_ident_030() {
    run_tests!("ab{1,}bc", "abq", ~"", "030", Ok(None), &'static [])
  }

  #[test]
  fn test_case_ident_031() {
    run_tests!("ab+bc", "abbbbc", ~"abbbbc", "031", Ok(Some(_)), &'static [])
  }

  #[test]
  fn test_case_ident_032() {
    run_tests!("ab{1,}bc", "abbbbc", ~"abbbbc", "032", Ok(Some(_)), &'static [])
  }

  #[test]
  fn test_case_ident_033() {
    run_tests!("ab{1,3}bc", "abbbbc", ~"abbbbc", "033", Ok(Some(_)), &'static [])
  }

  #[test]
  fn test_case_ident_034() {
    run_tests!("ab{3,4}bc", "abbbbc", ~"abbbbc", "034", Ok(Some(_)), &'static [])
  }

  #[test]
  fn test_case_ident_035() {
    run_tests!("ab{4,5}bc", "abbbbc", ~"abbbbc", "035", Ok(None), &'static [])
  }

  #[test]
  fn test_case_ident_036() {
    run_tests!("ab?bc", "abbc", ~"abbc", "036", Ok(Some(_)), &'static [])
  }

  #[test]
  fn test_case_ident_037() {
    run_tests!("ab?bc", "abc", ~"abc", "037", Ok(Some(_)), &'static [])
  }

  #[test]
  fn test_case_ident_038() {
    run_tests!("ab{0,1}bc", "abc", ~"abc", "038", Ok(Some(_)), &'static [])
  }

  #[test]
  fn test_case_ident_039() {
    run_tests!("ab?bc", "abbbbc", ~"", "039", Ok(None), &'static [])
  }

  #[test]
  fn test_case_ident_040() {
    run_tests!("ab?c", "abc", ~"abc", "040", Ok(Some(_)), &'static [])
  }

  #[test]
  fn test_case_ident_041() {
    run_tests!("ab{0,1}c", "abc", ~"abc", "041", Ok(Some(_)), &'static [])
  }

  #[test]
  fn test_case_ident_042() {
    run_tests!("^abc$", "abc", ~"abc", "042", Ok(Some(_)), &'static [])
  }

  #[test]
  fn test_case_ident_043() {
    run_tests!("^abc$", "abcc", ~"", "043", Ok(None), &'static [])
  }

  #[test]
  fn test_case_ident_044() {
    run_tests!("^abc", "abcc", ~"abc", "044", Ok(Some(_)), &'static [])
  }

  #[test]
  fn test_case_ident_045() {
    run_tests!("^abc$", "aabc", ~"", "045", Ok(None), &'static [])
  }

  #[test]
  fn test_case_ident_046() {
    run_tests!("abc$", "abcc", ~"", "046", Ok(None), &'static [])
  }

  #[test]
  fn test_case_ident_047() {
    run_tests!("^", "abc", ~"", "047", Ok(Some(_)), &'static [])
  }

  #[test]
  fn test_case_ident_048() {
    run_tests!("$", "abc", ~"", "048", Ok(Some(_)), &'static [])
  }

  #[test]
  fn test_case_ident_049() {
    run_tests!("a.c", "abc", ~"abc", "049", Ok(Some(_)), &'static [])
  }

  #[test]
  fn test_case_ident_050() {
    run_tests!("a.c", "axc", ~"axc", "050", Ok(Some(_)), &'static [])
  }

  #[test]
  fn test_case_ident_051() {
    run_tests!("a.*c", "axyzc", ~"axyzc", "051", Ok(Some(_)), &'static [])
  }

  #[test]
  fn test_case_ident_052() {
    run_tests!("a.*c", "axyzd", ~"", "052", Ok(None), &'static [])
  }

  #[test]
  fn test_case_ident_053() {
    run_tests!("a[bc]d", "abc", ~"", "053", Ok(None), &'static [])
  }

  #[test]
  fn test_case_ident_054() {
    run_tests!("a[bc]d", "abd", ~"abd", "054", Ok(Some(_)), &'static [])
  }

  #[test]
  fn test_case_ident_055() {
    run_tests!("a[b-d]e", "abd", ~"", "055", Ok(None), &'static [])
  }

  #[test]
  fn test_case_ident_056() {
    run_tests!("a[b-d]e", "ace", ~"ace", "056", Ok(Some(_)), &'static [])
  }

  #[test]
  fn test_case_ident_057() {
    run_tests!("a[b-d]", "aac", ~"ac", "057", Ok(Some(_)), &'static [])
  }

  #[test]
  fn test_case_ident_058() {
    run_tests!("a[-b]", "a-", ~"a-", "058", Ok(Some(_)), &'static [])
  }

  #[test]
  fn test_case_ident_059() {
    run_tests!("a[\\-b]", "a-", ~"a-", "059", Ok(Some(_)), &'static [])
  }

  #[test]
  fn test_case_ident_060() {
    run_tests!("a[]b", "-", ~"", "060", Err(_), &'static [])
  }

  #[test]
  fn test_case_ident_061() {
    run_tests!("a[", "-", ~"", "061", Err(_), &'static [])
  }

  #[test]
  fn test_case_ident_062() {
    run_tests!("a\\", "-", ~"", "062", Err(_), &'static [])
  }

  #[test]
  fn test_case_ident_063() {
    run_tests!("abc)", "-", ~"", "063", Err(_), &'static [])
  }

  #[test]
  fn test_case_ident_064() {
    run_tests!("(abc", "-", ~"", "064", Err(_), &'static [])
  }

  #[test]
  fn test_case_ident_065() {
    run_tests!("a]", "a]", ~"a]", "065", Ok(Some(_)), &'static [])
  }

  #[test]
  fn test_case_ident_066() {
    run_tests!("a[]]b", "a]b", ~"a]b", "066", Ok(Some(_)), &'static [])
  }

  #[test]
  fn test_case_ident_067() {
    run_tests!("a[\\]]b", "a]b", ~"a]b", "067", Ok(Some(_)), &'static [])
  }

  #[test]
  fn test_case_ident_068() {
    run_tests!("a[^bc]d", "aed", ~"aed", "068", Ok(Some(_)), &'static [])
  }

  #[test]
  fn test_case_ident_069() {
    run_tests!("a[^bc]d", "abd", ~"", "069", Ok(None), &'static [])
  }

  #[test]
  fn test_case_ident_070() {
    run_tests!("a[^-b]c", "adc", ~"adc", "070", Ok(Some(_)), &'static [])
  }

  #[test]
  fn test_case_ident_071() {
    run_tests!("a[^-b]c", "a-c", ~"", "071", Ok(None), &'static [])
  }

  #[test]
  fn test_case_ident_072() {
    run_tests!("a[^]b]c", "a]c", ~"", "072", Ok(None), &'static [])
  }

  #[test]
  fn test_case_ident_073() {
    run_tests!("a[^]b]c", "adc", ~"adc", "073", Ok(Some(_)), &'static [])
  }

  #[test]
  fn test_case_ident_074() {
    run_tests!("\\ba\\b", "a-", ~"a", "074", Ok(Some(_)), &'static [])
  }

  #[test]
  fn test_case_ident_075() {
    run_tests!("\\ba\\b", "-a", ~"a", "075", Ok(Some(_)), &'static [])
  }

  #[test]
  fn test_case_ident_076() {
    run_tests!("\\ba\\b", "-a-", ~"a", "076", Ok(Some(_)), &'static [])
  }

  #[test]
  fn test_case_ident_077() {
    run_tests!("\\by\\b", "xy", ~"", "077", Ok(None), &'static [])
  }

  #[test]
  fn test_case_ident_078() {
    run_tests!("\\by\\b", "yz", ~"", "078", Ok(None), &'static [])
  }

  #[test]
  fn test_case_ident_079() {
    run_tests!("\\by\\b", "xyz", ~"", "079", Ok(None), &'static [])
  }

  #[test]
  fn test_case_ident_080() {
    run_tests!("x\\b", "xyz", ~"", "080", Ok(None), &'static [])
  }

  #[test]
  fn test_case_ident_081() {
    run_tests!("x\\B", "xyz", ~"x", "081", Ok(Some(_)), &'static [])
  }

  #[test]
  fn test_case_ident_082() {
    run_tests!("\\Ba\\B", "a-", ~"", "082", Ok(None), &'static [])
  }

  #[test]
  fn test_case_ident_083() {
    run_tests!("\\Ba\\B", "-a", ~"", "083", Ok(None), &'static [])
  }

  #[test]
  fn test_case_ident_084() {
    run_tests!("\\Ba\\B", "-a-", ~"", "084", Ok(None), &'static [])
  }

  #[test]
  fn test_case_ident_085() {
    run_tests!("\\By\\B", "xy", ~"", "085", Ok(None), &'static [])
  }

  #[test]
  fn test_case_ident_086() {
    run_tests!("\\By\\B", "yz", ~"", "086", Ok(None), &'static [])
  }

  #[test]
  fn test_case_ident_087() {
    run_tests!("\\By\\b", "xy", ~"y", "087", Ok(Some(_)), &'static [])
  }

  #[test]
  fn test_case_ident_088() {
    run_tests!("\\by\\B", "yz", ~"y", "088", Ok(Some(_)), &'static [])
  }

  #[test]
  fn test_case_ident_089() {
    run_tests!("\\By\\B", "xyz", ~"y", "089", Ok(Some(_)), &'static [])
  }

  #[test]
  fn test_case_ident_090() {
    run_tests!("ab|cd", "abc", ~"ab", "090", Ok(Some(_)), &'static [])
  }

  #[test]
  fn test_case_ident_091() {
    run_tests!("ab|cd", "abcd", ~"ab", "091", Ok(Some(_)), &'static [])
  }

  #[test]
  fn test_case_ident_092() {
    run_tests!("()ef", "def", ~"ef", "092", Ok(Some(_)), &'static [""])
  }

  #[test]
  fn test_case_ident_093() {
    run_tests!("$b", "b", ~"", "093", Ok(None), &'static [])
  }

  #[test]
  fn test_case_ident_094() {
    run_tests!("a\\(b", "a(b", ~"a(b", "094", Ok(Some(_)), &'static [])
  }

  #[test]
  fn test_case_ident_095() {
    run_tests!("a\\(*b", "ab", ~"ab", "095", Ok(Some(_)), &'static [])
  }

  #[test]
  fn test_case_ident_096() {
    run_tests!("a\\(*b", "a((b", ~"a((b", "096", Ok(Some(_)), &'static [])
  }

  #[test]
  fn test_case_ident_097() {
    run_tests!("a\\\\b", "a\\b", ~"a\\b", "097", Ok(Some(_)), &'static [])
  }

  #[test]
  fn test_case_ident_098() {
    run_tests!("((a))", "abc", ~"a", "098", Ok(Some(_)), &'static ["a", "a"])
  }

  #[test]
  fn test_case_ident_099() {
    run_tests!("(a)b(c)", "abc", ~"abc", "099", Ok(Some(_)), &'static ["a", "c"])
  }

  #[test]
  fn test_case_ident_100() {
    run_tests!("a+b+c", "aabbabc", ~"abc", "100", Ok(Some(_)), &'static [])
  }

  #[test]
  fn test_case_ident_101() {
    run_tests!("(a+|b)*", "ab", ~"ab", "101", Ok(Some(_)), &'static ["b"])
  }

  #[test]
  fn test_case_ident_102() {
    run_tests!("(a+|b)+", "ab", ~"ab", "102", Ok(Some(_)), &'static ["b"])
  }

  #[test]
  fn test_case_ident_103() {
    run_tests!("(a+|b)?", "ab", ~"a", "103", Ok(Some(_)), &'static ["a"])
  }

  #[test]
  fn test_case_ident_104() {
    run_tests!(")(", "-", ~"", "104", Err(_), &'static [])
  }

  #[test]
  fn test_case_ident_105() {
    run_tests!("[^ab]*", "cde", ~"cde", "105", Ok(Some(_)), &'static [])
  }

  #[test]
  fn test_case_ident_106() {
    run_tests!("abc", "", ~"", "106", Ok(None), &'static [])
  }

  #[test]
  fn test_case_ident_107() {
    run_tests!("a*", "", ~"", "107", Ok(Some(_)), &'static [])
  }

  #[test]
  fn test_case_ident_108() {
    run_tests!("a|b|c|d|e", "e", ~"e", "108", Ok(Some(_)), &'static [])
  }

  #[test]
  fn test_case_ident_109() {
    run_tests!("(a|b|c|d|e)f", "ef", ~"ef", "109", Ok(Some(_)), &'static ["e"])
  }

  #[test]
  fn test_case_ident_110() {
    run_tests!("abcd*efg", "abcdefg", ~"abcdefg", "110", Ok(Some(_)), &'static [])
  }

  #[test]
  fn test_case_ident_111() {
    run_tests!("ab*", "xabyabbbz", ~"ab", "111", Ok(Some(_)), &'static [])
  }

  #[test]
  fn test_case_ident_112() {
    run_tests!("ab*", "xayabbbz", ~"a", "112", Ok(Some(_)), &'static [])
  }

  #[test]
  fn test_case_ident_113() {
    run_tests!("(ab|cd)e", "abcde", ~"cde", "113", Ok(Some(_)), &'static ["cd"])
  }

  #[test]
  fn test_case_ident_114() {
    run_tests!("[abhgefdc]ij", "hij", ~"hij", "114", Ok(Some(_)), &'static [])
  }

  #[test]
  fn test_case_ident_115() {
    run_tests!("^(ab|cd)e", "abcde", ~"", "115", Ok(None), &'static [])
  }

  #[test]
  fn test_case_ident_116() {
    run_tests!("(abc|)ef", "abcdef", ~"ef", "116", Ok(Some(_)), &'static [""])
  }

  #[test]
  fn test_case_ident_117() {
    run_tests!("(a|b)c*d", "abcd", ~"bcd", "117", Ok(Some(_)), &'static ["b"])
  }

  #[test]
  fn test_case_ident_118() {
    run_tests!("(ab|ab*)bc", "abc", ~"abc", "118", Ok(Some(_)), &'static ["a"])
  }

  #[test]
  fn test_case_ident_119() {
    run_tests!("a([bc]*)c*", "abc", ~"abc", "119", Ok(Some(_)), &'static ["bc"])
  }

  #[test]
  fn test_case_ident_120() {
    run_tests!("a([bc]*)(c*d)", "abcd", ~"abcd", "120", Ok(Some(_)), &'static ["bc", "d"])
  }

  #[test]
  fn test_case_ident_121() {
    run_tests!("a([bc]+)(c*d)", "abcd", ~"abcd", "121", Ok(Some(_)), &'static ["bc", "d"])
  }

  #[test]
  fn test_case_ident_122() {
    run_tests!("a([bc]*)(c+d)", "abcd", ~"abcd", "122", Ok(Some(_)), &'static ["b", "cd"])
  }

  #[test]
  fn test_case_ident_123() {
    run_tests!("a[bcd]*dcdcde", "adcdcde", ~"adcdcde", "123", Ok(Some(_)), &'static [])
  }

  #[test]
  fn test_case_ident_124() {
    run_tests!("a[bcd]+dcdcde", "adcdcde", ~"", "124", Ok(None), &'static [])
  }

  #[test]
  fn test_case_ident_125() {
    run_tests!("(ab|a)b*c", "abc", ~"abc", "125", Ok(Some(_)), &'static ["ab"])
  }

  #[test]
  fn test_case_ident_126() {
    run_tests!("((a)(b)c)(d)", "abcd", ~"abcd", "126", Ok(Some(_)), &'static ["abc", "a", "b", "d"])
  }

  #[test]
  fn test_case_ident_127() {
    run_tests!("[a-zA-Z_][a-zA-Z0-9_]*", "alpha", ~"alpha", "127", Ok(Some(_)), &'static [])
  }

  #[test]
  fn test_case_ident_128() {
    run_tests!("^a(bc+|b[eh])g|.h$", "abh", ~"bh", "128", Ok(Some(_)), &'static [""])
  }

  #[test]
  fn test_case_ident_129() {
    run_tests!("(bc+d$|ef*g.|h?i(j|k))", "effgz", ~"effgz", "129", Ok(Some(_)), &'static ["effgz", ""])
  }

  #[test]
  fn test_case_ident_130() {
    run_tests!("(bc+d$|ef*g.|h?i(j|k))", "ij", ~"ij", "130", Ok(Some(_)), &'static ["ij", "j"])
  }

  #[test]
  fn test_case_ident_131() {
    run_tests!("(bc+d$|ef*g.|h?i(j|k))", "effg", ~"", "131", Ok(None), &'static [])
  }

  #[test]
  fn test_case_ident_132() {
    run_tests!("(bc+d$|ef*g.|h?i(j|k))", "bcdd", ~"", "132", Ok(None), &'static [])
  }

  #[test]
  fn test_case_ident_133() {
    run_tests!("(bc+d$|ef*g.|h?i(j|k))", "reffgz", ~"effgz", "133", Ok(Some(_)), &'static ["effgz", ""])
  }

  #[test]
  fn test_case_ident_134() {
    run_tests!("(((((((((a)))))))))", "a", ~"a", "134", Ok(Some(_)), &'static ["a", "a", "a", "a", "a", "a", "a", "a", "a"])
  }

  #[test]
  fn test_case_ident_135() {
    run_tests!("multiple words of text", "uh-uh", ~"", "135", Ok(None), &'static [])
  }

  #[test]
  fn test_case_ident_136() {
    run_tests!("multiple words", "multiple words, yeah", ~"multiple words", "136", Ok(Some(_)), &'static [])
  }

  #[test]
  fn test_case_ident_137() {
    run_tests!("(.*)c(.*)", "abcde", ~"abcde", "137", Ok(Some(_)), &'static ["ab", "de"])
  }

  #[test]
  fn test_case_ident_138() {
    run_tests!("\\((.*), (.*)\\)", "(a, b)", ~"(a, b)", "138", Ok(Some(_)), &'static ["a", "b"])
  }

  #[test]
  fn test_case_ident_139() {
    run_tests!("[k]", "ab", ~"", "139", Ok(None), &'static [])
  }

  #[test]
  fn test_case_ident_140() {
    run_tests!("a[-]?c", "ac", ~"ac", "140", Ok(Some(_)), &'static [])
  }

  #[test]
  fn test_case_ident_141() {
    run_tests!("^(.+)?B", "AB", ~"AB", "141", Ok(Some(_)), &'static ["A"])
  }

  #[test]
  fn test_case_ident_142() {
    run_tests!("a{5}", "aaaaa", ~"aaaaa", "142", Ok(Some(_)), &'static [])
  }

  #[test]
  fn test_case_ident_143() {
    run_tests!("a{5,}", "aaaaaaa", ~"aaaaaaa", "143", Ok(Some(_)), &'static [])
  }

  #[test]
  fn test_case_ident_144() {
    run_tests!("a{5,7}", "aaaaaa", ~"aaaaaa", "144", Ok(Some(_)), &'static [])
  }

  #[test]
  fn test_case_ident_145() {
    run_tests!("a{5,}", "aaaa", ~"", "145", Ok(None), &'static [])
  }

}
