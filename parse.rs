use std::ptr;

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
  pub fn addRange(&mut self, s: char, e: char) -> Result<bool, &'static str> {
    if (s < e) {
      self.ranges.push((s, e));
    } else {
      return Err("Empty range")
    }

    Ok(true)
  }
  pub fn addChar(&mut self, s: char) -> Result<bool, &'static str> {
    self.ranges.push((s,s));

    Ok(true)
  }
}

pub mod ParseStack {
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
      Some(ParseStack::Literal(r)) => Ok(Regexp::new(OpNoop, 
                                                     Some(~ParseStack::Literal(r)),
                                                     None)),
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

// parse functions
//
// these take in a pointer to a ParseState and an input string,
// and finish / modify the ParseState

pub fn parse_charclass(t: &mut ~str, s: *mut ParseState) -> Result<ParseState, &'static str> {
 
  let mut ps = unsafe { ptr::read_and_zero_ptr(s) };

  // check to see if the first char following
  // '[' is a '^', if so, it is a negated char 
  // class
  let mut cc = CharClass::new();

  // we need to keep track of any [, ( in
  // the input, because we can just ignore 
  // them
  let mut nbracket: uint = 0;

  match t.char_at(0) {
    '^' => {
      t.shift_char();
      cc.negate();
    },
    _ => { }
  };

  while (t.len() > 0) {

    match t.char_at(0) {
      '[' => {
        nbracket += 1;
        t.shift_char();
      },
      ']' => {
        t.shift_char();
        if (nbracket > 0) {
          nbracket -= 1;
        } else {
          ps.pushCharClass(cc);
          return Ok(ps);
        }
      }
      c => {
        t.shift_char();
        
        // check to see if its this is part of a 
        // range
        if (t.len() > 1) {
          match t.char_at(0) {
            '-' => {
              if (t.char_at(1) != ']') {
                match cc.addRange(c, t.char_at(1)) {
                  Err(e) => return Err(e),
                  _ => { } // Ok...continue
                }
                t.shift_char();
                t.shift_char();
              }
            }
            _ => { 
              cc.addChar(c);
            }
          }
        } else {
          cc.addChar(c); 
        }
      }
    }

  }

  Err("Expected a ']'.")
}

// parse an input string recursively
// ideally, we wouldn't parse an input string recursively
// because rust does not optimize tail end
// recursive calls, but...
// this way is pretty
pub fn parse_recursive(t: &mut ~str, s: *mut ParseState) -> Result<ParseState, &'static str> {
  
  let mut ps = unsafe { ptr::read_and_zero_ptr(s) };

  // check for an err,
  // if not update the state
  macro_rules! set_ok(
    ($f: expr) => (
      match $f {
        Ok(s)   => { ps = s; }
        Err(e)  => return Err(e)
      }
    );
  )

  // cases for
  // parsing different characters
  // in the input string
  while (t.len() > 0) {

    match t.char_at(0) {
      '(' => {
        ps.doConcatenation();
        ps.pushLeftParen();

        t.shift_char();
        set_ok!(parse_recursive(t, ptr::to_mut_unsafe_ptr(&mut ps)));
        ps.doLeftParen();
      },
      ')' => {
        t.shift_char();
        if (ps.hasUnmatchedParens()) {
          break;
        }
        return Err("Unmatched ')'")
      }

      '|' => {
        ps.doConcatenation();
        ps.pushAlternation();

        t.shift_char();
        set_ok!(parse_recursive(t, ptr::to_mut_unsafe_ptr(&mut ps)));
        ps.doAlternation();
        
        if (ps.hasUnmatchedParens()) {
          break;
        }
      },

      '*' => {
        t.shift_char();
        ps.doKleine();
      },
      '?' => {
        t.shift_char();
        ps.doZeroOrOne();
      },
      '+' => {
        t.shift_char();
        ps.doOneOrMore();
      }

      '[' => {
        t.shift_char();
        set_ok!(parse_charclass(t, ptr::to_mut_unsafe_ptr(&mut ps)));
      }

      c => {
        ps.pushLiteral(c.to_str());
        t.shift_char();
      }
    }

    ps.trace();

  }

  ps.doConcatenation();

  // replace the content at
  // the old pointer, if a state was passed in
  /*match s {
    Some(st) => {
      unsafe { ptr::replace_ptr(st, ps); }
    },
    _ => { }
  }*/

  Ok(ps)
}

