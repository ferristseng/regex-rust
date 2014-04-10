# Test generator
import re
from cases import *
from datetime import datetime

FILE = open('benchmark/benches/rust_bench.rs', 'w')

OUTPUT = """
// This is an auto-generated test file
// Generated by benchmark/rust_bench_generator.py
//
// Last Modified: %s

extern crate rustre;


  fn execute (reg: ~str, text: ~str) -> () {
    let re = match rustre::regexp::UncompiledRegexp::new(reg) {
      Ok(regex) => regex,
      Err(e) => fail!(e)
    };

    re.search(text);
  }


  fn main() {
    for i in range(0, %s) {
      %s

    }
  }

"""

TEST_FN = \
"""
      execute(~\"%s\", ~\"%s\");"""

if __name__ == "__main__":
  date = datetime.today().strftime("%B %d %Y %I:%M%p")
  buf = ""

  for (i, test) in enumerate(TESTS):
    buf += TEST_FN % (test[0], test[1])

  FILE.write(OUTPUT % (date, NO_LOOPS, buf))


  print("Successfully generated test file: benchmark/benches/rust_bench.rs")
