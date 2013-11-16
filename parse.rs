// A Regex Operation
pub enum OpCode {
  OpConcatenation,
  OpAlternation,
  OpKleine,
  OpZeroOrOne,
  OpOneOrMore,
  OpLineStart,
  OpLineEnd,
  OpLeftParen,
  OpCapture,
}

// Flags for parsing 
// i'm including a bunch of extra flags for now,
// but we can add support for them as we go
enum ParseFlags {
  NoParseFlags  = 0,
  FoldCase      = 1 << 0
}

// Regexp Literal
// a literal character in a regex string (i.e 'abcd')
struct Literal {
  value: ~str 
}

impl Literal {
  fn new(s: &str) -> Literal {
    Literal { value: s.clone().to_owned() }
  }
}

// Regexp
// represents a variable number of states with an
// operator applied to them. 
struct Regexp { 
  op: OpCode, 
  state0: Option<~ParseStack::Entry>, 
  state1: Option<~ParseStack::Entry>
} 

impl Regexp {
  fn new(op: OpCode, state0: Option<~ParseStack::Entry>, 
         state1: Option<~ParseStack::Entry>) -> Regexp {
    Regexp { op: op, state0: state0, state1: state1 }
  }
}

// RegexpCharClass
// represents a character class (i.e '[a-z123]')
struct CharClass {
  priv negate: bool,
  priv ranges: ~[(char, char)] 
}

impl CharClass {
  fn new() -> CharClass {
    CharClass { ranges: ~[], negate: false }
  }
}

impl CharClass {
  fn negate(&mut self) {
    self.negate = true;
  }
  fn addRange(&mut self, s: char, e: char) -> Result<bool, &'static str> {
    Ok(true)
  }
  fn addChar(&mut self, s: char) -> Result<bool, &'static str> {
    Ok(true)
  }
}

mod ParseStack {
  use parse::OpCode;
  use parse::Literal;
  use parse::Regexp;
  use parse::CharClass;
  // different representations of expressions on
  // the stack
  pub enum Entry {
    Op(OpCode),
    Literal(Literal),
    Expression(Regexp),
    CharClass(CharClass)
  }
}

// current state of parsing
pub struct ParseState {
  priv stack: ~[ParseStack::Entry],
  priv nparen: uint, 
  flags: ParseFlags
}

impl ParseState {
  pub fn new() -> ParseState {
    ParseState { stack: ~[], nparen: 0, flags: NoParseFlags } 
  }
}

impl ParseState {
  pub fn pop(&mut self) -> Result<Regexp, &'static str> {
    match self.stack.pop_opt() {
      Some(ParseStack::Expression(r)) => Ok(r),
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
    self.stack.push(ParseStack::Literal(Literal::new(s)));
  }
  pub fn pushOperation(&mut self, op: OpCode) -> () {
    self.stack.push(ParseStack::Op(op));
  }
  pub fn pushExpression(&mut self, r: Regexp) -> () {
    self.stack.push(ParseStack::Expression(r));
  }
  pub fn pushCharClass(&mut self, cc: CharClass) -> () {
    self.stack.push(ParseStack::CharClass(cc));
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
    // state0 (branch2)
    // OpCode(OpAlternation)
    // state1 (branch1)
    let branch1 = match self.stack.pop_opt() {
      Some(s) => s,
      None => return Err("Nothing to alternate")
    };
    // make sure there is an alternation operand on the stack,
    // otherwise, might have parsed incorrectly
    match self.stack.pop_opt() {
      Some(ParseStack::Op(OpAlternation)) => { },
      _ => return Err("No alternation operand on the stack, but expected one")
    };
    let branch2 = match self.stack.pop_opt() {
      Some(s) => s,
      None => return Err("Nothing to alternate")
    };
    let r = Regexp::new(OpAlternation, Some(~branch2), Some(~branch1));

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
        Some(ParseStack::Op(op)) => {
          self.pushOperation(op); 
          return Ok(true);
        },
        Some(s) => s,
        None => return Err("Nothing to concatenate")
      };

      match self.stack.pop_opt() {
        Some(ParseStack::Op(op)) => {
          self.pushOperation(op); 
          self.stack.push(branch1);
          return Ok(true);
        },
        Some(ParseStack::Literal(s)) => {
          match branch1 {
            ParseStack::Literal(l) => self.pushLiteral(s.value + l.value),
            _ => { 
              let r = Regexp::new(OpConcatenation, 
                                  Some(~ParseStack::Literal(s)), 
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
      Some(ParseStack::Op(OpLeftParen)) => { },
      _ => return Err("Unexpected item on stack")
    }
    let r = Regexp::new(OpCapture, Some(~inner), None);
    self.pushExpression(r);
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
          &ParseStack::Expression(ref e) => {
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
                ParseStack::Expression(ref mut e) => {
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
