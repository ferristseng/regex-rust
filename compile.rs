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
  InstSplit(uint, uint)
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
      InstSplit(l, r)   => fmt!("InstSplit %u | %u", l, r)
    }
  }
}

pub fn compile_recursive(re: Regexp, stack: &mut ~[Instruction]) {
  match re.op {
    // this should correspond with the case 
    // of the input being only a string (i.e 'abc')
    OpNoop => {
      let lit = match re.state0 {
        ~ParseStack::Literal(s) => s,
        _ => Literal::new("")
      };
      for c in lit.value.iter() {
        stack.push(Instruction::new(InstLiteral(c)));
      }
      stack.push(Instruction::new(InstMatch));
    }
    OpAlternation | 
    OpConcatenation => {
       
    }
    _ => { }
  }

  println("--COMPILE STACK--");
  println(stack.to_str());
}
