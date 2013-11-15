// A Regex Operation
//
// Used on the stack to distinguish an operator
// (defined as an opcode), from a parsed
// regular expression.

pub enum OpCode {
  OpConcatenation,
  OpAlternation,
  OpKleine,
  OpZeroOrOne,
  OpOneOrMore,
  OpLineStart,
  OpLineEnd
}

// Regexp Literal
//
struct RegexpLiteral {
  value: ~str 
}

impl RegexpLiteral {
  pub fn new(s: &str) -> RegexpLiteral {
    RegexpLiteral { value: s.clone().to_owned() }
  }
}

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

// Representations of regexp on
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
    match self.stack.pop_opt() {
      Some(ReLiteral(l)) => {
        self.stack.push(ReLiteral(RegexpLiteral::new(l.value + s)));
      }
      _ => { 
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
  pub fn doAlternation(&mut self) {
    self.pushOperation(OpAlternation);
  }
  pub fn doConcatenation(&mut self) {
    self.pushOperation(OpConcatenation);
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
        // otherwise, we can make a new expression.
        match opcode {
          Some(OpKleine) => {
            self.stack.push(r);
          }
          Some(OpOneOrMore) => {
            match r {
              ReExpression(ref mut e) => {
                e.op = OpKleine;
              }
              _ => { }
            }
            self.stack.push(r);
          }
          _ => {
            let expr = Regexp::new(OpKleine, Some(~r), None);
            self.pushExpression(expr);
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
