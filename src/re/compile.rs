use state::*;
use charclass::CharClass;

// instruction opcodes
//
// more opcodes are probably required, but for now:
//
// Literal      = check if the input matches a literal
// Range        = check if the input is within a range
// Match        = the input matches!
// Jump         = goto a point in the stack
// CaptureStart = start capturing the input
// CaptureEnd   = end capturing the input
// Split        = start a new thread with one jumping to the
//                first uint, and the next jumping to the
//                second.
// InstNoop     = placeholder in most cases

#[deriving(Clone)]
pub enum InstOpCode {
  InstLiteral(char),
  InstRange(char, char),
  InstMatch,
  InstJump(uint),
  InstCaptureStart(uint, Option<~str>),
  InstCaptureEnd(uint),
  InstSplit(uint, uint),
  InstDotAll,
  InstAssertStart,
  InstAssertEnd,
  InstNoop
}

#[deriving(Clone)]
pub struct Instruction {
  op: InstOpCode 
}

impl Instruction {
  pub fn new(op: InstOpCode) -> Instruction {
    Instruction { op: op }
  }
}

impl ToStr for Instruction {
  fn to_str(&self) -> ~str {
    match self.op {
      InstLiteral(c)            => format!("InstLiteral {:c}", c), 
      InstRange(s, e)           => format!("InstRange {:c}-{:c}", s, e),
      InstMatch                 => ~"InstMatch", 
      InstJump(i)               => format!("InstJump {:u}", i),
      InstCaptureStart(id, _)   => format!("InstCaptureStart {:u}", id),
      InstCaptureEnd(id)        => format!("InstCaptureEnd {:u}", id),
      InstSplit(l, r)           => format!("InstSplit {:u} | {:u}", l, r),
      InstDotAll                => ~"InstDotAll",
      InstAssertStart           => ~"InstLineStart",
      InstAssertEnd             => ~"InstLineEnd",
      InstNoop                  => ~"InstNoop"
    }
  }
}

#[inline]
fn compile_literal(lit: &Literal, stack: &mut ~[Instruction]) {
  for c in lit.value.chars() {
    stack.push(Instruction::new(InstLiteral(c)));
  }
}

#[inline]
fn compile_charclass(cc: &CharClass, stack: &mut ~[Instruction]) {
  let mut ssize = stack.len();
  let mut rlen = cc.ranges.len();
  let rsize = ssize + rlen * 3;

  for &(start, end) in cc.ranges.iter() {
    if (rlen >= 2) {
      let split = Instruction::new(InstSplit(ssize + 1, ssize + 3));
      stack.push(split);

      ssize += 3;
      rlen  -= 1;
    }

    if (start == end) {
      stack.push(Instruction::new(InstLiteral(start)));
    } else {
      stack.push(Instruction::new(InstRange(start, end)));
    }

    stack.push(Instruction::new(InstJump(rsize - 1)));
  }
}

// generates a split instruction
#[inline]
fn generate_repeat_split(left: uint, right: uint, nongreedy: bool) -> Instruction {
  if (nongreedy) {
    Instruction::new(InstSplit(left, right))
  } else {
    Instruction::new(InstSplit(right, left))
  }
}

pub fn compile_recursive(re: &Regexp, stack: &mut ~[Instruction]) {
  _compile_recursive(re, stack);
  stack.push(Instruction::new(InstMatch));

  //debug_stack(stack);
}

fn _compile_recursive(re: &Regexp, stack: &mut ~[Instruction]) {
  // recurse on a sub expression if type is Expression,
  // otherwise just compile
  macro_rules! recurse(
    ($re: expr) => (
      {
        match $re {
          &Some(~ParseStack::Expression(ref x)) => {
            _compile_recursive(x, stack);
          }
          &Some(~ParseStack::Literal(ref lit)) => {
            compile_literal(lit, stack);
          }
          &Some(~ParseStack::CharClass(ref cc)) => {
            compile_charclass(cc, stack);
          }
          _ => { } // unreachable
        }
      }
    );
  )

  // insert a InstNoop...to be replaced later
  macro_rules! placeholder(
    () => (
      stack.push(Instruction::new(InstNoop))
    );
  )

  match &re.op {
    // this should correspond with the case 
    // of the input being only a string (i.e 'abc')
    // or a char class (i.e '[a-zA-Z]')
    &OpNoop => {
      match re.state0 {
        Some(~ParseStack::Literal(ref lit)) => {
          compile_literal(lit, stack);
        }
        Some(~ParseStack::CharClass(ref cc)) => {
          compile_charclass(cc, stack);
        }
        None => { }
        _ => unreachable!() // unreachable
      };
    }
    // compile to:
    // ...
    //      Split(L1, L2)
    // L1:  (state0)
    //      Jump(L3)
    // L2:  (state1)
    // L3:  ...
    &OpAlternation => {
      let ptr_split = stack.len();
      placeholder!();
      recurse!(&re.state0);

      let ptr_jmp = stack.len();
      placeholder!();
      recurse!(&re.state1);
      
      let split = Instruction::new(InstSplit(ptr_split + 1, ptr_jmp + 1));
      let jmp = Instruction::new(InstJump(stack.len()));

      stack[ptr_split] = split; 
      stack[ptr_jmp] = jmp; 
    }
    // compile to:
    // ...
    // (state0)
    // (state1)
    // ...
    &OpConcatenation => {
      recurse!(&re.state0);
      recurse!(&re.state1);
    }
    // compile to:
    // ...
    // CaptureStart
    // (state0)
    // CaptureEnd
    &OpCapture(id, ref name) => {
      if (re.hasFlag(ParseFlags::NoCapture)) {
        recurse!(&re.state0);
      } else {
        stack.push(Instruction::new(InstCaptureStart(id, name.clone())));
        recurse!(&re.state0);
        stack.push(Instruction::new(InstCaptureEnd(id)));
      }
    }
    // compile to:
    // ...
    // L1: Split(L2, L3) | Split(L3, L2) (NonGreedy)
    // L2: (state0)
    //     Jump(L1)
    // L3: ...
    // ...
    &OpKleine => {
      let ptr_split = stack.len();
      let nongreedy = re.hasFlag(ParseFlags::NonGreedy);
      placeholder!();

      recurse!(&re.state0);
      let jmp = Instruction::new(InstJump(ptr_split));
      stack.push(jmp);

      stack[ptr_split] = generate_repeat_split(stack.len(), ptr_split + 1, nongreedy);
    }
    // compile to:
    // ...
    // L1: (state0)
    //     Split(L1, L2) | Split(L2, L1) (NonGreedy)
    // L2: ...
    // ...
    &OpOneOrMore => {
      let ptr_inst = stack.len();
      let nongreedy = re.hasFlag(ParseFlags::NonGreedy);

      recurse!(&re.state0);
      
      let split = generate_repeat_split(stack.len() + 1, ptr_inst, nongreedy);
      stack.push(split);
    }
    // compile to:
    // ...
    //     Split(L1, L2) | Split(L2, L1) (NonGreedy)
    // L1: (state0)
    // L2: ... 
    &OpZeroOrOne => {
      let ptr_split = stack.len();
      let nongreedy = re.hasFlag(ParseFlags::NonGreedy);
      placeholder!();

      recurse!(&re.state0);

      stack[ptr_split] = generate_repeat_split(stack.len(), ptr_split + 1, nongreedy);
    }
    // there are 3 cases for a repeat op
    // that depend on end:
    //
    //    1. end == start...exact repetition of start times
    //    2. end == Some(n)...bounded repetition 
    //       from start to end
    //    3. end == None...unbounded repetition
    &OpRepeatOp(start, end) => {
      let nongreedy = re.hasFlag(ParseFlags::NonGreedy);

      for _ in range(0, start) {
        recurse!(&re.state0);
      }

      match end {
        // corresponds to the 2nd case
        Some(n) if n != start => {
          // each iteration should compile something 
          // similar to the '?' operator
          for _ in range(0, n - start) {
            let ptr_split = stack.len();
            placeholder!();

            recurse!(&re.state0);

            stack[ptr_split] = generate_repeat_split(stack.len(), ptr_split + 1, nongreedy);
          }
        }
        // this should look like a '*' operator
        None => {
          let ptr_split = stack.len();
          placeholder!();

          recurse!(&re.state0);
          let jmp = Instruction::new(InstJump(ptr_split));
          stack.push(jmp);

          stack[ptr_split] = generate_repeat_split(stack.len(), ptr_split + 1, nongreedy);
        }
        // this corresponds to the 1st case
        _ => { }
      }
    }
    &OpDotAll => {
      stack.push(Instruction::new(InstDotAll));
    }
    &OpAssertStart => {
      stack.push(Instruction::new(InstAssertStart));
    }
    &OpAssertEnd => {
      stack.push(Instruction::new(InstAssertEnd));
    }
    _ => { } // these are not covered cases...remove when all cases are completely covered
  }
}

fn debug_stack(stack: &mut ~[Instruction]) {
  let mut count: uint = 0;

  println("--COMPILE STACK--");
  for e in stack.iter() {
    println(format!("{:u}: {:s}", count, e.to_str()));
    count += 1;
  }
}
