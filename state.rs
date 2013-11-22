use error::ParseError::*;

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
  OpNoop
}

// Flags for parsing 
// i'm including a bunch of extra flags for now,
// but we can add support for them as we go
//
// actually, not really sure how these work
enum ParseFlags {
  NoParseFlags  = 0,
  FoldCase      = 1 << 0
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
struct Regexp { 
  op: OpCode, 
  state0: Option<~ParseStack::Entry>, 
  state1: Option<~ParseStack::Entry>
} 

impl Regexp {
  pub fn new(op: OpCode, state0: Option<~ParseStack::Entry>, 
             state1: Option<~ParseStack::Entry>) -> Regexp {
    Regexp { op: op, state0: state0, state1: state1 }
  }
}

// RegexpCharClass
// represents a character class (i.e '[a-z123]')
pub struct CharClass {
  priv negate: bool,
  priv ranges: ~[(char, char)] 
}

impl CharClass {
  pub fn new() -> CharClass {
    CharClass { ranges: ~[], negate: false }
  }
}

impl CharClass {
  pub fn negate(&mut self) {
    self.negate = true;
  }
  pub fn containsChar(&mut self, c: char) -> bool {
    true
  }
  pub fn addRange(&mut self, s: char, e: char) -> ParseCode {
    if (s < e) {
      self.ranges.push((s, e));
    } else {
      return ParseEmptyCharClassRange;
    }

    ParseOk
  }
  pub fn addChar(&mut self, s: char) -> ParseCode {
    self.ranges.push((s,s));

    ParseOk
  }
}

pub mod ParseStack {
  use state::OpCode;
  use state::Literal;
  use state::Regexp;
  use state::CharClass;
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
            return ParseRepeatedRepetition; 
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
}

impl ParseState {
  pub fn trace(&mut self) {
    println("--STACK--");
    for e in self.stack.iter() {
      println(fmt!("%?", e));
    }
  }
}

