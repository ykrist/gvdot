use std::fmt;

mod attributes;
pub mod values;


pub struct Html<'a>(&'a str);

impl fmt::Display for Html<'_> {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    f.write_char('<')?;
    f.write_str(self.0)?;
    f.write_char('>')?;
    Ok(())
  }
}

pub struct QuotedStr<'a>(&'a str);


impl fmt::Display for QuotedStr<'_> {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    f.write_char('"')?;
    let mut parts = self.0.split('"');
    if let Some(s) = parts.next() {
      f.write_str(s)?;
    }
    for s in parts {
      f.write_str(r#"\""#)?;
      f.write_str(s)?;
    }
    f.write_char('"')?;
    Ok(())
  }
}
pub struct CompositeArrowShape<const N: usize>([values::ArrowShape; N]);

impl<const N: usize> fmt::Display for CompositeArrowShape<N> {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    for s in &self.0 {
      fmt::Display::fmt(s, f)?;
    }
    Ok(())
  }
}

pub fn quote(s: &str) -> QuotedStr {
  QuotedStr(s)
}

pub fn html(s: &str) -> Html {
  Html(s)
}


pub(crate) use private::PrepareValue;

mod private {
  use super::*;

  pub trait PrepareValue {
    type Output: fmt::Display;
    fn prepare(self) -> Self::Output;
  }

  macro_rules! impl_prepare {
    ($($ty:path),+$(,)?) => {
        $(
        impl PrepareValue for $ty {
          type Output = $ty;
          fn prepare(self) -> Self::Output { self }
        }
        )*
      };
  }

  impl_prepare! {
    u8,
    u16,
    u32,
    u64,
    usize,
    i8,
    i16,
    i32,
    i64,
    isize,
    f32,
    f64,
    values::Shape,
    values::RankDir,
    values::RankType,
    values::ClusterMode,
    values::ArrowShape,
  }

  impl<'a> PrepareValue for &'a str {
    type Output = QuotedStr<'a>;
    fn prepare(self) -> QuotedStr<'a> {
      QuotedStr(self)
    }
  }

  impl PrepareValue for Html<'_> {
    type Output = Self;
    fn prepare(self) -> Self::Output {
      self
    }
  }

  impl<const N: usize> PrepareValue for [values::ArrowShape; N] {
    type Output = CompositeArrowShape<N>;
    fn prepare(self) -> Self::Output {
      CompositeArrowShape(self)
    }
  }
}

/// A Graphviz attribute for a node, edge, graph, subgraph or cluster.
///
/// The trait is parameterised by the types that the attribute accepts.
/// For example, `Attribute<i32>` and `Attribute<f64>` are both implemented
/// for the [`Weight`](crate::attr::Weight) attribute, so one can use both `f64` and
/// `i32` when using `.attr(attr::Weight, value)`.
///
/// # Examples
/// ```
/// use gvdot::*;
///
/// let mut g = Graph::new().directed().in_memory();
/// g.add_node(0)?;
/// g.add_node(1)?;
/// g.add_edge(0, 1)?.attr(attr::Weight, 1.0)?;
/// g.add_edge(1, 0)?.attr(attr::Weight, 1u8)?;
/// # std::io::Result::Ok(())
/// ```
pub trait Attribute<V>: AttributeName {}

pub trait AttributeName {
  fn name() -> &'static str;
}

pub use attributes::*;
use std::fmt::Write;
