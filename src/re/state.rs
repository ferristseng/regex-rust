use error::ParseError::*;
use charclass::CharClass;

pub mod ParseFlags {
  pub static NoParseFlags:  u8 = 0b00000000;
  pub static NoCapture:     u8 = 0b00000010;
  pub static NonGreedy:     u8 = 0b00000100;
}

pub mod ParseStack {
  use super::{OpCode, Literal, Regexp};
  use charclass::CharClass;
  // different representations of expressions on
  // the stack
  pub enum Entry {
    Op(OpCode),
    Literal(Literal),
    Expression(Regexp),
    CharClass(CharClass)
  }
}

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
  OpCapture(uint, Option<~str>),
  OpRepeatOp(uint, Option<uint>),
  OpDotAll,
  OpNoop
}

// Regexp Literal
// a literal character in a regex string (i.e 'abcd')
pub struct Literal {
  value: ~str 
}

impl Literal {
  pub fn new(s: &str) -> Literal {
    Literal { value: s.clone().to_owned() }
  }
}

// Regexp
// represents a variable number of states with an
// operator applied to them. 
pub struct Regexp { 
  op: OpCode, 
  state0: Option<~ParseStack::Entry>, 
  state1: Option<~ParseStack::Entry>,
  flags: u8 
} 

impl Regexp {
  pub fn new(op: OpCode, state0: Option<~ParseStack::Entry>, 
             state1: Option<~ParseStack::Entry>) -> Regexp {
    Regexp { 
      op: op, 
      state0: state0, 
      state1: state1,
      flags: ParseFlags::NoParseFlags
    }
  }
}

impl Regexp {
  pub fn addFlag(&mut self, flag: u8) {
    self.flags = self.flags | flag;
  }
  pub fn hasFlag(&self, flag: u8) -> bool {
    (self.flags & flag) > 0
  }
}

// current state of parsing
//
// | - nparen:
// |   number of parenthases not resolved
// | - ncaps:
// |   number of parenthases seen
// | - ptr:
// |   reference to a position in the regexp input str
// | - len:
// |   length of the regexp input str
// | - flags:
// |   global flags
pub struct ParseState {
  priv stack: ~[ParseStack::Entry],
  priv nparen: uint,
  priv ncaps: uint,
  priv ptr: uint,
  priv len: uint,
  priv flags: u8 
}

impl ParseState {
  pub fn new(regexp: &str) -> ParseState {
    ParseState { 
      stack: ~[], 
      nparen: 0, 
      ncaps: 0,
      ptr: 0,
      len: regexp.len(),
      flags: ParseFlags::NoParseFlags 
    } 
  }
}

impl ParseState {
  pub fn addFlag(&mut self, flag: u8) {
    self.flags = self.flags | flag;
  }
  pub fn hasFlag(&self, flag: u8) -> bool {
    (self.flags & flag) > 0
  }
}

impl ParseState {
  pub fn incr(&mut self, num: uint) -> uint {
    let ptr = self.ptr;

    self.ptr += num;

    return ptr
  }
  pub fn ptr(&mut self) -> uint {
    return self.ptr
  }
  pub fn remainder(&mut self) -> uint {
    return self.len - self.ptr
  }
}

impl ParseState {
  pub fn pop(&mut self) -> Result<Regexp, ParseCode> {
    match self.stack.pop_opt() {
      Some(ParseStack::Expression(r)) => Ok(r),
      Some(ParseStack::Literal(r)) => {
        Ok(Regexp::new(OpNoop, Some(~ParseStack::Literal(r)), None))
      },
      Some(ParseStack::CharClass(r)) => {
        Ok(Regexp::new(OpNoop, Some(~ParseStack::CharClass(r)), None))
      },
      Some(ParseStack::Op(_)) => Err(ParseInternalError), 
      None => Err(ParseEmptyStack)
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
  pub fn pushLiteral(&mut self, s: &str) {
    self.stack.push(ParseStack::Literal(Literal::new(s)));
  }
  pub fn pushOperation(&mut self, op: OpCode) {
    self.stack.push(ParseStack::Op(op));
  }
  pub fn pushExpression(&mut self, r: Regexp) {
    self.stack.push(ParseStack::Expression(r));
  }
  pub fn pushCharClass(&mut self, cc: CharClass) {
    self.stack.push(ParseStack::CharClass(cc));
  }
}

impl ParseState {
  pub fn pushAlternation(&mut self) {
    self.pushOperation(OpAlternation);
  }
  pub fn pushLeftParen(&mut self) {
    self.nparen += 1;
    self.pushOperation(OpLeftParen);
  }
  pub fn pushDotAll(&mut self) {
    let r = Regexp::new(OpDotAll, None, None);
    self.pushExpression(r);
  }
  pub fn pushLineStart(&mut self) {
    let r = Regexp::new(OpLineStart, None, None);
    self.pushExpression(r);
  }
  pub fn pushLineEnd(&mut self) {
    let r = Regexp::new(OpLineEnd, None, None);
    self.pushExpression(r);
  }
}

impl ParseState {
  pub fn doAlternation(&mut self) -> ParseCode { 
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
      None => return ParseEmptyAlternate
    };
    // make sure there is an alternation operand on the stack,
    // otherwise, might have parsed incorrectly
    match self.stack.pop_opt() {
      Some(ParseStack::Op(OpAlternation)) => { },
      _ => return ParseUnexpectedOperand 
    };
    let branch2 = match self.stack.pop_opt() {
      Some(s) => s,
      None => return ParseEmptyAlternate
    };
    let r = Regexp::new(OpAlternation, Some(~branch2), Some(~branch1));

    self.pushExpression(r);

    ParseOk
  }
  pub fn doConcatenation(&mut self) -> ParseCode {
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
          return ParseOk;
        },
        Some(s) => s,
        None => return ParseEmptyConcatenate
      };

      match self.stack.pop_opt() {
        Some(ParseStack::Op(op)) => {
          self.pushOperation(op); 
          self.stack.push(branch1);
          return ParseOk;
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
        None => return ParseEmptyConcatenate 
      };
    }

    ParseOk
  }
  pub fn doLeftParen(&mut self, noncapturing: bool) -> ParseCode { 
    self.nparen -= 1;
    self.doConcatenation();
    let inner = self.stack.pop();
    // after the left paren operand should be on the top 
    // of the stack
    match self.stack.pop_opt() {
      Some(ParseStack::Op(OpLeftParen)) => { },
      _ => return ParseExpectedOperand 
    }
    let mut r = Regexp::new(OpCapture(self.ncaps, None), 
                            Some(~inner), 
                            None);
    if (noncapturing) {
      r.addFlag(ParseFlags::NoCapture);
    } else {
      self.ncaps += 1;
    }

    self.pushExpression(r);

    ParseOk
  }
  pub fn doRepeatOp(&mut self, op: OpCode) -> ParseCode { 
    match self.stack.pop_opt() {
      Some(r) => {
        let mut r = r;
        // check if we should have apply a 
        // nongreedy flag
        let nongreedy = match op {
          OpZeroOrOne => true,
          _ => false
        };
        // check to see if the expr on the top
        // of the stack has some repition 
        // op applied to it.
        let opcode = match &r {
          &ParseStack::Expression(ref e) => {
            match e.op {
              OpKleine => Some(OpKleine),
              OpOneOrMore => Some(OpOneOrMore),
              OpRepeatOp(s, e) => Some(OpRepeatOp(s,e)),
              OpZeroOrOne => Some(OpZeroOrOne),
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
        // otherwise, we can make a new expression.
        match opcode {
          Some(OpOneOrMore) | 
          Some(OpKleine) |
          Some(OpRepeatOp(_, _)) |
          Some(OpZeroOrOne) => {
            if (nongreedy) {
              match r {
                ParseStack::Expression(ref mut e) => {
                  e.addFlag(ParseFlags::NonGreedy);
                }
                _ => { } // should never hit this case
              }
              self.stack.push(r)
            } else {
              return ParseRepeatedRepetition; 
            }
          },
          None => {
            let expr = Regexp::new(op, Some(~r), None);
            self.pushExpression(expr);
          }
          _ => unreachable!() 
        }
      }
      _ => {
        return ParseEmptyRepetition;
      }
    }

    ParseOk
  }
  pub fn doKleine(&mut self) -> ParseCode { 
    self.doRepeatOp(OpKleine)
  }
  pub fn doOneOrMore(&mut self) -> ParseCode { 
    self.doRepeatOp(OpOneOrMore)
  }
  pub fn doZeroOrOne(&mut self) -> ParseCode { 
    self.doRepeatOp(OpZeroOrOne)
  }
  pub fn doBoundedRepetition(&mut self, start: uint, end: uint) -> ParseCode {
    match self.stack.pop_opt() {
      Some(ParseStack::Op(_)) => return ParseUnexpectedOperand,
      Some(s) => {
        if start <= end {
          let expr = Regexp::new(OpRepeatOp(start, Some(end)), Some(~s), None);
          self.pushExpression(expr);
        } else {
          return ParseEmptyRepetitionRange
        }
      }
      None => return ParseEmptyRepetition
    }

    ParseOk
  }
  pub fn doUnboundedRepetition(&mut self, start: uint) -> ParseCode {
    match self.stack.pop_opt() {
      Some(ParseStack::Op(_)) => return ParseUnexpectedOperand,
      Some(s) => {
        let expr = Regexp::new(OpRepeatOp(start, None), Some(~s), None);
        self.pushExpression(expr); 
      }
      None => return ParseEmptyRepetition
    }

    ParseOk
  }
}

// debug
impl ParseState {
  pub fn trace(&mut self) {
    println("--STACK--");
    for e in self.stack.iter() {
      println(format!("{:?}", e));
    }
  }
}
