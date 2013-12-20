// a match result

#[deriving(Clone)]
pub struct Match {
  start: uint,
  end: uint,
  input: ~str,
  groups: ~[Option<CapturingGroup>]
}

impl Match {
  pub fn new(start: uint, end: uint, input: &str, 
         groups: ~[Option<CapturingGroup>]) -> Match {
    Match {
      start: start,
      end: end,
      input: input.to_owned(),
      groups: groups
    }
  }
}

impl Match {
  pub fn group(&self, index: uint) -> ~str {
    if (index < self.groups.len()) {
      match self.groups[index] {
        Some(ref group) => {
          self.input.slice(group.start, group.end).to_owned()
        }
        None => ~""
      }
    } else {
      ~""
    }
  }
  pub fn matched(&self) -> ~str {
    if (self.start < self.input.len()) {
      self.input.slice(self.start, self.end).to_owned()
    } else {
      ~""
    }
  }
}

impl ToStr for Match {
  fn to_str(&self) -> ~str {
    format!("<Match str: {:s} groups: {:u}>", self.input.slice(self.start, self.end), 
            self.groups.len())
  }
}

// capturing group

#[deriving(Clone)]
pub struct CapturingGroup {
  start: uint,
  end: uint,
  name: ~str,
  num: uint
}

impl CapturingGroup {
  pub fn new(start: uint, end: uint, name: &Option<~str>, num: uint) -> CapturingGroup {
    let name = match name {
      &Some(ref s) => s.to_owned(),
      &None => num.to_str()
    };
    CapturingGroup {
      start: start,
      end: end,
      name: name,
      num: num
    }
  }
}

