use std::vec::with_capacity;
use std::util::swap;
use compile::Instruction;
use compile::{InstLiteral, InstRange, InstMatch, InstJump, 
  InstCaptureStart, InstCaptureEnd, InstSplit, InstNoop};

// object containing implementation
// details for executing compiled 
// instructions

struct Prog {
  strat: ~ExecStrategy
}

impl Prog {
  pub fn new(inst: ~[Instruction]) -> Prog {
    Prog::new_with_pike_vm(inst)
  }
  pub fn new_with_pike_vm(inst: ~[Instruction]) -> Prog {
    let strat = ~PikeVM::new(inst) as ~ExecStrategy;
    Prog {
      strat: strat
   }
  }
  pub fn new_with_recursive(inst: ~[Instruction]) -> Prog {
    let strat = ~RecursiveBacktracking::new(inst) as ~ExecStrategy;
    Prog {
      strat: strat
    }
  }
}

impl Prog {
  pub fn run(&self, input: &str) {
    match self.strat.run(input) {
      Some(t) => {
        println(fmt!("[FOUND %u]", t.sp));
        println(input.slice_to(t.sp));
      }
      None => println("[NOT FOUND]")
    } 
  }
}

// Prog expects an ExecStrategy to run...
// this should be able to take compiled 
// instructions and execute them (see compile.rs)

trait ExecStrategy {
  fn run(&self, input: &str) -> Option<Thread>;
}

// the implementation for both PikeVM
// and RecursiveBacktracking strategies are 
// outlined here:
// -> http://swtch.com/~rsc/regexp/regexp2.html

struct Thread {
  pc: uint, // some index of an instruction
  sp: uint  // index of a char in the input
}

impl Thread {
  fn new(pc: uint, sp: uint) -> Thread {
    Thread { 
      pc: pc, 
      sp: sp
    }
  }
}

impl ToStr for Thread {
  fn to_str(&self) -> ~str {
    fmt!("<Thread pc: %u, sp: %u>", self.pc, self.sp)
  }
}

struct PikeVM {
  priv inst: ~[Instruction],
  priv len: uint
}

impl PikeVM {
  fn new(inst: ~[Instruction]) -> PikeVM {
    let len = inst.len();
    PikeVM {
      inst: inst,
      len: len 
    }
  }
}

impl ExecStrategy for PikeVM {
  fn run(&self, input: &str) -> Option<Thread> {
    // \x03 is an end of string indicator. it resolves issues
    // the program reaches the end of the string, and still
    // needs to perform instructions
    let input = input.to_owned().append("\x03");

    // setup
    let mut sp = 0;
    let mut found = None;

    let mut clist: ~[Thread] = with_capacity(self.len);
    let mut nlist: ~[Thread] = with_capacity(self.len);
    
    clist.push(Thread::new(0, sp));

    for c in input.iter() {
      //println(c.to_str());

      let mut i = 0;
      let mut num = clist.len();

      while (i < num) {
        //println(fmt!("RUNNING INST %?", clist[i]));

        let pc = clist[i].pc;

        match self.inst[pc].op {
          InstLiteral(m) => {
            if (c == m) {
              //println(fmt!("c: (%c) == m: (%c)", c, m));
              nlist.push(Thread::new(pc + 1, clist[i].sp));
            }
          }
          InstRange(start, end) => {
            if (c >= start && c <= end) {
              //println(fmt!("c: (%c) within (%c-%c)", c, start, end));
              nlist.push(Thread::new(pc + 1, sp));
            }
          }
          InstMatch => {
            clist[i].sp = sp;
            found = Some(clist[i]); 
          }
          InstJump(addr) => {
            //println("JMP");
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
          InstNoop => { 
            clist.push(Thread::new(pc + 1, clist[i].sp));
          } // continue
        }

        //println(fmt!("BEFORE %u", i));

        i += 1;
        num = clist.len();

        //println(fmt!("clist: %?", clist));
        //println(fmt!("nlist: %?", nlist));
      }

      swap(&mut clist, &mut nlist);
      /*
      println("SWAPPING");
      println("AFTER");
      println(fmt!("clist: %?", clist));
      println(fmt!("nlist: %?", nlist));
      */
      nlist.clear();
      
      sp += c.len_utf8_bytes();
    }

    found 
  }
}

// ignore this
// for now...
// this implementation is feature complete, but slow.
// we can execute all the instructions we currently support
// in PikeVM

struct RecursiveBacktracking {
  priv inst: ~[Instruction],
  priv len: uint
}

impl RecursiveBacktracking {
  fn new(inst: ~[Instruction]) -> RecursiveBacktracking {
    let len = inst.len();
    RecursiveBacktracking {
      inst: inst,
      len: len
    }
  }
}

impl ExecStrategy for RecursiveBacktracking {
  fn run(&self, input: &str) -> Option<Thread> {
    let input = input.to_owned().append("\x03");

    None
  }
}
