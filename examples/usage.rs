use std::io;
use gvdot::*;


fn main() -> io::Result<()> {
  let mut g = gvdot::Graph::new()
    .directed()
    .strict(false)
    .in_memory()
    .attr(attr::RankDir, val::RankDir::LR)?;

  g.add_node(1)?
    .attr(attr::Label, attr::html("hello <i>m8</i>"))?
    .attr_raw("color", "gray")?;

  g.modify_nodes()?
    .attr(attr::Shape, val::Shape::Box3d);

  g.add_node(2)?
    .attr(attr::Label, "Node \\N")?
    // .attr_raw("label", attr::quote_str("Node \\N"))?
    .attr(attr::Color, "blue")?;

  {
    let mut s = g.add_subgraph(100)?;

    s.add_node(StrId::new("foo").unwrap())?
      .attr_raw("label", attr::quote("Hello there!"))?;

    let mut ss = s.add_subgraph(1000)?
      .attr_raw("rank", "same")?;

    ss.add_node(420)?;
    ss.add_node(421)?
      .attr_raw("shape", "square")?;
    ss.add_edge(420, 421)?.attr_raw("label", attr::quote("oh no"))?;
  }

  g.add_edge(1, 2)?
    .attr(attr::Weight, 1.01)?
    .attr(attr::ArrowHead, [val::ArrowShape::diamond().open(), val::ArrowShape::crow().left_side()])?;

    // .attr(attr::Arro)

  let x = g.into_string();
  println!("{}", x);
  render_svg(&x, Layout::Dot, "scrap.svg")?;
  std::fs::write("scrap.dot", x);
  Ok(())
}