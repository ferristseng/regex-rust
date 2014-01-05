use parse::Expr;
use parse::{Greedy, NonGreedy};
use parse::{Empty, Literal, CharClass, CharClassStatic, Alternation,
            Concatenation, Repetition, Capture, AssertWordBoundary,
            AssertNonWordBoundary, AssertStart, AssertEnd};
use charclass::Range;

#[deriving(Clone)]
pub enum Instruction {
  InstLiteral(char),
  InstRange(char, char),
  InstMatch,
  InstJump(uint),
  InstCaptureStart(uint, Option<~str>),
  InstCaptureEnd(uint),
  InstSplit(uint, uint),
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
      InstAssertStart           => ~"InstLineStart",
      InstAssertEnd             => ~"InstLineEnd",
      InstWordBoundary          => ~"InstWordBoundary",
      InstNonWordBoundary       => ~"InstNonWordBoundary",
      InstNoop                  => ~"InstNoop"
    }
  }
}

#[inline]
fn compile_charclass(ranges: &[Range], stack: &mut ~[Instruction]) {
  let mut ssize = stack.len();
  let mut rlen = ranges.len();
  let rsize = ssize + rlen * 3;

  for &(start, end) in ranges.iter() {
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

/// Generates a split insturction depending on the nongreedy quantifier
///
/// # Arguments
///
/// *  left - The preferred branch to take for nongreedy. If this branch matches first, 
///           the right hand side will not execute if nongreedy.
/// *  right - The preferred branch to take for greedy.
/// *  nongreedy - Specifies which branch to prefer (left or right).
#[inline]
fn generate_repeat_split(left: uint, right: uint, nongreedy: bool) -> Instruction {
  if (nongreedy) {
    InstSplit(left, right)
  } else {
    InstSplit(right, left)
  }
}

/// Calls _compile_recursive, then pushes a `InstMatch` onto the 
/// end of the Instruction stack
///
/// Returns the compiled stack of Instructions 
///
/// # Arguments
///
/// See `_compile_recursive(re: &Expr, stack: &mut ~[Instruction])`
pub fn compile_recursive(re: &Expr) -> ~[Instruction] {
  let mut stack = ~[];
  _compile_recursive(re, &mut stack);
  stack.push(InstMatch);

  //debug_stack(stack);
  
  stack
}

/// Compiles a Regexp into a list of Instructions recursively
///
/// Returns the number of captures instructions compiled.
///
/// # Arguments
///
/// * re - The Regexp to compile
/// * stack - The list of instructions to dump to
#[inline]
fn _compile_recursive(expr: &Expr, stack: &mut ~[Instruction]) -> uint {
  let mut ncap = 0;

  // Inserts a InstNoop
  macro_rules! placeholder(
    () => (
      stack.push(InstNoop)
    );
  )

  match *expr {
    Literal(c) => {
      stack.push(InstLiteral(c));
    }
    Alternation(ref lft, ref rgt) => {
      // Compile to:
      // ...
      //      Split(L1, L2)
      // L1:  (state0)
      //      Jump(L3)
      // L2:  (state1)
      // L3:  ...
      let ptr_split = stack.len();
      placeholder!();
      ncap += _compile_recursive(*lft, stack);

      let ptr_jmp = stack.len();
      placeholder!();
      ncap += _compile_recursive(*rgt, stack);

      let split = InstSplit(ptr_split + 1, ptr_jmp + 1);
      let jmp = InstJump(stack.len());

      stack[ptr_split] = split; 
      stack[ptr_jmp] = jmp; 
    }
    Concatenation(ref lft, ref rgt) => {
      // Compile to:
      // ...
      // (state0)
      // (state1)
      // ...
      ncap += _compile_recursive(*lft, stack);
      ncap += _compile_recursive(*rgt, stack);
    }
    CharClass(ref ranges) => { 
      compile_charclass(*ranges, stack);
    }
    CharClassStatic(ranges) => {
      compile_charclass(ranges, stack);
    }
    Capture(ref expr, id, ref name) => {
      ncap += 1;
      // Compile to:
      // ...
      // CaptureStart
      // (state0)
      // CaptureEnd
      stack.push(InstCaptureStart(id, (*name).clone()));
      ncap += _compile_recursive(*expr, stack);
      stack.push(InstCaptureEnd(id));
    }
    Repetition(ref expr, start, end, quantifier) => {
      let nongreedy = match quantifier {
        Greedy    => false,
        NonGreedy => true
      };

      for _ in range(0, start) {
        _compile_recursive(*expr, stack);
      }

      match end {
        Some(n) if n != start => {
          for _ in range(0, n - start) {
            let ptr_split = stack.len();
            placeholder!();
            ncap += _compile_recursive(*expr, stack);

            stack[ptr_split] = generate_repeat_split(stack.len(), ptr_split + 1, nongreedy);
          }
        }
        None => {
          let ptr_split = stack.len();

          placeholder!();
          ncap += _compile_recursive(*expr, stack);

          let jmp = InstJump(ptr_split);
          stack.push(jmp);

          stack[ptr_split] = generate_repeat_split(stack.len(), ptr_split + 1, nongreedy);
        }
        _ => ()
      }
    }
    AssertWordBoundary => {
      stack.push(InstWordBoundary);
    }
    AssertNonWordBoundary => {
      stack.push(InstNonWordBoundary);
    }
    AssertStart => {
      stack.push(InstAssertStart);
    }
    AssertEnd => {
      stack.push(InstAssertEnd);
    }
    Empty => ()
  }

  ncap
}

fn debug_stack(stack: &mut ~[Instruction]) {
  let mut count: uint = 0;

  println("--COMPILE STACK--");
  for e in stack.iter() {
    println(format!("{:u}: {:s}", count, e.to_str()));
    count += 1;
  }
}
