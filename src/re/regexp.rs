use exec::{ExecStrategy, PikeVM};
use compile::Instruction;
use result::Match;
use parse::{parse, ParseFlags};
use compile::compile_recursive;
use error::ParseError::*;

/// Uncompiled regular expression.
pub struct Regexp {
	prog: ~[Instruction]
}

// Error enum for replace function
pub enum ReplStringSpecError {
	UndefinedGroupName,
	GroupNumberOutOfBounds,
	MalformedGroupSpec
}

impl ReplStringSpecError {
	fn getName(err : ReplStringSpecError) -> ~str{
		match err {
			UndefinedGroupName => ~"UndefinedGroupName",
			GroupNumberOutOfBounds => ~"GroupNumberOutOfBounds",
			MalformedGroupSpec => ~"MalformedGroupSpec"
		}
	}
}

/// Constructors
impl Regexp {
	pub fn new(s: &str, f: &mut ParseFlags) -> Result<Regexp, ParseCode> {
		match parse(s, f) {
			Ok(ref expr) => {
				let prog = compile_recursive(expr);
				Ok(Regexp { prog: prog })
			}
			Err(e) => Err(e)
		}
	}
}

/// TODO:
/// The API needs some work.
/// Allow for other implementations to be used?
impl Regexp {
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

	pub fn split(&self, input: &str) -> ~[~str] { // This is lengthier than it should be; I'll keep working to improve it.
		let mut result: ~[~str] = ~[];
		let mut cur_start = 0;
		let mut end = input.len();

		let matches = self.find_all(input); // Check whether input contains the regex
		for m in matches.iter() {
			result.push(input.slice(cur_start, m.start).to_owned());
			cur_start = m.end;
		}
		result.push(input.slice(cur_start, end).to_owned());

		return result;
	}

	pub fn find_all(&self, input: &str) -> ~[Match] {
		let mut matches : ~[Match] = ~[];

		let len = input.len();
		let strat = PikeVM::new(self.prog, 0);

		let mut start = 0;
		for _ in range(0, len + 1) {	// Run starting at each character
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

	pub fn replace(&self, input: &str, replaceWith: &str) -> Result<~str, ReplStringSpecError> {
		match self.replacen(input, replaceWith) {
			Ok((replaced, _)) => Ok(replaced),
			Err(error) => Err(error)
		}
	}

	pub fn replacen(&self, input: &str, replaceWith: &str) -> Result<(~str, uint), ReplStringSpecError> {
		let strat = PikeVM::new(self.prog, 0);
		let mut replaced = input.to_owned();
		let mut start = 0;
		let emptyPatternAdd = if self.prog.len()==1 {1} else {0};
		let mut repCount = 0;

		while start <= replaced.len(){
			match strat.run(replaced, start) {
				Some(t) => {
					let mat = Match::new(start, t.end, replaced, t.captures);
					match self.formReplaceString(mat.clone(), replaceWith) {
						Ok(replStr) => {
							replaced = format!("{:s}{:s}{:s}", replaced.slice_to(start), replStr, replaced.slice_from(mat.end));
							start += replStr.len() + emptyPatternAdd;
							repCount += 1;
						}
						Err(error) => { return Err(error); }
					}
				}
				None => {
					start += 1;
				}
			}
		}

		Ok((replaced, repCount))
	}

	fn formReplaceString(&self, mat : Match, replWith : &str) -> Result<~str, ReplStringSpecError>{
		let groupEscapeStr = r"\";

		let mut i = replWith.find_str(groupEscapeStr);
		let mut done = ~"";
		let mut replStr = replWith;
		while i != None {
			let start = i.unwrap();
			done = done + replStr.slice_to(start);
			replStr = replStr.slice_from(start + 1);

			if replStr.len() == 0 {break;}
			let group = replStr.char_at(0)=='g';
			if group {
				if 1 == replStr.len() {
					// error, undefined group
					return Err(MalformedGroupSpec);
				}
				let delimited = replStr.char_at(1) == '<';
				if delimited {
					let groupEnd = replStr.find('>');
					if groupEnd == None {
						// error, unterminated group name
						return Err(MalformedGroupSpec);
					}
					let groupName = replStr.slice(2, groupEnd.unwrap());
					let groupNameNum = from_str::<uint>(groupName);
					let groupMatch = match groupNameNum {
							Some(num) => mat.group(num),
							None => mat.group_by_name(groupName)
					};
					match groupMatch {
						Some(res) => {
							done = done + res;
							replStr = replStr.slice_from(groupEnd.unwrap() + 1);
						}
						None => {
							// error, group name not found
							return Err(UndefinedGroupName)
						}
					}
				}
				else {
					// error, invalid group spec
				}
			}
			else {
				let valid = replStr.char_at(0) <= '9' && replStr.char_at(0) >= '0';
				if valid {
					let mut numLength = 1;
					loop {
						if numLength < replStr.len() && replStr.char_at(numLength) <= '9' && replStr.char_at(numLength) >= '0' {
							numLength = numLength + 1;
						}
						else {
							break;
						}
					}
					let groupNum = from_str::<uint>(replStr.slice_to(numLength));
					let groupMatch = mat.group(groupNum.unwrap());
					match groupMatch {
						Some(res) => {
							done = done + res;
							replStr = replStr.slice_from(numLength);
						}
						None => {
							// error, invalid group number
							return Err(GroupNumberOutOfBounds);
						}
					}
				}
				else {
					done = done + replStr.slice_to(1);
					if replStr.len() > 1 {
						replStr = replStr.slice_from(1);
					}
					else {
						break;
					}
				}
			}
			i = replStr.find_str(groupEscapeStr);
		}
		done = done + replStr;
		return Ok(done);
	}
}

#[cfg(test)]
mod library_functions_test {
	use super::*;
	use super::ReplStringSpecError;
	use parse::ParseFlags;

	fn test_replace (re: &str, input: &str, flags: ~str, replaceWith: &str, expect: Result<~str, ReplStringSpecError>) {
		let f = &mut ParseFlags::new();
		f.setFlags(flags);
		let reg = match Regexp::new(re, f) {
			Ok(regex) => regex,
			Err(e) => fail!(e)
		};
		let result = reg.replace(input, replaceWith);
		let printResult = match result {Ok(exp)=>exp, Err(error)=>ReplStringSpecError::getName(error)};
		let printExpect = match expect {Ok(exp)=>exp, Err(error)=>ReplStringSpecError::getName(error)};
		if printResult != printExpect {
			fail!(format!("Replacing {:s} in {:s} with {:s} yielded {:s}, not expected result of {:s}\n", re, input, replaceWith, printResult, printExpect));
		}
	}

	fn test_replacen (re: &str, input: &str, flags: ~str, replaceWith: &str, expect: Result<(~str, uint), ReplStringSpecError>) {
		let f = &mut ParseFlags::new();
		f.setFlags(flags);
		let reg = match Regexp::new(re, f) {
			Ok(regex) => regex,
			Err(e) => fail!(e)
		};
		let result = reg.replacen(input, replaceWith);
		let printResult = match result {Ok((exp, num))=>(exp, num), Err(error)=>(ReplStringSpecError::getName(error), 0)};
		let printExpect = match expect {Ok((exp, num))=>(exp, num), Err(error)=>(ReplStringSpecError::getName(error), 0)};
		if printResult != printExpect {
			match printResult {
				(resS, resN) => {
					match printExpect {
						(expS, expN) => fail!(format!("Replacing {:s} in {:s} with {:s} yielded {:s} with {:u} replaces, not expected result of {:s} with {:u} replaces\n",
							re, input, replaceWith, resS, resN, expS, expN))
					}
				}
			}
		}
	}

	macro_rules! test_find_all(
		($re: expr, $input: expr, $flags: expr, $expect: expr) => (
			{
				let f = &mut ParseFlags::new();
				f.setFlags($flags);
				let re = match Regexp::new($re, f) {
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

	macro_rules! test_split(
		($re: expr, $input: expr, $expect: expr) => (
			{
				let re = match Regexp::new($re, &mut ParseFlags::new()) {
					Ok(regex) => regex,
					Err(e) => fail!(e)
				};
				let result = re.split($input);
				let mut i = 0;
				for &item in $expect.iter() {
					if i >= result.len() {
						fail!(format!("Results list only has {:u} elements, expected to have {:u}\n", i, $expect.len()));
					}
					let res = result[i].clone();
					if res != item.to_owned() {
						fail!(format!("Split on regexp '{:s}' yielded '{:s}' at element {:u} of results list, not expected result of '{:s}'\n", $re, res, i, item.clone()));
					}
					i = i + 1;
				}
			}
		);
	)

	#[test]
	fn test_replace_01() {
		test_replace("a*ba*", "abaaacaabaaaccdab", ~"", "", Ok(~"cccd"));
	}

	#[test]
	fn test_replace_02() {
		test_replace("a*ba{1,}", "abaaacaabaaacca", ~"", "", Ok(~"ccca"));
	}

	#[test]
	fn test_replace_03() {
		test_replace("a*ba{1,}", "abaaacaabaaacca", ~"", "aba", Ok(~"abacabacca"));
	}

	#[test]
	fn test_replace_04() {
		test_replace("a", "aaaaaaaaaaaa", ~"", "b", Ok(~"bbbbbbbbbbbb"));
	}

	#[test]
	fn test_replace_05() {
		test_replace("a{1,}", "aaaaaaaaaaaa", ~"", "b", Ok(~"b"));
	}

	#[test]
	fn test_replace_06() {
		test_replace("a{1,}", "aaaaaaaaaaaa", ~"", "", Ok(~""));
	}

	#[test]
	fn test_replace_07() {
		test_replace("", "aaaa", ~"", "b", Ok(~"babababab"));
	}

	#[test]
	fn test_replace_08() {
		test_replace("a?bab", "abababab", ~"", "c", Ok(~"cc"));
	}

	#[test]
	fn test_replace_09() {
		test_replace("a", "aa", ~"", "ccc", Ok(~"cccccc"));
	}

	#[test]
	fn test_replace_10() {
		test_replace("b", "aa", ~"", "ccc", Ok(~"aa"));
	}

	#[test]
	fn test_replace_11() {
		test_replace("(abab)c", "ababcababc", ~"", r"\1", Ok(~"abababab"));
	}

	#[test]
	fn test_replace_12() {
		test_replace("(a)(b)(c)(d)(e)(f)(g)(h)(i)(j)(k)", "abcdefghijk", ~"", r"\11", Ok(~"k"));
	}

	#[test]
	fn test_replace_13() {
		test_replace("(a)(b)(c)(d)(e)(f)(g)(h)(i)(j)(k)", "abcdefghijk", ~"", r"\11win", Ok(~"kwin"));
	}

	#[test]
	fn test_replace_14() {
		test_replace("(a)(b)(c)(d)(e)(f)(g)(h)(i)(j)(k)", "", ~"", r"\11win", Ok(~""));
	}

	#[test]
	fn test_replace_15() {
		test_replace("(?P<named>a)(b)(c)(d)(e)(f)(g)(h)(i)(j)(k)", "abcdefghijk", ~"", r"\g<named>", Ok(~"a"));
	}

	#[test]
	fn test_replace_16() {
		test_replace("(?P<named>a)(b)(c)(d)(e)(f)(g)(h)(i)(j)(k)", "abcdefghijk", ~"", r"\g<1>", Ok(~"a"));
	}

	#[test]
	fn test_replace_17() {
		test_replace("(a)(b)(c)(d)(e)(f)(g)(h)(i)(j)(k)", "abcdefghijk", ~"", r"\g<2>", Ok(~"b"));
	}

	#[test]
	fn test_replace_18() {
		test_replace("(a)(b)c", "abc", ~"", r"\\g<1>\g<1>", Ok(~r"\g<1>a"));
	}

	#[test]
	fn test_replace_19() {
		test_replace("(a)(b)c", "abc", ~"", r"\3\g<1>", Err(super::GroupNumberOutOfBounds));
	}

	#[test]
	fn test_replace_20() {
		test_replace("(a)(b)c", "abc", ~"", r"\g<name>", Err(super::UndefinedGroupName));
	}

	#[test]
	fn test_replace_21() {
		test_replace("(a)(b)c", "abc", ~"", r"\g", Err(super::MalformedGroupSpec));
	}

	#[test]
	fn test_replace_22() {
		test_replace("(a)(b)c", "abc", ~"", r"\g<asda", Err(super::MalformedGroupSpec));
	}

	#[test]
	fn test_replacen_01() {
		test_replacen("a*ba*", "abaaacaabaaaccdab", ~"", "", Ok((~"cccd", 3)));
	}

	#[test]
	fn test_replacen_02() {
		test_replacen("a*ba{1,}", "abaaacaabaaacca", ~"", "", Ok((~"ccca", 2)));
	}

	#[test]
	fn test_replacen_03() {
		test_replacen("a*ba{1,}", "abaaacaabaaacca", ~"", "aba", Ok((~"abacabacca", 2)));
	}

	#[test]
	fn test_replacen_04() {
		test_replacen("a", "aaaaaaaaaaaa", ~"", "b", Ok((~"bbbbbbbbbbbb", 12)));
	}

	#[test]
	fn test_replacen_05() {
		test_replacen("a{1,}", "aaaaaaaaaaaa", ~"", "b", Ok((~"b", 1)));
	}

	#[test]
	fn test_replacen_06() {
		test_replacen("a{1,}", "aaaaaaaaaaaa", ~"", "", Ok((~"", 1)));
	}

	#[test]
	fn test_replacen_07() {
		test_replacen("", "aaaa", ~"", "b", Ok((~"babababab", 5)));
	}

	#[test]
	fn test_replacen_08() {
		test_replacen("a?bab", "abababab", ~"", "c", Ok((~"cc", 2)));
	}

	#[test]
	fn test_replacen_09() {
		test_replacen("a", "aa", ~"", "ccc", Ok((~"cccccc", 2)));
	}

	#[test]
	fn test_replacen_10() {
		test_replacen("b", "aa", ~"", "ccc", Ok((~"aa", 0)));
	}

	#[test]
	fn test_find_all_01() {
		test_find_all!("a*ba*", "abaaacaabaaaccdab", ~"", &["abaaa", "aabaaa", "ab"]);
	}

	#[test]
	fn test_find_all_02() {
		test_find_all!("a*ba{1,}", "abaaacaabaaaccab", ~"", &["abaaa", "aabaaa"]);
	}

	#[test]
	fn test_find_all_03() {
		test_find_all!("a*ba{1,}", "abaaacaabaaaccab", ~"", &["abaaa", "aabaaa"]);
	}

	#[test]
	fn test_find_all_04() {
		test_find_all!("a", "aaaaaaaaaaaa", ~"", &["a", "a", "a", "a", "a", "a", "a",
			"a", "a", "a", "a", "a"]);
	}

	#[test]
	fn test_find_all_05() {
		test_find_all!("a{1,}", "aaaaaaaaaaaa", ~"", &["aaaaaaaaaaaa"]);
	}

	#[test]
	fn test_find_all_06() {
		test_find_all!("a{1,}", "aaabaaaabaaa", ~"", &["aaa", "aaaa", "aaa"]);
	}

	#[test]
	fn test_find_all_07() {
		test_find_all!("", "aaaa", ~"", &["", "", "", ""]);
	}

	#[test]
	fn test_find_all_08() {
		test_find_all!("a?bab", "ababababbab", ~"", &["abab", "abab", "bab"]);
	}

	#[test]
	fn test_find_all_09() {
		test_find_all!("a", "aa", ~"", &["a", "a"]);
	}

	#[test]
	fn test_find_all_10() {
		test_find_all!("a*b*c*d*", "abcdbabcdabcbababcbdabcbdaabbbccccddddd", ~"", &["abcd",
			"b", "abcd", "abc", "b", "ab", "abc", "bd", "abc", "bd", "aabbbccccddddd"]);
	}

	#[test]
	fn test_split_01() {
		test_split!("x*", "abab", &["abab"]);
	}

	#[test]
	fn test_split_02() {
		test_split!("c", "abc", &["ab"]);
	}

	#[test]
	fn test_split_03() {
		test_split!("c", "cab", &["ab"]);
	}

	#[test]
	fn test_split_04() {
		test_split!("a{1,}", "aaaaaabc", &["bc"]);
	}

	#[test]
	fn test_split_05() {
		test_split!("a{1,}", "aaaaaabaab", &["b", "b"]);
	}

	#[test]
	fn test_split_06() {
		test_split!("a{1,}", "aaaaaabaaaa", &["b"]);
	}


}


#[cfg(test)]
mod tests {
	use super::*;
	use parse::ParseFlags;

	#[test]
	fn parse_alternation_ok_test() {
		assert!(Regexp::new("a|b", &mut ParseFlags::new()).is_ok());
	}

	#[test]
	fn parse_concatenation_ok_test() {
		assert!(Regexp::new("a(bc)d", &mut ParseFlags::new()).is_ok());
	}

	#[test]
	fn parse_char_class_ok_test() {
		assert!(Regexp::new("[a-zABC!@#]]]", &mut ParseFlags::new()).is_ok());
	}

	#[test]
	fn parse_capture_ok_test() {
		assert!(Regexp::new("(hel(ABC)ok)", &mut ParseFlags::new()).is_ok());
	}

	#[test]
	fn parse_capture_fail_test() {
		assert!(Regexp::new("(hel(ABC)ok", &mut ParseFlags::new()).is_err());
	}

	#[test]
	fn search_group_fetch() {
		match Regexp::new("(?P<hello>d)", &mut ParseFlags::new()) {
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
