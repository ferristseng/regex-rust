use std::vec::with_capacity;
use std::util::swap;
use compile::Instruction;
use compile::{InstLiteral, InstRange, InstMatch, InstJump, 
  InstCaptureStart, InstCaptureEnd, InstSplit, InstNoop};
use error::ExecError::*;

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
    match self.strat.run() {
      ExecMatchFound => {
        println("[FOUND]");
      }
      _ => println("[NOT FOUND]")
    } 
  }
}

// Prog expects an ExecStrategy to run...
// this should be able to take compiled 
// instructions and execute them (see compile.rs)

trait ExecStrategy {
  fn run(&mut self) -> ExecCode;
}

// the implementation for both PikeVM
// and RecursiveBacktracking strategies are 
// outlined here:
// -> http://swtch.com/~rsc/regexp/regexp2.html

struct Thread {
  pc: uint, // some index of an instruction
  sp: uint, // index of a char in the input
  captures: ~[(uint, uint)]
}

impl Thread {
  fn new(pc: uint, sp: uint) -> Thread {
    Thread { 
      pc: pc, 
      sp: sp,
      captures: ~[] 
    }
  }
}

struct PikeVM {
  input: ~str,
  inst: ~[Instruction],
  len: uint
}

impl PikeVM {
  fn new(inst: ~[Instruction], input: &str) -> PikeVM {
    // \x03 is an end of string indicator. it resolves issues
    // the program reaches the end of the string, and still
    // needs to perform instructions
    let len = inst.len();
    PikeVM {
      input: input.to_owned().append("\x03"),
      inst: inst,
      len: len 
    }
  }
}

impl ExecStrategy for PikeVM {
  fn run(&mut self) -> ExecCode {
    let mut sp = 0;

    let mut clist: ~[Thread] = with_capacity(self.len);
    let mut nlist: ~[Thread] = with_capacity(self.len);
    
    clist.push(Thread::new(0, sp));

    for c in self.input.iter() {
      println(c.to_str());

      let mut i = 0;
      let mut num = clist.len();

      while (i < num) {
        println(fmt!("RUNNING INST %?", clist[i]));

        let pc = clist[i].pc;

        match self.inst[pc].op {
          InstLiteral(m) => {
            if (c == m) {
              println(fmt!("c(%c) | m(%c)", c, m));
              nlist.push(Thread::new(pc + 1, clist[i].sp));
            }
          }
          InstRange(start, end) => {
            if (c >= start && c <= end) {
              nlist.push(Thread::new(pc + 1, sp));
            }
          }
          InstMatch => {
            return ExecMatchFound;
          }
          InstJump(addr) => {
            println("JMP");
            clist.push(Thread::new(addr, clist[i].sp));
          }
          InstCaptureStart => {
            clist.push(Thread::new(pc + 1, clist[i].sp));
          }
          InstCaptureEnd => {
            clist.push(Thread::new(pc + 1, clist[i].sp));
          }
          InstSplit(laddr, raddr) => {
            clist.push(Thread::new(laddr, clist[i].sp));
            clist.push(Thread::new(raddr, clist[i].sp));
          }
          InstNoop => { } // continue
        }

        println(fmt!("BEFORE %u", i));

        i += 1;
        num = clist.len();

        println(fmt!("clist: %?", clist));
        println(fmt!("nlist: %?", nlist));
      }

      println("SWAPPING");
      swap(&mut clist, &mut nlist);
      println("AFTER");
      println(fmt!("clist: %?", clist));
      println(fmt!("nlist: %?", nlist));
      nlist.clear();

      sp += 1;
    }

    ExecMatchNotFound
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
  fn run(&mut self) -> ExecCode {
    ExecMatchFound
  }
}
