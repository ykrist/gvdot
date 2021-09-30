use std::fmt;
use lazy_static::lazy_static;

#[derive(Debug, Copy, Clone)]
pub struct StrId<'a> {
  inner: &'a str
}

impl fmt::Display for StrId<'_> {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    f.write_str(self.inner)
  }
}


impl<'a> StrId<'a> {
  pub fn new(s: &'a str) -> crate::DotResult<Self> {
    lazy_static! {
        static ref RE: regex::Regex = regex::Regex::new("[a-zA-Z-_][a-zA-Z-_0-9]*").unwrap();
    }
    if (&*RE).is_match(s) {
      Ok(StrId { inner: s })
    } else {
      Err(crate::DotError::InvalidStrId)
    }
  }
}


pub trait Id : fmt::Display {}

impl Id for StrId<'_> {}

impl Id for u64 {}

