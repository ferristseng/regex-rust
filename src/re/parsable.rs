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
  pub fn peek(&self) -> char {
    self.peekn(1)
  }
  pub fn peekn(&self, num: uint) -> char {
    let mut ptr = self.cursor;
    for _ in range(0, num) {
      ptr += self.input.char_at(ptr).len_utf8_bytes()
    }
    self.input.char_at(ptr)
  }
  pub fn next(&mut self) {
    self.len -= 1;
    self.cursor += self.current().len_utf8_bytes();
  }
  pub fn consume(&mut self, n: uint) {
    for _ in range(0, n) {
      self.next();
    }
  }
  pub fn current(&self) -> char {
    self.input.char_at(self.cursor)
  }
  pub fn isEnd(&self) -> bool {
    self.len == 0
  }
}
