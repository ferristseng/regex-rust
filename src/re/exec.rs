use std::vec;
use std::util::swap;
use compile::Instruction;
use compile::{InstLiteral, InstRange, InstMatch, InstJump, 
  InstCaptureStart, InstCaptureEnd, InstSplit, 
  InstAssertStart, InstAssertEnd, InstWordBoundary,
  InstNonWordBoundary, InstNoop, InstProgress};
use result::{Match, CapturingGroup};

/// This should be able to take compiled 
/// instructions and execute them (see compile.rs)
pub trait ExecStrategy {
  fn run(&self, input: &str, start_index: uint) -> Option<Thread>;
}

#[deriving(Clone)]
struct Thread {
  pc: uint,
  end: uint,
  start_sp: uint,
  captures: ~[Option<CapturingGroup>]
}

impl Thread {
  fn new(pc: uint, end: uint, start_sp: uint) -> Thread {
    Thread { 
      pc: pc, 
      end: end,
      start_sp: start_sp,
      captures: ~[]
    }
  }
}

impl ToStr for Thread {
  fn to_str(&self) -> ~str {
    format!("<Thread pc: {:u}, end: {:u}, start_sp: {:u}>", self.pc, self.end, self.start_sp)
  }
}

/// Pike VM implementation
///
/// Supports everything except
/// Assertions and Backreferences
pub struct PikeVM<'a> {
  priv inst:  &'a [Instruction],
  priv ncaps: uint
}

impl<'a> PikeVM<'a> {
  pub fn new(inst: &'a [Instruction], ncaps: uint) -> PikeVM<'a> {
    PikeVM {
      inst: inst,
      ncaps: ncaps 
    }
  }
}

impl<'a> PikeVM<'a> {
  #[inline]
  fn addThread(&self, mut t: Thread, tlist: &mut ~[Thread], sp: uint) {
    loop {
      match self.inst[t.pc] {
        InstJump(addr) => {
          t.pc = addr;
        }
        InstSplit(laddr, raddr) => {
          let mut split = t.clone();
          split.pc = laddr;
          split.start_sp = sp;

          t.pc = raddr;

          self.addThread(split, tlist, sp);
        }
        InstCaptureStart(num, ref name) => {
          t.pc = t.pc + 1;
          
          // Fill in spaces with None, if there is no
          // knowledge of a capture instruction
          while (t.captures.len() < num + 1) {
            t.captures.push(None);
          }

          t.captures[num] = Some(CapturingGroup::new(t.end, t.end, num, name));
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
        InstProgress => {
            if(t.start_sp < sp) {
                //println("Passed Progress Instruction");
                t.pc = t.pc + 1;
            } else {
                println!("Progess Instruction Failed {}", t.to_str());
                return;
            }
        }
        _ => break
      }
    }

    tlist.push(t);
  }
}

impl<'a> ExecStrategy for PikeVM<'a> {
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

    let mut clist: ~[Thread] = vec::with_capacity(self.inst.len());
    let mut nlist: ~[Thread] = vec::with_capacity(self.inst.len());
    
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
    self.addThread(Thread::new(0, sp, sp), &mut clist, 0);

    // The main loop.
    //
    // For each character in the input, loop through threads (starting with
    // one dummy thread) that represent different traversal
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

              self.addThread(t, &mut nlist, sp);
            }
          }
          InstRange(start, end) => {
            if (c >= start && c <= end && i != input.char_len()) {
              t.pc = t.pc + 1;
              t.end = sp;

              self.addThread(t, &mut nlist, sp);
            }
          }
          InstAssertStart => {
            if (i == 0) {
              t.pc = t.pc + 1;

              self.addThread(t, &mut clist, sp);
            }
          }
          InstAssertEnd => {
            // Account for the extra character added onto each
            // input string
            if (i == input.char_len() - 1) {
              t.pc = t.pc + 1;

              self.addThread(t, &mut clist, sp);
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
            
            self.addThread(t, &mut clist, sp);
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

            self.addThread(t, &mut clist, sp);
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

    // Adjust for captures that were
    // seen while parsing to get the proper
    // groups length in the `Match`.
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
