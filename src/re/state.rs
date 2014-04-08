pub struct State<'a> {
  pub input: &'a str,
  pub cursor: uint,
  pub len: uint,
  pub ncaptures: uint,
  pub nparens: uint
}

impl<'a> State<'a> {
  #[inline]
  pub fn new(input: &'a str) -> State<'a> {
    State {
      input: input,
      cursor: 0,
      len: input.char_len(),
      ncaptures: 0,
      nparens: 0
    }
  }
}

impl<'a> State<'a> {
  pub fn len(&self) -> uint {
    self.len
  }
  pub fn peek(&self) -> Option<char> {
    self.peekn(1)
  }
  pub fn peekn(&self, num: uint) -> Option<char> {
    let mut ptr = self.cursor;
    for _ in range(0, num) {
      if (ptr < self.input.len()) {
        ptr += self.input.char_at(ptr).len_utf8_bytes();
      }
    }
    if (ptr < self.input.len()) {
      Some(self.input.char_at(ptr))
    } else {
      None
    }
  }
  pub fn next(&mut self) {
    match self.current() {
      Some(c) => {
        self.cursor += c.len_utf8_bytes();
      }
      None => ()
    }
    self.len -= 1;
  }
  pub fn consume(&mut self, n: uint) {
    for _ in range(0, n) {
      self.next();
    }
  }
  pub fn current(&mut self) -> Option<char> {
    if (self.cursor < self.input.len()) {
      Some(self.input.char_at(self.cursor))
    } else {
      None
    }
  }
  pub fn isEnd(&self) -> bool {
    self.len == 0
  }
  pub fn hasUnmatchedParens(&self) -> bool {
    self.nparens > 0
  }
}
