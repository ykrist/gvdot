use std::fmt;
use lazy_static::lazy_static;
use thiserror::Error;

#[derive(Debug, Copy, Clone)]
pub struct StrId<'a> {
  inner: &'a str
}

impl fmt::Display for StrId<'_> {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    f.write_str(self.inner)
  }
}

#[derive(Error, Debug, Clone)]
#[error("string is not a valid DOT id")]
pub struct InvalidStrId;

impl<'a> StrId<'a> {
  pub fn new(s: &'a str) -> Result<Self, InvalidStrId> {
    lazy_static! {
        static ref RE: regex::Regex = regex::Regex::new("[a-zA-Z-_][a-zA-Z-_0-9]*").unwrap();
    }
    if (&*RE).is_match(s) {
      Ok(StrId { inner: s })
    } else {
      Err(InvalidStrId)
    }
  }
}


pub trait Id : fmt::Display {}

impl Id for StrId<'_> {}

macro_rules! impl_id_primitive {
    ($($t:ty),+$(,)?) => {
      $(
        impl Id for $t {}
      )*
    };
}

impl_id_primitive!{
  u8,
  u16,
  u32,
  u64,
  u128,
  usize,
  i8,
  i16,
  i32,
  i64,
  i128,
  isize,
}

