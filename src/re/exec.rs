use std::vec::with_capacity;
use std::util::swap;
use compile::Instruction;
use compile::{InstLiteral, InstRange, InstMatch, InstJump, 
  InstCaptureStart, InstCaptureEnd, InstSplit, InstDotAll, 
  InstAssertStart, InstAssertEnd, InstWordBoundary,
  InstNonWordBoundary, InstNoop};
use result::{Match, CapturingGroup};

// object containing implementation
// details for executing compiled 
// instructions

pub struct Prog {
  priv strat: ~ExecStrategy,
  priv ncaps: uint
}

impl Prog {
  pub fn new(inst: ~[Instruction], ncaps: uint) -> Prog {
    Prog::new_with_pike_vm(inst, ncaps)
  }
  pub fn new_with_pike_vm(inst: ~[Instruction], ncaps: uint) -> Prog {
    let strat = ~PikeVM::new(inst, ncaps) as ~ExecStrategy;
    Prog {
      strat: strat,
      ncaps: ncaps
   }
  }
  pub fn new_with_recursive(inst: ~[Instruction], ncaps: uint) -> Prog {
    let strat = ~RecursiveBacktracking::new(inst, ncaps) as ~ExecStrategy;
    Prog {
      strat: strat,
      ncaps: ncaps
    }
  }
}

impl Prog {
  pub fn run(&self, input: &str, start: uint) -> Option<Match> {    
    match self.strat.run(input, start) {
      Some(t) => {
        Some(Match::new(start, t.end, input, t.captures))
      }
      None => None 
    } 
  }
}

// Prog expects an ExecStrategy to run...
// this should be able to take compiled 
// instructions and execute them (see compile.rs)

trait ExecStrategy {
  fn run(&self, input: &str, start_index: uint) -> Option<Thread>;
}

// the implementation for both PikeVM
// and RecursiveBacktracking strategies are 
// outlined here:
// -> http://swtch.com/~rsc/regexp/regexp2.html

#[deriving(Clone)]
struct Thread {
  pc: uint,
  end: uint,
  captures: ~[Option<CapturingGroup>]
}

impl Thread {
  fn new(pc: uint, end: uint) -> Thread {
    Thread { 
      pc: pc, 
      end: end,
      captures: ~[]
    }
  }
}

impl ToStr for Thread {
  fn to_str(&self) -> ~str {
    format!("<Thread pc: {:u}, end: {:u}>", self.pc, self.end)
  }
}

struct PikeVM {
  priv inst: ~[Instruction],
  priv len: uint,
  priv ncaps: uint
}

impl PikeVM {
  fn new(inst: ~[Instruction], ncaps: uint) -> PikeVM {
    let len = inst.len();
    PikeVM {
      inst: inst,
      len: len,
      ncaps: ncaps
    }
  }
}

impl PikeVM {
  #[inline]
  fn addThread(&self, mut t: Thread, tlist: &mut ~[Thread]) {
    loop {
      match self.inst[t.pc] {
        InstJump(addr) => {
          t.pc = addr;
        }
        InstSplit(laddr, raddr) => {
          let mut split = t.clone();
          split.pc = laddr;

          t.pc = raddr;

          self.addThread(split, tlist);
        }
        InstCaptureStart(num, ref id) => {
          t.pc = t.pc + 1;
          
          // Fill in spaces with None, if there is no
          // knowledge of a capture instruction
          while (t.captures.len() < num + 1) {
            t.captures.push(None);
          }

          t.captures[num] = Some(CapturingGroup::new(t.end, t.end, id, num));
        }
        InstCaptureEnd(num) => {
          t.pc = t.pc + 1;

          match t.captures[num] {
            Some(ref mut cap) => {
              cap.end = t.end;
            }
            None => unreachable!()
          }
        }
        InstNoop => { 
          t.pc = t.pc + 1;
        }
        _ => break 
      }
    }

    tlist.push(t); 
  }
}

impl ExecStrategy for PikeVM {
  fn run(&self, input: &str, start_index: uint) -> Option<Thread> {
    // \x03 is an end of string indicator. it resolves issues
    // the program reaches the end of the string, and still
    // needs to perform instructions
    // This needs to be accounted for when computing things like 
    // the end of the input string
    let input = input.to_owned().append("\x03");

    // `sp` is a reference to a byte position in the input string. 
    // Anytime this is incremented, we have to be aware of the number of
    // bytes the character is.
    let mut sp = 0; 
    let mut found = None;

    let mut clist: ~[Thread] = with_capacity(self.len);
    let mut nlist: ~[Thread] = with_capacity(self.len);
    
    // To start from an index other than than the first character, 
    // need to compute the number of bytes from the beginning to 
    // wherever we want to start
    for i in range(0, input.char_len()) {
      let c = input.char_at(sp);

      // Wait until the start_index is hit
      if (i == start_index) {
        break;
      }

      // Some chars are different byte lengths, so 
      // we can't just inc by 1
      sp += c.len_utf8_bytes();
    }

    self.addThread(Thread::new(0, sp), &mut clist);

    // The main loop. 
    // 
    // For each character in the input, loop through threads (starting with 
    // one dummy thread) that represent  different traversal 
    // paths through the list of instructions. The only
    // time new threads are created, are when `InstSplit` instructions occur.
    for i in range(start_index, input.char_len()) {
      let c = input.char_at(sp);

      sp += c.len_utf8_bytes();

      //println(format!("-- Execution ({:c}|{:u}) --", c, sp));

      while (clist.len() > 0) {
        let mut t = clist.shift();;

        match self.inst[t.pc] {
          InstLiteral(m) => {
            if (c == m && i != input.char_len()) {
              t.pc = t.pc + 1;
              t.end = sp;

              self.addThread(t, &mut nlist);
            }
          }
          InstRange(start, end) => {
            if (c >= start && c <= end && i != input.char_len()) {
              t.pc = t.pc + 1;
              t.end = sp;

              self.addThread(t, &mut nlist);
            }
          }
          InstDotAll => {
            t.pc = t.pc + 1;
            t.end = sp;

            self.addThread(t, &mut nlist);
          }
          InstAssertStart => {
            if (i == 0) {
              t.pc = t.pc + 1;

              self.addThread(t, &mut clist);
            }
          }
          InstAssertEnd => {
            // Account for the extra character added onto each 
            // input string
            if (i == input.char_len() - 1) {
              t.pc = t.pc + 1;

              self.addThread(t, &mut clist);
            }
          }
          InstWordBoundary => {
            if (i == 0 ||
                i == input.char_len()) {
              continue;
            } 
            if (i == start_index &&
                !input.char_at_reverse(t.end).is_alphanumeric()) {
              continue;
            }
            if (!c.is_alphanumeric()) {
              continue;
            } 
            t.pc = t.pc + 1;
            
            self.addThread(t, &mut clist);
          }
          InstNonWordBoundary => {
            if (i == start_index &&
                i != 0 &&
                input.char_at_reverse(t.end).is_alphanumeric()) {
              continue;
            }
            if (i != input.char_len() && 
                i != 0 &&
                i != start_index &&
                c.is_alphanumeric()) { 
              continue;
            }
            t.pc = t.pc + 1;

            self.addThread(t, &mut clist);
          }
          InstMatch => {
            found = Some(t.clone()); 
            break;
          }
          _ => unreachable!() 
        }
      }

      swap(&mut clist, &mut nlist);
      nlist.clear();
    }

    match found {
      Some(ref mut ma) => {
        if (ma.captures.len() < self.ncaps) {
          for _ in range(ma.captures.len(), self.ncaps) {
            ma.captures.push(None);
          }
        }
      }
      _ => { }
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
  priv len: uint,
  priv ncaps: uint
}

impl RecursiveBacktracking {
  fn new(inst: ~[Instruction], ncaps: uint) -> RecursiveBacktracking {
    let len = inst.len();
    RecursiveBacktracking {
      inst: inst,
      len: len,
      ncaps: ncaps
    }
  }
}

impl ExecStrategy for RecursiveBacktracking {
  fn run(&self, input: &str, start_index: uint) -> Option<Thread> {
    let input = input.to_owned().append("\x03");

    None
  }
}
