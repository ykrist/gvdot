use thiserror::Error;
use std::io::{self, Write};
use std::fmt;

mod id;
mod attr;

pub use id::{Id, StrId};


#[derive(Error, Debug, Clone)]
pub enum DotError {
  #[error("string is not a valid DOT id")]
  InvalidStrId
}


pub type DotResult<T> = Result<T, DotError>;

pub struct Node<'a, G: GraphComponent> {
  attr_written: bool,
  parent: &'a mut G,
}

pub struct Edge<'a, G: GraphComponent> {
  attr_written: bool,
  parent: &'a mut G,
}

pub struct Subgraph<'a, G: GraphComponent> {
  indent: Indent,
  parent: &'a mut G,
}

#[must_use = "call .finish()"]
pub struct Graph<W> {
  result: io::Result<()>,
  directed: bool,
  strict: bool,
  writer: W,
}


#[derive(Debug, Default, Clone)]
pub struct GraphBuilder {
  directed: bool,
  strict: bool,
}

impl GraphBuilder {
  pub fn directed(mut self) -> Self {
    self.directed = true;
    self
  }

  pub fn strict(mut self, on: bool) -> Self {
    self.strict = on;
    self
  }

  pub fn create<W: Write>(self, name: impl Id, writer: W) -> io::Result<Graph<W>> {
    let GraphBuilder { directed, strict } = self;
    let mut g = Graph {
      directed,
      strict,
      result: Ok(()),
      writer,
    };
    g.init(name)?;
    Ok(g)
  }
}

impl Graph<()> {
  pub fn new() -> GraphBuilder { GraphBuilder::default() }
}

impl<W: Write> Graph<W> {
  fn init(&mut self, name: impl Id) -> io::Result<()> {
    let graph = if self.directed { "digraph" } else { "graph" };
    let strict = if self.strict { "strict " } else { "" };
    writeln!(self.writer, "{}{} {} {{", strict, graph, name)
  }

  pub fn finish(mut self) -> io::Result<()> {
    writeln!(self.writer, "}}")
  }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Indent(u16);

impl fmt::Display for Indent {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    use fmt::Write;
    for _ in 0..(self.0 * 2) {
      f.write_char(' ')?;
    }
    Ok(())
  }
}

pub trait GraphComponent: Sized {
  type Writer: Write;

  fn root(&mut self) -> &mut Graph<Self::Writer>;

  fn indent(&self) -> Indent;

  fn add_node<'a>(&'a mut self, id: impl Id) -> io::Result<Node<'a, Self>> {
    let i = self.indent();
    let g = self.root();
    write!(g.writer, "{}{}", i, id)?;
    Ok(Node {
      parent: self,
      attr_written: false,
    })
  }
  fn add_edge<'a>(&'a mut self, from: impl Id, to: impl Id) -> io::Result<Edge<'a, Self>> {
    let i = self.indent();
    let g = self.root();
    let edge_op = if g.directed { "->" } else { "--" };
    write!(g.writer, "{}{} {} {}", i, from, edge_op, to)?;
    Ok(Edge {
      parent: self,
      attr_written: false,
    })
  }

  fn add_subgraph<'a>(&'a mut self, id: impl Id) -> io::Result<Subgraph<'a, Self>> {
    let i = self.indent();
    let g = self.root();
    writeln!(g.writer, "{}subgraph {} {{", i, id)?;
    Ok(Subgraph {
      indent: Indent(self.indent().0 + 1),
      parent: self,
    })
  }
}


impl<W: io::Write> GraphComponent for Graph<W> {
  type Writer = W;

  fn indent(&self) -> Indent { Indent(1) }

  fn root(&mut self) -> &mut Graph<Self::Writer> {
    self
  }
}

impl<G: GraphComponent> GraphComponent for Subgraph<'_, G> {
  type Writer = G::Writer;
  fn indent(&self) -> Indent { self.indent }

  fn root(&mut self) -> &mut Graph<Self::Writer> {
    self.parent.root()
  }
}


impl<G: GraphComponent> Drop for Subgraph<'_, G> {
  fn drop(&mut self) {
    let i = self.parent.indent();
    let g = self.root();
    let result = writeln!(g.writer, "{}}}", i);
    g.result = result;
  }
}

impl<G: GraphComponent> Node<'_, G> {
  pub fn set_attr(mut self, attr: &str, val: impl fmt::Display) -> io::Result<Self> {
    let g = self.parent.root();
    let prefix = if !self.attr_written { '[' } else { ',' };
    write!(g.writer, "{}{}={}", prefix, attr, val)?;
    self.attr_written = true;
    Ok(self)
  }
}

impl<G: GraphComponent> Edge<'_, G> {
  pub fn set_attr(mut self, attr: &str, val: impl fmt::Display) -> io::Result<Self> {
    let g = self.parent.root();
    let prefix = if !self.attr_written { '[' } else { ',' };
    write!(g.writer, " {}{}={}", prefix, attr, val)?;
    self.attr_written = true;
    Ok(self)
  }
}

impl<G: GraphComponent> Subgraph<'_, G> {
  pub fn set_attr(mut self, attr: &str, val: impl fmt::Display) -> io::Result<Self> {
    let i = self.indent();
    let g = self.parent.root();
    writeln!(g.writer, "{}{}={};",i, attr, val)?;
    Ok(self)
  }
}


impl<G: GraphComponent> Drop for Node<'_, G> {
  fn drop(&mut self) {
    let g = self.parent.root();
    let result = if self.attr_written {
      writeln!(g.writer, "];")
    } else {
      writeln!(g.writer, ";")
    };
    g.result = result;
  }
}

impl<G: GraphComponent> Drop for Edge<'_, G> {
  fn drop(&mut self) {
    let g = self.parent.root();
    let result = if self.attr_written {
      writeln!(g.writer, "];")
    } else {
      writeln!(g.writer, ";")
    };
    g.result = result;
  }
}