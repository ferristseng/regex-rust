use state::*;
use charclass::CharClass;

#[deriving(Clone)]
pub enum Instruction {
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
  InstWordBoundary,
  InstNonWordBoundary,
  InstNoop
}

impl ToStr for Instruction {
  fn to_str(&self) -> ~str {
    match *self {
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
      InstWordBoundary          => ~"InstWordBoundary",
      InstNonWordBoundary       => ~"InstNonWordBoundary",
      InstNoop                  => ~"InstNoop"
    }
  }
}

#[inline]
fn compile_literal(lit: &~str, stack: &mut ~[Instruction]) {
  for c in lit.chars() {
    stack.push(InstLiteral(c));
  }
}

#[inline]
fn compile_charclass(cc: &CharClass, stack: &mut ~[Instruction]) {
  let mut ssize = stack.len();
  let mut rlen = cc.ranges.len();
  let rsize = ssize + rlen * 3;

  for &(start, end) in cc.ranges.iter() {
    if (rlen >= 2) {
      let split = InstSplit(ssize + 1, ssize + 3);
      stack.push(split);

      ssize += 3;
      rlen  -= 1;
    }

    if (start == end) {
      stack.push(InstLiteral(start));
    } else {
      stack.push(InstRange(start, end));
    }

    stack.push(InstJump(rsize - 1));
  }
}

/**
 * Generates a split insturction depending on the nongreedy quantifier
 *
 * # Arguments
 *
 * *  left - The preferred branch to take for nongreedy. If this branch matches first, 
 *           the right hand side will not execute if nongreedy.
 * *  right - The preferred branch to take for greedy.
 * *  nongreedy - Specifies which branch to prefer (left or right).
 */
#[inline]
fn generate_repeat_split(left: uint, right: uint, nongreedy: bool) -> Instruction {
  if (nongreedy) {
    InstSplit(left, right)
  } else {
    InstSplit(right, left)
  }
}

/**
 * Calls _compile_recursive, then pushes a `InstMatch` onto the 
 * end of the Instruction stack
 *
 * # Arguments
 *
 * See `_compile_recursive(re: &Regexp, stack: &mut ~[Instruction])`
 */
pub fn compile_recursive(re: &Regexp, stack: &mut ~[Instruction]) {
  _compile_recursive(re, stack);
  stack.push(InstMatch);

  //debug_stack(stack);
}

/** 
 * Compiles a Regexp into a list of Instructions recursively
 *
 * # Arguments
 *
 * * re - The Regexp to compile
 * * stack - The list of instructions to dump to
 */
#[inline]
fn _compile_recursive(re: &Regexp, stack: &mut ~[Instruction]) {
  // Recurse on a subexpression if type is Expression,
  // otherwise just call the appropriate compile funciton.
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
          _ => unreachable!()
        }
      }
    );
  )

  // Inserts a InstNoop
  macro_rules! placeholder(
    () => (
      stack.push(InstNoop)
    );
  )

  match &re.op {
    // If the opcode is an OpNoop, there is a bare 
    // Literal, CharClass, or the input Regexp was 
    // empty.
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
    // Compile to:
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
      
      let split = InstSplit(ptr_split + 1, ptr_jmp + 1);
      let jmp = InstJump(stack.len());

      stack[ptr_split] = split; 
      stack[ptr_jmp] = jmp; 
    }
    // Compile to:
    // ...
    // (state0)
    // (state1)
    // ...
    &OpConcatenation => {
      recurse!(&re.state0);
      recurse!(&re.state1);
    }
    // Compile to:
    // ...
    // CaptureStart
    // (state0)
    // CaptureEnd
    &OpCapture(id, ref name) => {
      if (re.hasFlag(ParseFlags::NoCapture)) {
        recurse!(&re.state0);
      } else {
        stack.push(InstCaptureStart(id, name.clone()));
        recurse!(&re.state0);
        stack.push(InstCaptureEnd(id));
      }
    }
    // Compile to:
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
      let jmp = InstJump(ptr_split);
      stack.push(jmp);

      stack[ptr_split] = generate_repeat_split(stack.len(), ptr_split + 1, nongreedy);
    }
    // Compile to:
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
    // Compile to:
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
    // There are 3 cases for a repeat op
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
        // Corresponds to the 2nd case
        Some(n) if n != start => {
          // Each iteration should compile something 
          // similar to the '?' operator
          for _ in range(0, n - start) {
            let ptr_split = stack.len();
            placeholder!();

            recurse!(&re.state0);

            stack[ptr_split] = generate_repeat_split(stack.len(), ptr_split + 1, nongreedy);
          }
        }
        // This should look like a '*' operator
        None => {
          let ptr_split = stack.len();
          placeholder!();

          recurse!(&re.state0);
          let jmp = InstJump(ptr_split);
          stack.push(jmp);

          stack[ptr_split] = generate_repeat_split(stack.len(), ptr_split + 1, nongreedy);
        }
        // this corresponds to the 1st case
        _ => { }
      }
    }
    &OpDotAll => stack.push(InstDotAll),
    &OpAssertStart => stack.push(InstAssertStart),
    &OpAssertEnd => stack.push(InstAssertEnd),
    &OpWordBoundary => stack.push(InstWordBoundary),
    &OpNonWordBoundary => stack.push(InstNonWordBoundary),
    _ => unreachable!() 
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
