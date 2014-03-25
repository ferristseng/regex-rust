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
  pub fn group(&self, index: uint) -> Option<~str> {
    if (index < self.groups.len()) {
      match self.groups[index] {
        Some(ref group) => {
          Some(self.input.slice(group.start, group.end).to_owned())
        }
        None => None
      }
    } else {
      None
    }
  }

  pub fn group_by_name(&self, name: &str) -> Option<~str> {
    let name = &name.to_owned();
    for group_wrap in self.groups.iter() {
      match *group_wrap {
        Some(ref group) => {
          match group.name {
            Some(ref group_name) if group_name == name => {
              return Some(self.input.slice(group.start, group.end).to_owned());
            }
            _ => {}
          };
        }
        None => {}
      };
    }
    None
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
    format!("<Match str: {:s} groups: {:u}>", self.matched(),
            self.groups.len())
  }
}

#[deriving(Clone)]
pub struct CapturingGroup {
  start: uint,
  end: uint,
  num: uint,
  name: Option<~str>
}

impl CapturingGroup {
  pub fn new(start: uint, end: uint, num: uint, name: &Option<~str>) -> CapturingGroup {
    CapturingGroup {
      start: start,
      end: end,
      num: num,
      name: name.clone()
    }
  }
}

