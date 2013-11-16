// A Regex Operation
//
pub enum OpCode {
  OpConcatenation,
  OpAlternation,
  OpKleine,
  OpZeroOrOne,
  OpOneOrMore,
  OpLineStart,
  OpLineEnd,
  OpLeftParen,
  OpRightParen
}

// Regexp Literal
// a literal character in a regex string (i.e 'abcd')
struct RegexpLiteral {
  value: ~str 
}

impl RegexpLiteral {
  pub fn new(s: &str) -> RegexpLiteral {
    RegexpLiteral { value: s.clone().to_owned() }
  }
}

// Regexp
// represents a variable number of states with an
// operator applied to them. 
struct Regexp { 
  op: OpCode, 
  state0: Option<~RegexpStack>, 
  state1: Option<~RegexpStack>
} 

impl Regexp {
  fn new(op: OpCode, state0: Option<~RegexpStack>, state1: 
         Option<~RegexpStack>) -> Regexp {
    Regexp { op: op, state0: state0, state1: state1 }
  }
}

// RegexpCharClass
// represents a character class (i.e '[a-z123]')
struct RegexpCharClass;

// different representations of expressions on
// the stack
enum RegexpStack {
  ReOp(OpCode),
  ReLiteral(RegexpLiteral),
  ReExpression(Regexp)
}

// current state of parsing
pub struct ParseState {
  priv stack: ~[RegexpStack],
  priv nparen: uint 
}

impl ParseState {
  pub fn new() -> ParseState {
    ParseState { stack: ~[], nparen: 0 } 
  }
}

impl ParseState {
  pub fn pop(&mut self) -> Result<Regexp, &'static str> {
    match self.stack.pop_opt() {
      Some(ReExpression(r)) => Ok(r),
      _ => Err("Unknown error")
    }
  }
}

impl ParseState {
  pub fn hasUnmatchedParens(&mut self) -> bool {
    self.nparen > 0
  }
  pub fn hasNoParens(&mut self) -> bool {
    self.nparen == 0
  }
}

impl ParseState {
  pub fn pushLiteral(&mut self, s: &str) -> () {
    self.stack.push(ReLiteral(RegexpLiteral::new(s)));
  }
  pub fn pushOperation(&mut self, op: OpCode) -> () {
    self.stack.push(ReOp(op));
  }
  pub fn pushExpression(&mut self, r: Regexp) -> () {
    self.stack.push(ReExpression(r));
  }
}

impl ParseState {
  pub fn pushAlternation(&mut self) -> () {
    self.pushOperation(OpAlternation);
  }
  pub fn pushLeftParen(&mut self) -> () {
    self.nparen += 1;
    self.pushOperation(OpLeftParen);
  }
}

impl ParseState {
  pub fn doAlternation(&mut self) -> Result<bool, &'static str> {
    // try to pop off two items from the stack.
    // these should be branches that you can take.
    //     -> state0
    // s |
    //     -> state1
    // 
    // with alternation occuring sequentially, we can 
    // make an equivalent model by having state0 or state1 
    // also be a regexp with a alternation operand applied to it.
    //     -> state0
    // s |              -> 'state0
    //     -> state1  |
    //                  -> 'state1
    //
    // the operand should be pushed onto the stack before we start parsing 
    // the right hand side of the alternation, so the stack should look something
    // like this before doAlternation() is called:
    //
    // state0
    // OpCode(OpAlternation)
    // state1
    let branch1 = match self.stack.pop_opt() {
      Some(s) => s,
      None => return Err("Nothing to alternate")
    };
    match self.stack.pop_opt() {
      Some(ReOp(OpAlternation)) => { },
      _ => return Err("No alternation operand on the stack, but expected one")
    };
    let branch2 = match self.stack.pop_opt() {
      Some(s) => s,
      None => return Err("Nothing to alternate")
    };
    let r = Regexp::new(OpAlternation, Some(~branch1), Some(~branch2));

    self.pushExpression(r);

    Ok(true)
  }
  pub fn doConcatenation(&mut self) -> Result<bool, &'static str> {
    while (self.stack.len() > 1) {
      // try to take two items off the stack to
      // concatenate.
      // if either of them are opcodes, just 
      // push them back no the stack, and return 
      // (this means there is a singular item on the stack,
      // and can't be concatenated with anything)
      //
      // state0 -> state1
      let branch1 = match self.stack.pop_opt() {
        Some(ReOp(op)) => {
          self.pushOperation(op); 
          return Ok(true);
        },
        Some(s) => s,
        None => return Err("Nothing to concatenate")
      };

      match self.stack.pop_opt() {
        Some(ReOp(op)) => {
          self.pushOperation(op); 
          self.stack.push(branch1);
          return Ok(true);
        },
        Some(ReLiteral(s)) => {
          match branch1 {
            ReLiteral(l) => self.pushLiteral(s.value + l.value),
            _ => { 
              let r = Regexp::new(OpConcatenation, Some(~ReLiteral(s)), 
                                  Some(~branch1)); 
              self.pushExpression(r);
            }
          }
        }
        Some(s) => { 
          let r = Regexp::new(OpConcatenation, Some(~s), Some(~branch1));
          self.pushExpression(r);
        },
        None => return Err("Nothing to concatenate")
      };
    }
    Ok(true)
  }
  pub fn doLeftParen(&mut self) -> Result<bool, &'static str> {
    self.nparen -= 1;
    self.doConcatenation();
    let inner = self.stack.pop();
    // after the left paren operand should be on the top 
    // of the stack
    match self.stack.pop_opt() {
      Some(ReOp(OpLeftParen)) => { },
      _ => return Err("Unexpected item on stack")
    }
    self.stack.push(inner);
    Ok(true)
  }
  pub fn doRepeatOp(&mut self, op: OpCode) -> Result<bool, &'static str> {
    match self.stack.pop_opt() {
      Some(r) => {
        let mut r = r;
        // check if we should have apply a 
        // nongreedy flag
        let mut nongreedy = false;
        // check to see if the expr on the top
        // of the stack has some repition 
        // op applied to it.
        let opcode = match &r {
          &ReExpression(ref e) => {
            match e.op {
              OpKleine => Some(OpKleine),
              OpOneOrMore => Some(OpOneOrMore),
              OpZeroOrOne => {
                nongreedy = true;
                Some(OpZeroOrOne)
              },
              _ => None
            }
          },
          _ => {
            None
          }
        };
        // if we found that the expr already had
        // a repeat op, we might need to throw a 
        // error
        // 
        // note: in the match block below,
        // the (_) case will cover cases
        // Some(OpOneOrMore), Some(OpZeroOrOne)
        //
        // otherwise, we can make a new expression.
        match opcode {
          Some(OpOneOrMore) | 
          Some(OpKleine) |
          Some(OpZeroOrOne) => {
            if (nongreedy) {
              match r {
                ReExpression(ref mut e) => {
                  // set greedy flag
                }
                _ => { } // should never hit this case
              }
              self.stack.push(r)
            }
            return Err("Repeated use of repetition.");
          },
          None => {
            let expr = Regexp::new(op, Some(~r), None);
            self.pushExpression(expr);
          }
          _ => { } // should never hit this case
        }
      }
      _ => {
        return Err("'*' not applied to any state.")
      }
    }
    Ok(true)
  }
  pub fn doKleine(&mut self) -> Result<bool, &'static str> {
    self.doRepeatOp(OpKleine)
  }
  pub fn doOneOrMore(&mut self) -> Result<bool, &'static str> {
    self.doRepeatOp(OpOneOrMore)
  }
  pub fn doZeroOrOne(&mut self) -> Result<bool, &'static str> {
    self.doRepeatOp(OpZeroOrOne)
  }
}

impl ParseState {
  pub fn trace(&mut self) {
    println("--STACK--");
    for e in self.stack.iter() {
      println(fmt!("%?", e));
    }
  }
}
