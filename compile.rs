use state::*;

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

enum InstOpCode {
  InstLiteral(char),
  InstRange(char, char),
  InstMatch,
  InstJump(uint),
  InstCaptureStart,
  InstCaptureEnd,
  InstSplit(uint, uint),
  InstNoop
}

struct Instruction {
  op: InstOpCode 
}

impl Instruction {
  fn new(op: InstOpCode) -> Instruction {
    Instruction { op: op }
  }
}

impl ToStr for Instruction {
  fn to_str(&self) -> ~str {
    match self.op {
      InstLiteral(c)    => fmt!("InstLiteral %c", c), 
      InstRange(s, e)   => fmt!("InstRange %c-%c", s, e),
      InstMatch         => ~"InstMatch", 
      InstJump(i)       => fmt!("InstJump %u", i),
      InstCaptureStart  => ~"InstCaptureStart",
      InstCaptureEnd    => ~"InstCaptureEnd",
      InstSplit(l, r)   => fmt!("InstSplit %u | %u", l, r),
      InstNoop          => ~"InstNoop"
    }
  }
}

fn compile_literal(lit: &Literal, stack: &mut ~[Instruction]) {
  for c in lit.value.iter() {
    stack.push(Instruction::new(InstLiteral(c)));
  }
}
          
fn compile_charclass(cc: &CharClass, stack: &mut ~[Instruction]) {

}

pub fn compile_recursive(re: &Regexp, stack: &mut ~[Instruction]) {
  _compile_recursive(re, stack);
  stack.push(Instruction::new(InstMatch));

  // debug
  println("--COMPILE STACK--");
  println(stack.to_str());
}

fn _compile_recursive(re: &Regexp, stack: &mut ~[Instruction]) {
  // recurse on a sub expression if type is Expression,
  // otherwise just compile
  macro_rules! recurse(
    ($re: expr) => (
      {
        match $re {
          &Some(~ParseStack::Expression(ref x)) => {
            compile_recursive(x, stack);
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
        _ => { } // unreachable
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
      stack.push(Instruction::new(InstNoop));
      recurse!(&re.state0);

      let ptr_jmp = stack.len();
      stack.push(Instruction::new(InstNoop));
      recurse!(&re.state1);
      
      stack[ptr_split] = Instruction::new(InstSplit(ptr_split + 1, ptr_jmp + 1));
      stack[ptr_jmp] = Instruction::new(InstJump(stack.len()));
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
    _ => { }
  }

  // debug
  println("--COMPILE STACK--");
  println(stack.to_str());
}
