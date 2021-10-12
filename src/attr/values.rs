use strum::*;

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash, Display)]
#[strum(serialize_all="UPPERCASE")]
pub enum RankDir {
  TB,
  BT,
  LR,
  RL,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash, Display)]
#[strum(serialize_all="lowercase")]
pub enum RankType {
  Same,
  Min,
  Max,
  Source,
  Sink,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash, Display)]
#[strum(serialize_all="lowercase")]
pub enum ClusterMode {
  Local,
  Global,
  None,
}


#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash, Display)]
#[strum(serialize_all="lowercase")]
pub enum DirType {
  Forward,
  Backward,
  Both,
  None,
}


#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash, Display)]
#[strum(serialize_all="lowercase")]
pub enum Shape {
  Box,
  Polygon,
  Ellipse,
  Oval,
  Circle,
  Point,
  Egg,
  Triangle,
  Plaintext,
  Plain,
  Diamond,
  Trapezium,
  Parallelogram,
  House,
  Pentagon,
  Hexagon,
  Septagon,
  Octagon,
  Doublecircle,
  Doubleoctagon,
  Tripleoctagon,
  Invtriangle,
  Invtrapezium,
  Invhouse,
  Mdiamond,
  Msquare,
  Mcircle,
  Rect,
  Rectangle,
  Square,
  Star,
  None,
  Underline,
  Cylinder,
  Note,
  Tab,
  Folder,
  Box3d,
  Component,
  Promoter,
  Cds,
  Terminator,
  Utr,
  Primersite,
  Restrictionsite,
  Fivepoverhang,
  Threepoverhang,
  Noverhang,
  Assembly,
  Signature,
  Insulator,
  Ribosite,
  Rnastab,
  Proteasesite,
  Proteinstab,
  Rpromoter,
  Rarrow,
  Larrow,
  Lpromoter,
  Record,
}

pub use arrow::ArrowShape;
mod arrow {
  use std::fmt::{self, Write};
  use super::*;

  #[derive(Copy, Clone, Debug, Hash, Eq, PartialEq, Display)]
  #[strum(serialize_all="lowercase")]
  #[repr(u8)]
  enum Side {
    L,
    R,
  }

  #[derive(Copy, Clone, Debug, Hash, Eq, PartialEq, Display)]
  #[repr(u8)]
  #[strum(serialize_all="lowercase")]
  enum ArrowPrimitiveShape {
    Box,
    Crow,
    Curve,
    Diamond,
    Dot,
    Icurve,
    Inv,
    None,
    Normal,
    Tee,
    Vee,
  }

  #[derive(Copy, Clone, Debug, Hash, Eq, PartialEq)]
  pub struct ArrowShape {
    open: bool,
    side: Option<Side>,
    shp: ArrowPrimitiveShape,
  }

  impl fmt::Display for ArrowShape {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
      if self.open {
        f.write_char('o')?;
      }
      if let Some(s) = self.side.as_ref() {
        fmt::Display::fmt(s, f)?;
      }
      fmt::Display::fmt(&self.shp, f)?;
      Ok(())
    }
  }

  macro_rules! impl_primitive_constructor {
      ($($n:ident, $v:path),+$(,)?) => {
        $(
          pub fn $n() -> Self {
            ArrowShape {
              open: false,
              side: None,
              shp: $v
            }
          }
        )*
      };
    }

  impl ArrowShape {
    pub fn open(mut self) -> Self {
      self.open = true;
      self
    }
    pub fn left_side(mut self) -> Self {
      self.side = Some(Side::L);
      self
    }
    pub fn right_side(mut self) -> Self {
      self.side = Some(Side::R);
      self
    }

    impl_primitive_constructor! {
      box_, ArrowPrimitiveShape::Box,
      crow, ArrowPrimitiveShape::Crow,
      curve, ArrowPrimitiveShape::Curve,
      diamond, ArrowPrimitiveShape::Diamond,
      dot, ArrowPrimitiveShape::Dot,
      icurve, ArrowPrimitiveShape::Icurve,
      inv, ArrowPrimitiveShape::Inv,
      none, ArrowPrimitiveShape::None,
      normal, ArrowPrimitiveShape::Normal,
      tee, ArrowPrimitiveShape::Tee,
      vee, ArrowPrimitiveShape::Vee,
    }

  }
}
//
// impl fmt::Display for RankDir {
//   fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//     let s = match self {
//       RankDir::TB => "TB",
//       RankDir::BT => "BT",
//       RankDir::LR => "LR",
//       RankDir::RL => "RL",
//     };
//     f.write_str(s)
//   }
// }

/*



 */