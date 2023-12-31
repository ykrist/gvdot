use std::io::{self, Write};
use std::fmt;
use strum::IntoStaticStr;

mod id;
pub mod attr;

#[doc(inline)]
pub use id::{Id, StrId};
#[doc(inline)]
pub use crate::attr::Attribute;
#[doc(inline)]
pub use attr::values as val;


pub struct Node<'a, G: GraphComponent> {
  parent: &'a mut G,
}

pub struct Nodes<'a, G: GraphComponent> {
  parent: &'a mut G,
}

pub struct Edge<'a, G: GraphComponent> {
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

#[derive(Debug)]
pub struct GvPipe {
  pipe: std::process::ChildStdin,
  gv: std::process::Child
}

impl io::Write for GvPipe {
  #[inline(always)]
  fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
    // print!("{}", std::str::from_utf8(buf).unwrap());
    self.pipe.write(buf)
  }
  #[inline(always)]
  fn flush(&mut self) -> io::Result<()> {
    self.pipe.flush()
  }
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


  pub fn in_memory(self) -> Graph<Vec<u8>> {
    self.create(StrId::default(), Vec::new()).unwrap()
  }

  /// Stream the graph directly to a Graphviz layout program
  pub fn stream_to_gv(self, prog: Layout, filepath: impl AsRef<Path>) -> io::Result<Graph<GvPipe>> {
    let mut gv = spawn_gv_proc(prog, filepath)?;
    let pipe = gv.stdin.take().unwrap();
    self.create(StrId::default(), GvPipe{ pipe, gv })
  }
}

impl Graph<()> {
  pub fn new() -> GraphBuilder { GraphBuilder::default() }
}


impl Graph<Vec<u8>> {
  pub fn into_string(mut self) -> String {
    self.write_end().unwrap();
    String::from_utf8(self.writer).unwrap()
  }
}

impl Graph<GvPipe> {
  /// Wait for GraphViz to finish writing the file.
  pub fn wait(mut self) -> io::Result<std::process::ExitStatus> {
    self.write_end()?;
    let GvPipe { mut gv, pipe } = self.writer;
    drop(pipe);
    gv.wait()
  }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash, IntoStaticStr)]
#[strum(serialize_all="lowercase")]
pub enum Layout {
  Dot,
  Neato,
  Twopi,
  Circo,
  Fdp,
  Sfdp,
  Patchwork,
  Osage,
}


fn spawn_gv_proc(prog: Layout, filepath: impl AsRef<Path>) -> io::Result<std::process::Child> {
  use std::process::{Stdio, Command};
  let prog: &'static str = prog.into();
  let filepath = filepath.as_ref();
  Command::new(prog)
    .arg("-T")
    .arg(filepath.extension().expect("filepath must have an extension"))
    .arg("-o")
    .arg(filepath)
    .stdin(Stdio::piped())
    .stdout(Stdio::inherit())
    .stderr(Stdio::inherit())
    .spawn()
}

pub fn render(dot: &str, prog: Layout, filepath: impl AsRef<Path>) -> io::Result<std::process::ExitStatus> {
  let mut gv = spawn_gv_proc(prog, filepath)?;
  gv.stdin.take().unwrap().write(dot.as_bytes())?;
  gv.wait()
}

impl<W: Write> Graph<W> {
  fn init(&mut self, name: impl Id) -> io::Result<()> {
    let graph = if self.directed { "digraph" } else { "graph" };
    let strict = if self.strict { "strict " } else { "" };
    writeln!(self.writer, "{}{} {} {{", strict, graph, name)
  }

  pub fn finish(mut self) -> io::Result<()> {
    self.write_end()
  }

  fn write_end(&mut self) -> io::Result<()> {
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

pub trait GraphComponent: Sized + private::GraphRoot {
  fn indent(&self) -> Indent;


  fn add_node<'a>(&'a mut self, id: impl Id) -> io::Result<Node<'a, Self>> {
    let i = self.indent();
    let g = self.root();
    write!(g.writer, "{}{} [", i, id)?;
    Ok(Node {
      parent: self,
    })
  }

  fn modify_nodes<'a>(&'a mut self) -> io::Result<Nodes<'a, Self>> {
    let i = self.indent();
    let g = self.root();
    write!(g.writer, "{}node [", i)?;
    Ok(Nodes {
      parent: self,
    })
  }

  fn add_edge<'a>(&'a mut self, from: impl Id, to: impl Id) -> io::Result<Edge<'a, Self>> {
    let i = self.indent();
    let g = self.root();
    let edge_op = if g.directed { "->" } else { "--" };
    write!(g.writer, "{}{} {} {} [", i, from, edge_op, to)?;
    Ok(Edge {
      parent: self,
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

use private::{GraphRoot};
use std::path::Path;

mod private {
  use super::*;
  pub trait GraphRoot {
    type Writer: io::Write;

    fn writer(&mut self) -> &mut Self::Writer {
      &mut self.root().writer
    }

    fn root(&mut self) -> &mut Graph<Self::Writer>;
  }


  impl<W: io::Write> GraphRoot for Graph<W> {
    type Writer = W;
    fn root(&mut self) -> &mut Graph<Self::Writer> { self }
  }

  impl<G: GraphComponent> GraphRoot for Subgraph<'_, G> {
    type Writer = G::Writer;
    fn root(&mut self) -> &mut Graph<Self::Writer> { self.parent.root() }
  }


  impl<G: GraphComponent> GraphRoot for Node<'_, G> {
    type Writer = G::Writer;
    fn root(&mut self) -> &mut Graph<Self::Writer> { self.parent.root() }
  }

  impl<G: GraphComponent> GraphRoot for Nodes<'_, G> {
    type Writer = G::Writer;
    fn root(&mut self) -> &mut Graph<Self::Writer> { self.parent.root() }
  }

  impl<G: GraphComponent> GraphRoot for Edge<'_, G> {
    type Writer = G::Writer;
    fn root(&mut self) -> &mut Graph<Self::Writer> { self.parent.root() }
  }


  impl<W: io::Write> GraphComponent for Graph<W> {
    fn indent(&self) -> Indent { Indent(1) }
  }


  impl<G: GraphComponent> GraphComponent for Subgraph<'_, G> {
    fn indent(&self) -> Indent { self.indent }
  }
}

pub trait SetAttribute : Sized + private::GraphRoot {
  /// Set an attribute with static type-checking
  fn attr<A, V>(mut self, _attr: A, val: V) -> io::Result<Self>
    where
      A: Attribute<V>,
      V: attr::PrepareValue
  {
    write!(self.writer(), "{}={},", A::name(), val.prepare())?;
    Ok(self)
  }

  /// Set an attribute without type-checking.  Strings may need to be quoted (see [`quote_str`](crate::attr::quote_str))
  fn attr_raw(mut self, attr: &str, val: impl fmt::Display) -> io::Result<Self> {
    write!(self.writer(), "{}={},", attr, val)?;
    Ok(self)
  }
}

impl<G: GraphComponent> SetAttribute for Node<'_, G> {}
impl<G: GraphComponent> SetAttribute for Nodes<'_, G> {}
impl<G: GraphComponent> SetAttribute for Edge<'_, G> {}

impl<G: GraphComponent> SetAttribute for G {
  fn attr<A, V>(mut self, _attr: A, val: V) -> io::Result<Self>
    where
      A: Attribute<V>,
      V: attr::PrepareValue
  {
    let i = self.indent();
    writeln!(self.writer(), "{}{}={};", i, A::name(), val.prepare())?;
    Ok(self)
  }

  fn attr_raw(mut self, attr: &str, val: impl fmt::Display) -> io::Result<Self> {
    let i = self.indent();
    writeln!(self.writer(), "{}{}={};",i, attr, val)?;
    Ok(self)
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


impl<G: GraphComponent> Drop for Node<'_, G> {
  fn drop(&mut self) {
    let g = self.parent.root();
    g.result = writeln!(g.writer, "];");
  }
}

impl<G: GraphComponent> Drop for Nodes<'_, G> {
  fn drop(&mut self) {
    let g = self.parent.root();
    g.result = writeln!(g.writer, "];");
  }
}

impl<G: GraphComponent> Drop for Edge<'_, G> {
  fn drop(&mut self) {
    let g = self.parent.root();
    g.result = writeln!(g.writer, "];");
  }
}

