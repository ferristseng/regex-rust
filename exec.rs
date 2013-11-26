use compile::{Instruction};

struct Prog {
  strat: ~ExecStrategy
}

trait ExecStrategy {
  fn run(&mut self);
}

// the implementation for both PikeVM
// and RecursiveBacktracking strategies here:
// * http://swtch.com/~rsc/regexp/regexp2.html
struct PikeVM {
  input: ~str,
  inst: ~[Instruction],
  pc: uint,
  sp: uint,
}

impl PikeVM {
  fn new(inst: ~[Instruction], input: &str) -> PikeVM {
    PikeVM {
      input: input.to_owned(),
      inst: inst,
      pc: 0,
      sp: 0,
    }
  }
}

impl ExecStrategy for PikeVM {
  fn run(&mut self) {
    // reset
    self.pc = 0;
    self.sp = 0;
  }
}

// ignore this
struct RecursiveBacktracking {
  inst: ~[Instruction]
}

impl ExecStrategy for RecursiveBacktracking {
  fn run(&mut self) {

  }
}
