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

// 
pub struct RegexpState {
  priv stack: ~[RegexpStack],
  priv nparen: uint 
}

impl RegexpState {
  pub fn new() -> RegexpState {
    RegexpState { stack: ~[], nparen: 0 } 
  }
}

impl RegexpState {
  pub fn pushLiteral(&mut self, s: &str) -> () {
    // special cases to consider that we can optimize for:
    //   * if we have multiple literals in a row, we can condense
    //     that expression into one expression (i.e 'abc' can just 
    //     be ReLiteral(abc))
    match self.stack.pop_opt() {
      Some(ReLiteral(l)) => {
        self.stack.push(ReLiteral(RegexpLiteral::new(l.value + s)));
      }
      Some(r) => { 
        self.stack.push(r);
        self.stack.push(ReLiteral(RegexpLiteral::new(s)));
      }
      None => {
        self.stack.push(ReLiteral(RegexpLiteral::new(s)));
      }
    }
  }
  pub fn pushOperation(&mut self, op: OpCode) -> () {
    self.stack.push(ReOp(op));
  }
  pub fn pushExpression(&mut self, r: Regexp) -> () {
    self.stack.push(ReExpression(r));
  }
}

impl RegexpState {
  pub fn pushAlternation(&mut self) -> () {
    self.pushOperation(OpAlternation);
  }
  pub fn pushLeftParen(&mut self) -> () {
    self.pushOperation(OpLeftParen);
  }
}

impl RegexpState {
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
    // try to take two items off the stack to
    // concatenate.
    //
    // state0 -> state1
    let branch1 = match self.stack.pop_opt() {
      Some(s) => s,
      None => return Err("Nothing to concatenate")
    };
    let branch2 = match self.stack.pop_opt() {
      Some(s) => s,
      None => return Err("Nothing to concatenate")
    };
    let r = Regexp::new(OpConcatenation, Some(~branch1), Some(~branch2));

    self.pushExpression(r);
    
    Ok(true)
  }
  pub fn tryConcatenation(&mut self) {
    // tries to perform a concatenation. this simply 
    // means performing a concatenation operation
    // if two or more items are on the stack.
    if (self.stack.len() > 1) {
      self.doConcatenation();
    }
  }
  pub fn doLeftParen(&mut self) {

  }
  pub fn doRightParen(&mut self) {

  }
  pub fn doKleine(&mut self) -> Result<bool, &'static str> {
    match self.stack.pop_opt() {
      Some(r) => {
        let mut r = r;
        // check to see if the expr on the top
        // of the stack has some repition 
        // op applied to it.
        let opcode = match &r {
          &ReExpression(ref e) => {
            match e.op {
              OpKleine      => Some(OpKleine),
              OpOneOrMore   => Some(OpOneOrMore),
              OpZeroOrMore  => Some(OpZeroOrMore)
            }
          },
          _ => {
            None
          }
        };
        // if we found that the expr already had
        // a repition op, try to condense (by collapsing
        // overlapping cases...i.e, *?, *+, **, can really
        // just be *).
        // 
        // note: in the match block below,
        // the (_) case will cover cases
        // Some(OpOneOrMore), Some(OpZeroOrMore)
        //
        // otherwise, we can make a new expression.
        match opcode {
          Some(OpKleine) => {
            self.stack.push(r);
          }
          None => {
            let expr = Regexp::new(OpKleine, Some(~r), None);
            self.pushExpression(expr);
          }
          _ => {
            match r {
              ReExpression(ref mut e) => {
                e.op = OpKleine;
              }
              _ => { }
            }
            self.stack.push(r);
          }
        }
      }
      _ => {
        return Err("'*' not applied to any state.")
      }
    }
    Ok(true)
  }
}

impl RegexpState {
  pub fn trace(&mut self) {
    println("--STACK--");
    for e in self.stack.iter() {
      println(fmt!("%?", e));
    }
  }
}
