use std::io;
use gvdot::{GraphComponent, StrId};


fn main() -> io::Result<()> {
  let mut buf = Vec::new();

  let mut g = gvdot::Graph::new()
    .directed()
    .strict(false)
    .create(0, &mut buf)?;

  g.add_node(1)?
    .set_attr("color", "gray")?;

  g.add_node(2)?
    .set_attr("color", "blue")?;

  {
    let mut s = g.add_subgraph(100)?;

    s.add_node(StrId::new("foo").unwrap())?
      .set_attr("label", "Hello there!")?;

    let mut ss = s.add_subgraph(1000)?
      .set_attr("rank", "same")?;

    ss.add_node(420)?;
    ss.add_node(421)?;
    ss.add_edge(420, 421)?.set_attr("label", "oh no")?;
  }

  g.add_edge(1, 2)?;
  g.finish()?;

  let x = std::str::from_utf8(&buf).unwrap();

  println!("{}", x);
  Ok(())
}