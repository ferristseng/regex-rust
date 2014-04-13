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

	pub fn split(&self, input: &str) -> ~[~str] { // This is lengthier than it should be; I'll keep working to improve it.
		let mut start_indices = ~[];
		let mut match_lengths = ~[];
		let mut matches: ~[Match] = ~[];
		let mut result: ~[~str] = ~[];
		let len = input.len();
		let strat = PikeVM::new(self.prog, 0); 

		matches = self.find_all(input); // Check whether input contains the regex
		for i in range(0, matches.len()) {
			let m = matches[i].input.slice(matches[i].start, matches[i].end);
			if m.is_empty() {
				result.push(input.to_owned());
				return result;
			}
		}

		let mut start = 0;
		for _ in range(0, len + 1) { // Run starting at each character
				match strat.run(input, start) { // run only matches one thing...
					Some(t) => {
						let nextPos = t.end;
						start_indices.push(start);
						match_lengths.push(t.end - start + 1);
						start = nextPos;
					}
					None => {
						start += 1;
					}
				}
		}

		if start_indices.len() == 1 { // If 1 match
			if start_indices[0] == 0 { // Matched at start of input
				let res = format!("{:s}", input.slice_from(start_indices[0] + match_lengths[0]-1));
				result.push(res);
				return result;
			} else { // At end of input
				let res = format!("{:s}", input.slice_to(start_indices[0]));
				result.push(res);
				return result;
			}
		} 

		for i in range(0, start_indices.len()) { // If more than 1 match
			if i == start_indices.len()-1 { // If reached end of input
				let res = format!("{:s}", input.slice_from(start_indices[i] + match_lengths[i] - 1));
				result.push(res);
			} else {
				let res = format!("{:s}", input.slice(start_indices[i] + match_lengths[i] - 1, start_indices[i+1]));
				result.push(res);
			}
		}
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

	pub fn replace(&self, input: &str, replaceWith: &str) -> ~str {
		match self.replacen(input, replaceWith) {
			(replaced, _) => replaced,
		}
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
					let mat = Match::new(start, t.end, input, t.captures);
					let replStr = self.formReplaceString(mat.clone(), replaceWith);
					replaced = format!("{:s}{:s}{:s}", replaced.slice_to(start), replStr, replaced.slice_from(mat.end));
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

	fn formReplaceString<'a>(&self, mat : Match, replWith : &'a str) -> &'a str{
		let groupEscapeStr = r"\";

		let i = replWith.find_str(groupEscapeStr);
		let mut done = ~"";
		let mut replStr = replWith;
		while i != None {
			let start = i.unwrap();
			done = done + replStr.slice_to(start + 1);
			replStr = replStr.slice_from(start + 1);

			if replStr.len() == 0 {break;}
			let group = replStr.char_at(0)=='g';
			if group {
				if 1 == replStr.len() {
					// error, undefined group
				}
				let delimited = replStr.char_at(1) == '<';
				if delimited {
					let groupEnd = replStr.find('>');
					if groupEnd == None {
						// error, unterminated group name
					}
					let groupName = replStr.slice(2, groupEnd.unwrap());
					let groupMatch = mat.group_by_name(groupName);
					match groupMatch {
						Some(res) => {
							done = done + res;
							replStr = replStr.slice_from(groupEnd.unwrap() + 1);
						}
						None => {
							// error, group name not found
						}
					}
				}
				let valid = (replStr.char_at(1) <= '9' && replStr.char_at(1) >= '0');
				if valid {
					let mut numLength = 1;
					loop {
						if replStr.char_at(1 + numLength) <= '9' && replStr.char_at(1 + numLength) >= '0' {
							numLength = numLength + 1;
						}
						else {
							break;
						}
					}
					let groupNum = from_str::<uint>(replStr.slice(1, numLength + 1));
					let groupMatch = mat.group(groupNum.unwrap());
					match groupMatch {
						Some(res) => {
							done = done + res;
							replStr = replStr.slice_from(numLength + 1);
						}
						None => {
							// error, invalid group number
						}
					}
				}
				else {
					// error, invalid group spec
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
		return replStr;
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

	macro_rules! test_split(
		($re: expr, $input: expr, $expect: expr) => (
			{
				let re = match UncompiledRegexp::new($re) {
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
