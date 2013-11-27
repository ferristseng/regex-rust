use std::vec::with_capacity;
use std::ptr::{replace_ptr, to_mut_unsafe_ptr};
use compile::{Instruction, InstOpCode};
use compile::{InstLiteral, InstRange, InstMatch, InstJump, 
  InstCaptureStart, InstCaptureEnd, InstSplit, InstNoop};

// object containing implementation
// details for executing compiled 
// instructions

struct Prog {
  strat: ~ExecStrategy
}

impl Prog {
  pub fn new(inst: ~[Instruction], input: &str) -> Prog {
    Prog::new_with_pike_vm(inst, input)
  }
  pub fn new_with_pike_vm(inst: ~[Instruction], input: &str) -> Prog {
    let strat = ~PikeVM::new(inst, input) as ~ExecStrategy;
    Prog {
      strat: strat
   }
  }
  pub fn new_with_recursive(inst: ~[Instruction], input: &str) -> Prog {
    let strat = ~RecursiveBacktracking::new(inst, input) as ~ExecStrategy;
    Prog {
      strat: strat
    }
  }
}

impl Prog {
  pub fn run(&mut self) {
    self.strat.run();
  }
}

// Prog expects an ExecStrategy to run...
// this should be able to take compiled 
// instructions and execute them (see compile.rs)

trait ExecStrategy {
  fn run(&mut self);
}

// the implementation for both PikeVM
// and RecursiveBacktracking strategies are 
// outlined here:
// -> http://swtch.com/~rsc/regexp/regexp2.html

struct Thread {
  pc: uint, // some index of an instruction
  captures: ~[(uint, uint)]
}

impl Thread {
  fn new(pc: uint) -> Thread {
    Thread { 
      pc: pc, 
      captures: ~[] 
    }
  }
}

struct PikeVM {
  input: ~str,
  inst: ~[Instruction],
  pc: uint,
  sp: uint,
  len: uint
}

impl PikeVM {
  fn new(inst: ~[Instruction], input: &str) -> PikeVM {
    let len = inst.len();
    PikeVM {
      input: input.to_owned(),
      inst: inst,
      pc: 0,
      sp: 0,
      len: len 
    }
  }
}

impl ExecStrategy for PikeVM {
  fn run(&mut self) {
    // reset
    self.pc = 0;
    self.sp = 0;

    let mut clist: ~[Thread] = with_capacity(self.len);
    let mut nlist: ~[Thread] = with_capacity(self.len);
    
    clist.push(Thread::new(self.pc));

    for c in self.input.iter() {
      let num_entries = clist.len();

      println(c.to_str());

      for i in range(0, num_entries) {
        self.pc = clist[i].pc;

        match self.inst[self.pc].op {
          InstLiteral(m) => {
            if c != m {
              return;
            }
            nlist.push(Thread::new(self.pc + 1));
          }
          InstRange(s, e) => {

          }
          InstMatch => {
            clist[i].captures.push((i, i));
            return;
          }
          InstJump(i) => {

          }
          InstCaptureStart => {

          }
          InstCaptureEnd => {

          }
          InstSplit(l, r) => {
            clist.push(Thread::new(l));
            clist.push(Thread::new(r));
          }
          InstNoop => { }
        }
      }
      // this should be safe because nlist and clist 
      // are vectors of the same size (their size shouldn't
      // change as well).
      println(fmt!("nlist: %?", nlist));
      unsafe {
        replace_ptr(
          to_mut_unsafe_ptr(&mut clist), 
          nlist);
      }
      println(fmt!("clist: %?", clist));
      nlist = ~[];
    }
  }
}

// ignore this
// for now...
// this implementation is feature complete, but slow.
// we can execute all the instructions we currently support
// in PikeVM

struct RecursiveBacktracking {
  input: ~str,
  inst: ~[Instruction]
}

impl RecursiveBacktracking {
  fn new(inst: ~[Instruction], input: &str) -> RecursiveBacktracking {
    RecursiveBacktracking {
      input: input.to_owned(),
      inst: inst
    }
  }
}

impl ExecStrategy for RecursiveBacktracking {
  fn run(&mut self) {

  }
}
