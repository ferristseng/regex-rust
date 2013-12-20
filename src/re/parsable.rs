pub struct Parsable<'a> {
  priv input: &'a str,
  priv cursor: uint,
  priv len: uint
}

impl<'a> Parsable<'a> {
  #[inline]
  pub fn new(input: &'a str) -> Parsable<'a> {
    Parsable {
      input: input,
      cursor: 0,
      len: input.char_len()
    }
  }
}

impl<'a> Parsable<'a> {
  pub fn len(&self) -> uint {
    self.len
  }
  pub fn peek(&self) -> Option<char> {
    self.peekn(1)
  }
  pub fn peekn(&self, num: uint) -> Option<char> {
    let mut ptr = self.cursor;
    for _ in range(0, num) {
      ptr += self.input.char_at(ptr).len_utf8_bytes()
    }
    if (ptr < self.input.len()) {
      Some(self.input.char_at(ptr))
    } else {
      None
    }
  }
  pub fn next(&mut self) {
    self.len -= 1;
    match self.current() {
      Some(c) => {
        self.cursor += c.len_utf8_bytes();
      }
      None => { }
    }
  }
  pub fn consume(&mut self, n: uint) {
    for _ in range(0, n) {
      self.next();
    }
  }
  pub fn current(&self) -> Option<char> {
    if (!self.isEnd()) {
      Some(self.input.char_at(self.cursor))
    } else {
      None
    }
  }
  pub fn isEnd(&self) -> bool {
    self.len == 0
  }
}
