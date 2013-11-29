use extra::sort::merge_sort;
use std::char::{from_u32, MAX};
use error::ParseError::*;

pub mod ParseFlags {
  // Flags for parsing 
  // Taken from re2 so there might be extras
  pub static NoParseFlags:  u32 = 0b00000000000000000000000000000000;
  pub static FoldCase:      u32 = 0b00000000000000000000000000000001;
  pub static Literal:       u32 = 0b00000000000000000000000000000010;
  pub static ClassNL:       u32 = 0b00000000000000000000000000000100;
  pub static DotNL:         u32 = 0b00000000000000000000000000001000;
  pub static MatchNL:       u32 = ClassNL | DotNL; 
  pub static OneLine:       u32 = 0b00000000000000000000000000010000;
  pub static Latin1:        u32 = 0b00000000000000000000000000100000;
  pub static NonGreedy:     u32 = 0b00000000000000000000000001000000;
  pub static PerlClasses:   u32 = 0b00000000000000000000000010000000;
  pub static PerlB:         u32 = 0b00000000000000000000000100000000;
  pub static PerlX:         u32 = 0b00000000000000000000001000000000;
  pub static UnicodeGroups: u32 = 0b00000000000000000000010000000000;
  pub static NeverNL:       u32 = 0b00000000000000000000100000000000;
  pub static NeverCapture:  u32 = 0b00000000000000000001000000000000;
  pub static LikePERL:      u32 = ClassNL | OneLine | PerlClasses | 
                                  PerlB | PerlX | UnicodeGroups;
  pub static WasDollar:     u32 = 0b00000000000000000010000000000000;
}

pub mod ParseStack {
  use super::*;
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
  OpCapture,
  OpRepeatOp(uint, Option<uint>),
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
  flags: u32 
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
  pub fn addFlag(&mut self, flag: u32) {
    self.flags = self.flags | flag;
  }
  pub fn hasFlag(&self, flag: u32) -> bool {
    (self.flags & flag) > 0
  }
}

// RegexpCharClass
// represents a character class (i.e '[a-z123]')
pub struct CharClass {
  ranges: ~[(char, char)] 
}

impl CharClass {
  pub fn new() -> CharClass {
    CharClass { ranges: ~[] }
  }
}

impl CharClass {
  pub fn negate(&mut self) -> ParseCode {
    let ordered = do merge_sort(self.ranges) |range1, range2| {
      let &(start1, _) = range1;
      let &(start2, _) = range2;
      start1 <= start2
    };

    let mut min: char = '\U00000000'; 
    let max: char = MAX;

    self.ranges = ~[];

    for &(start, end) in ordered.iter() {
      match CharClass::prev_char(start) {
        Some(e) => {
          if (min < e) {
            self.addRange(min, e); 
          }
        },
        None => { } // continue
      };
      if (min < end) {
        min = match CharClass::next_char(end) {
          Some(c) => c,
          None => end
        };
      }
    }

    // patch the end
    if (min != max) {
      self.addRange(min, max);
    }

    if self.ranges.len() == 0 {
      ParseEmptyCharClassRange
    } else {
      ParseOk
    }
  }
  pub fn containsChar(&mut self, c: char) -> bool {
    let mut covered = false;

    for &(start, end) in self.ranges.iter() {
      if (c >= start && c <= end) {
        covered = true;
        break
      }
    }

    covered
  }
  pub fn addRange(&mut self, s: char, e: char) -> ParseCode {
    if (s <= e) {
      self.ranges.push((s, e));
    } else {
      return ParseEmptyCharClassRange;
    }

    ParseOk
  }
}

impl CharClass {
  fn prev_char(c: char) -> Option<char> {
    match from_u32(c as u32 - 1) {
      None => None,
      r    => r   
    }
  }
  fn next_char(c: char) -> Option<char> {
    match from_u32(c as u32 + 1) {
      None => None,
      r    => r
    }
  }
}

// current state of parsing
pub struct ParseState {
  priv stack: ~[ParseStack::Entry],
  priv nparen: uint, 
  flags: u32 
}

impl ParseState {
  pub fn new() -> ParseState {
    ParseState { 
      stack: ~[], 
      nparen: 0, 
      flags: ParseFlags::NoParseFlags 
    } 
  }
}

impl ParseState {
  pub fn addFlag(&mut self, flag: u32) {
    self.flags = self.flags | flag;
  }
  pub fn hasFlag(&self, flag: u32) -> bool {
    (self.flags & flag) > 0
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
  pub fn doLeftParen(&mut self) -> ParseCode { 
    self.nparen -= 1;
    self.doConcatenation();
    let inner = self.stack.pop();
    // after the left paren operand should be on the top 
    // of the stack
    match self.stack.pop_opt() {
      Some(ParseStack::Op(OpLeftParen)) => { },
      _ => return ParseExpectedOperand 
    }
    let r = Regexp::new(OpCapture, Some(~inner), None);
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
          _ => { } // should never hit this case
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
      None => return ParseEmptyStack
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
      None => return ParseEmptyStack
    }

    ParseOk
  }
}

// debug
impl ParseState {
  pub fn trace(&mut self) {
    println("--STACK--");
    for e in self.stack.iter() {
      println(fmt!("%?", e));
    }
  }
}

// tests

#[cfg(test)]
mod char_class_tests {
  use std::char::MAX;
  use state::*;
  use error::ParseError::*;
  
  #[test]
  fn char_class_good() {
    let mut cc = CharClass::new(); 
    cc.addRange('A', 'Z');
    cc.addRange('F', 'F');
    cc.addRange('A', 'あ');
    assert_eq!(cc.ranges, ~[('A', 'Z'), ('F', 'F'), ('A', 'あ')]); 
  }

  #[test]
  fn char_class_empty() {
    let mut cc = CharClass::new();
    assert!(match cc.addRange('Z', 'A') { 
      ParseEmptyCharClassRange => true, _ => false });
  }

  #[test]
  fn char_class_negate() {
    let mut cc = CharClass::new();
    cc.addRange('A', '\U0000FA08');
    cc.negate();
    assert_eq!(cc.ranges, ~[('\U00000000', '@'), ('\U0000FA09', MAX)]);
  }

  #[test]
  fn char_class_negate_multiple() {
    let mut cc = CharClass::new();
    cc.addRange('們', '我');
    cc.addRange('A', 'Z');
    cc.negate();
    assert_eq!(cc.ranges, ~[('\U00000000', '@'), ('[', '\U00005010'), 
               ('\U00006212', MAX)])
  }

  #[test]
  fn char_class_negate_overlap() {
    let mut cc = CharClass::new();
    cc.addRange('a', 'd');
    cc.addRange('c', 'c');
    cc.negate();
    assert_eq!(cc.ranges, ~[('\U00000000', '`'), ('e', MAX)]);
  }

  #[test]
  fn char_class_negate_bounds() {
    let mut cc = CharClass::new();
    cc.addRange('\U00000000', MAX);
    assert!(match cc.negate() {
      ParseEmptyCharClassRange => true, _ => false });
  }
}

#[cfg(test)]
mod flag_tests {
  use super::*;

  #[test]
  fn test_add_flag_ok() {
    let mut re = Regexp::new(OpNoop, None, None); 
    re.addFlag(ParseFlags::OneLine);
    assert!(re.flags == ParseFlags::OneLine)
  }

  #[test]
  fn test_multiple_add_flag_ok() {
    let mut re = Regexp::new(OpNoop, None, None);
    re.addFlag(ParseFlags::OneLine);
    re.addFlag(ParseFlags::NeverCapture);
    assert!(re.flags == ParseFlags::OneLine | ParseFlags::NeverCapture);
  }

  #[test]
  fn test_has_flag_ok() {
    let mut re = Regexp::new(OpNoop, None, None);
    re.addFlag(ParseFlags::NeverCapture);
    re.addFlag(ParseFlags::OneLine);
    re.addFlag(ParseFlags::NonGreedy);
    assert!(re.hasFlag(ParseFlags::NonGreedy));
  }
}
