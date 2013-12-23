use error::ParseError::*;
use charclass::CharClass;

/**
 * Flags that can be applied to Regexps
 */
pub mod ParseFlags {
  pub static NoParseFlags:  u8 = 0b00000000;
  pub static NoCapture:     u8 = 0b00000010;
  pub static NonGreedy:     u8 = 0b00000100;
}

/**
 * Holds the representations available for an Entry on
 * the stack maintained by the `ParseState`.
 *
 * # Options
 *
 * *  `Marker(OpCode)` - An OpCode marker. These are removed before 
 *    the final concatenation.
 * *  `Literal(Literal)` - A character that will be matched in 
 *    the input.
 * *  `Expression(Regexp)` - An Expression with up to two states
 *    with an operation applied to them.
 * *  `CharClass(CharClass)` - A class of characters (i.e [a-z])
 */
pub mod ParseStack {
  use super::{OpCode, Regexp};
  use charclass::CharClass;

  pub enum Entry {
    Marker(OpCode),
    Literal(~str),
    Expression(Regexp),
    CharClass(CharClass)
  }
}

/** 
 * Operations that can be applied to Expressions on the 
 * parse stack.
 *
 * # Markers
 *
 * Some of these op codes are markers for portions of the input
 * that are not competely parsed. `ParseState` expects that 
 * these markers are completely removed by the end of parsing.
 *
 * * `OpLeftParen`
 *  
 * # Operations
 *
 * The rest of these are operations applied to a up to two states.
 * For example, an alternation (a|b), can be represented as following:
 *
 *   Regexp { 
 *     op: OpAlternation, 
 *     state1: Some(~ParseStack::Literal(a)),
 *     state2: Some(~ParseStack::Literal(b))
 *     flags: ...
 *   }
 *
 * # Special Case
 *
 * * If the input string is empty, an Expression with an `OpNoop` is 
 *   applied to it.
 */
pub enum OpCode {
  OpConcatenation,
  OpAlternation,
  OpKleine,
  OpZeroOrOne,
  OpOneOrMore,
  OpAssertStart,
  OpAssertEnd,
  OpWordBoundary,
  OpNonWordBoundary,
  OpLeftParen(uint),
  OpCapture(uint, Option<~str>),
  OpRepeatOp(uint, Option<uint>),
  OpDotAll,
  OpNoop
}

/**
 * A variable number of out states, with an operation applied 
 * to them.
 *
 * Only the first state is use, if the Operation only requires
 * one state (the second state is `None`).
 *
 * # Opcodes and Corresponding Number of States
 * 
 * 0 states:
 *
 * *  OpAssertStart
 * *  OpAssertEnd
 * *  OpWordBoundary
 * *  OpNonWordBoundary
 * *  OpDotAll
 * *  OpNoop  
 * 
 * 1 state:
 *
 * *  OpKleine
 * *  OpZeroOrOne
 * *  OpOneOrMore
 * *  OpCapture
 * *  OpRepeatOp
 *
 * 2 states:
 *
 * *  OpConcatenation
 * *  OpAlternation
 */
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

/**
 * Keeps track of the state of parsing using a stack of expressions.
 *
 * # Fields
 *
 * *  stack - The stack or expressions (represented by `ParseStack::Entry`)
 * *  nparen - The number of parenthases not resolved (a right parenthases 
 *    has not been seen.
 * *  ncaps - The number of captures in total seen.
 * *  flags - Flags applied to all expressions in each entry on the stack.
 */
pub struct ParseState {
  priv stack: ~[ParseStack::Entry],
  priv nparen: uint,
  priv ncaps: uint,
  priv flags: u8 
}

impl ParseState {
  pub fn new() -> ParseState {
    ParseState { 
      stack: ~[], 
      nparen: 0, 
      ncaps: 0,
      flags: ParseFlags::NoParseFlags 
    } 
  }
  /**
   * Get the completed Regexp item from the stack.
   *
   * Call this after parsing has completed to retrieve the final 
   * Regexp tree. 
   */
  pub fn pop(&mut self) -> Result<Regexp, ParseCode> {
    match self.stack.pop_opt() {
      Some(ParseStack::Expression(r)) => Ok(r),
      Some(ParseStack::Literal(r)) => {
        Ok(Regexp::new(OpNoop, Some(~ParseStack::Literal(r)), None))
      },
      Some(ParseStack::CharClass(r)) => {
        Ok(Regexp::new(OpNoop, Some(~ParseStack::CharClass(r)), None))
      },
      Some(ParseStack::Marker(_)) => Err(ParseInternalError), 
      None => Ok(Regexp::new(OpNoop, None, None)) 
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
  pub fn hasUnmatchedParens(&mut self) -> bool {
    self.nparen > 0
  }
  pub fn hasNoParens(&mut self) -> bool {
    self.nparen == 0
  }
  pub fn getParenLen(&mut self) -> uint {
    self.nparen
  }
  pub fn getCaptureLen(&mut self) -> uint {
    self.ncaps
  }
}

/**
 * These are general commands for pushing representations of 
 * an Expression on the stack.
 */
impl ParseState {
  pub fn pushLiteral(&mut self, s: ~str) {
    self.stack.push(ParseStack::Literal(s));
  }
  pub fn pushOperation(&mut self, op: OpCode) {
    self.stack.push(ParseStack::Marker(op));
  }
  pub fn pushExpression(&mut self, r: Regexp) {
    self.stack.push(ParseStack::Expression(r));
  }
  pub fn pushCharClass(&mut self, cc: CharClass) {
    self.stack.push(ParseStack::CharClass(cc));
  }
}

/**
 * These methods wrap some of the general push commands. These are 
 * simple operations that only add to the stack, and do not modify
 * other elements on the stack.
 */
impl ParseState {
  /// Push an Alternation marker onto the stack. 
  pub fn pushAlternation(&mut self) {
    self.pushOperation(OpAlternation);
  }
  /// Push a Left Parenthases marker on the stack.
  pub fn pushLeftParen(&mut self) {
    self.nparen += 1;
    self.pushOperation(OpLeftParen(self.ncaps));
  }
  /// Push a DotAll expression on the stack. Because a '.' can   
  /// be repeated, this should be a Regexp (Expression) as opposed 
  /// to simply a marker.
  pub fn pushDotAll(&mut self) {
    let r = Regexp::new(OpDotAll, None, None);
    self.pushExpression(r);
  }
  /// Push an AssertStart expression on the stack ('^').
  pub fn pushAssertStart(&mut self) {
    let r = Regexp::new(OpAssertStart, None, None);
    self.pushExpression(r);
  }
  /// Push an AssertEnd expression on the stack ('$').
  pub fn pushAssertEnd(&mut self) {
    let r = Regexp::new(OpAssertEnd, None, None);
    self.pushExpression(r);
  }
  /// Push a WordBoundary expression on the stack ('\B')
  pub fn pushWordBoundary(&mut self) {
    let r = Regexp::new(OpWordBoundary, None, None);
    self.pushExpression(r);
  }
  /// Push a NonWordBoundary expression on the stack ('\b').
  pub fn pushNonWordBoundary(&mut self) {
    let r = Regexp::new(OpNonWordBoundary, None, None);
    self.pushExpression(r);
  }
}

/**
 * These methods modify preexisting items on the stack, 
 * and push new items on the stack. Unlike the push operations 
 * above, these can fail. They return an explicit `ParseCode` 
 * that describes the type of failure.
 */
impl ParseState {
  pub fn doAlternation(&mut self) -> ParseCode { 
    // Check the stack for a left branch (state0)
    let branch1 = match self.stack.pop_opt() {
      Some(s) => s,
      None => return ParseEmptyAlternate
    };
    // Make sure there is an Alternation operand on the stack
    match self.stack.pop_opt() {
      Some(ParseStack::Marker(OpAlternation)) => { },
      _ => return ParseUnexpectedOperand 
    };
    // Check the stack for a right branch (state1)
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
      // Try to take two items off the stack to
      // concatenate.
      // If either of them are Opcodes, just 
      // push them back no the stack, and return 
      // (this implies there is a singular item on the stack,
      // and can't be concatenated with anything)
      let branch1 = match self.stack.pop_opt() {
        Some(ParseStack::Marker(op)) => {
          self.pushOperation(op); 
          return ParseOk;
        },
        Some(s) => s,
        None => return ParseEmptyConcatenate
      };
      match self.stack.pop_opt() {
        Some(ParseStack::Marker(op)) => {
          self.pushOperation(op); 
          self.stack.push(branch1);
          return ParseOk;
        },
        // If a more than one Literal are on the stack, 
        // just combine them into one Literal
        Some(ParseStack::Literal(s)) => {
          match branch1 {
            ParseStack::Literal(l) => self.pushLiteral(s+ l),
            _ => { 
              let r = Regexp::new(OpConcatenation, 
                                  Some(~ParseStack::Literal(s)), 
                                  Some(~branch1)); 
              self.pushExpression(r);
            }
          }
        }
        // Otherwise, make a new Regexp with the out states being the 
        // branches that were popped earlier
        Some(s) => { 
          let r = Regexp::new(OpConcatenation, Some(~s), Some(~branch1));
          self.pushExpression(r);
        },
        None => return ParseEmptyConcatenate 
      };
    }

    ParseOk
  }
  /// Completes a left parenthases operation started by 
  /// putting a marker on the stack (by `pushLeftParen(&mut self)`)
  pub fn doLeftParen(&mut self, noncapturing: bool) -> ParseCode { 
    self.nparen -= 1;
    self.doConcatenation();
    let inner = self.stack.pop();
    // Check for the marker. If it isn't there or something else is
    // there's an error elsewhere in the code
    let capn = match self.stack.pop_opt() {
      Some(ParseStack::Marker(OpLeftParen(n))) => { 
        if (self.hasUnmatchedParens()) {
          n + self.nparen
        } else {
          n
        }
      },
      _ => return ParseExpectedOperand 
    };
    let mut r = Regexp::new(OpCapture(capn, None), 
                            Some(~inner), 
                            None);
    // Add a NoCapture flag if something prompted it in the
    // input string. Do not count it as a seen capture.
    if (noncapturing) {
      r.addFlag(ParseFlags::NoCapture);
    } else {
      self.ncaps += 1;
    }

    self.pushExpression(r);

    ParseOk
  }
  /// Applies an repeat operation to a state on the stack. 
  /// If there is none, throw an error.
  pub fn doRepeatOp(&mut self, op: OpCode) -> ParseCode { 
    match self.stack.pop_opt() {
      Some(r) => {
        let mut r = r;
        // If `op` is a '?', then it might be a nongreedy 
        // quantifier.
        let nongreedy = match op {
          OpZeroOrOne => true,
          _ => false
        };
        // Check the opcode on the item on top of the stack. 
        // If there already is a repeat opcode, and a nongreedy 
        // is true, then `op` is a nongreedy quantifier.
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
        match opcode {
          // Apply the nongreedy quantifier if `op` was a '?'
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
      // Nothing was on the stack
      _ => return ParseEmptyRepetition
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
  /// Apply a bounded repetition operator to an item on the stack if there is 
  /// one, and validate it as well.
  pub fn doBoundedRepetition(&mut self, start: uint, end: uint) -> ParseCode {
    match self.stack.pop_opt() {
      Some(ParseStack::Marker(_)) => return ParseUnexpectedOperand,
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
  /// Apply a unbounded repetition operator to an item on the stack if there is 
  /// one, and validate it as well.
  pub fn doUnboundedRepetition(&mut self, start: uint) -> ParseCode {
    match self.stack.pop_opt() {
      Some(ParseStack::Marker(_)) => return ParseUnexpectedOperand,
      Some(s) => {
        let expr = Regexp::new(OpRepeatOp(start, None), Some(~s), None);
        self.pushExpression(expr); 
      }
      None => return ParseEmptyRepetition
    }

    ParseOk
  }
}

impl ParseState {
  /// Prints the stack
  pub fn trace(&mut self) {
    println("--STACK--");
    for e in self.stack.iter() {
      println(format!("{:?}", e));
    }
  }
}
