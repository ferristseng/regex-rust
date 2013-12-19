use std::vec::with_capacity;
use std::util::swap;
use compile::Instruction;
use compile::{InstLiteral, InstRange, InstMatch, InstJump, 
  InstCaptureStart, InstCaptureEnd, InstSplit, InstDotAll, 
  InstNoop};

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
  pub fn run(&self, input: &str) -> Option<Match> {    
    match self.strat.run(input) {
      Some(t) => {
        Some(Match::new(0, t.sp, input.to_owned(), t.captures.clone()))
      }
      None => None 
    } 
  }

  pub fn replace(&self, input: &str, repstr: &str) {
    match self.strat.run(input) {
      Some(t) => {
        let matchstr = input.slice_to(t.sp);
        println(input.replace(matchstr, repstr));
      }
      None => println("[NOT FOUND]")
    }
  }
}

#[deriving(Clone)]
pub struct Match {
  start: uint,
  end: uint,
  input: ~str,
  groups: ~[Option<CapturingGroup>]
}

impl Match {
  fn new(start: uint, end: uint, input: &str, 
         groups: ~[Option<CapturingGroup>]) -> Match {
    Match {
      start: start,
      end: end,
      input: input.to_owned(),
      groups: groups
    }
  }
}

impl Match {
  pub fn group(&self, index: uint) -> ~str {
    if (index < self.groups.len()) {
      match self.groups[index] {
        Some(ref group) => {
          return self.input.slice(group.start, group.end).to_owned()
        }
        None => return ~""
      }
    }
    return ~""
  }
  pub fn matched(&self) -> ~str {
    self.input.slice(self.start, self.end).to_owned()
  }
}

impl ToStr for Match {
  fn to_str(&self) -> ~str {
    fmt!("<Match str: %s groups: %u>", self.input.slice(self.start, self.end), 
         self.groups.len())
  }
}

// capturing group

#[deriving(Clone)]
pub struct CapturingGroup {
  start: uint,
  end: uint,
  name: ~str,
  num: uint
}

impl CapturingGroup {
  fn new(start: uint, end: uint, name: &Option<~str>, num: uint) -> CapturingGroup {
    let name = match name {
      &Some(ref s) => s.to_owned(),
      &None => num.to_str()
    };
    CapturingGroup {
      start: start,
      end: end,
      name: name,
      num: num
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

#[deriving(Clone)]
struct Thread {
  pc: uint,
  sp: uint,
  captures: ~[Option<CapturingGroup>]
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

impl PikeVM {
  #[inline]
  fn addThread(&self, mut t: Thread, tlist: &mut ~[Thread]) {
    match self.inst[t.pc].op {
      InstJump(addr) => {
        t.pc = addr;

        self.addThread(t, tlist);
      }
      InstSplit(laddr, raddr) => {
        let mut split = t.clone();
        split.pc = raddr;

        t.pc = laddr;

        self.addThread(t, tlist);
        self.addThread(split, tlist);
      }
      InstCaptureStart(num, ref id) => {
        t.pc = t.pc + 1;
        
        // fill in spaces with None, if there is no
        // knowledge of a capture instruction
        while (t.captures.len() < num + 1) {
          t.captures.push(None);
        }

        t.captures[num] = Some(CapturingGroup::new(t.sp, t.sp, id, num));

        self.addThread(t, tlist);
      }
      InstCaptureEnd(num) => {
        t.pc = t.pc + 1;

        match t.captures[num] {
          Some(ref mut cap) => {
            cap.end = t.sp;
          }
          None => unreachable!()
        }

        self.addThread(t, tlist);
      }
      InstNoop => { 
        t.pc = t.pc + 1;

        self.addThread(t, tlist);
      }
      _ => { 
        tlist.push(t); 
      }
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
    
    self.addThread(Thread::new(0, sp), &mut clist);

    for c in input.iter() {
      // some chars are different byte lengths, so 
      // we can't just inc by 1
      sp += c.len_utf8_bytes();

      while (clist.len() > 0) {
        let mut t = clist.shift();;

        match self.inst[t.pc].op {
          InstLiteral(m) => {
            if (c == m && sp != input.len()) {
              t.pc = t.pc + 1;
              t.sp = sp;

              self.addThread(t, &mut nlist);
            }
          }
          InstRange(start, end) => {
            if (c >= start && c <= end && sp != input.len()) {
              t.pc = t.pc + 1;
              t.sp = sp;

              self.addThread(t, &mut nlist);
            }
          }
          InstDotAll => {
            t.pc = t.pc + 1;
            t.sp = sp;

            self.addThread(t, &mut nlist);
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
