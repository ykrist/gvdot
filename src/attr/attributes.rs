use super::*;

#[derive(Copy, Clone, Default, Debug, Hash, Eq, PartialEq)]
pub struct Background;
impl AttributeName for Background {
    fn name() -> &'static str { "_background" } 
}
impl Attribute<&str> for Background {}

#[derive(Copy, Clone, Default, Debug, Hash, Eq, PartialEq)]
pub struct Area;
impl AttributeName for Area {
    fn name() -> &'static str { "area" } 
}
impl Attribute<f64> for Area {}

#[derive(Copy, Clone, Default, Debug, Hash, Eq, PartialEq)]
pub struct ArrowHead;
impl AttributeName for ArrowHead {
    fn name() -> &'static str { "arrowhead" } 
}
impl Attribute<[crate::val::ArrowShape; 1]> for ArrowHead {}
impl Attribute<[crate::val::ArrowShape; 2]> for ArrowHead {}
impl Attribute<[crate::val::ArrowShape; 3]> for ArrowHead {}
impl Attribute<[crate::val::ArrowShape; 4]> for ArrowHead {}
impl Attribute<crate::val::ArrowShape> for ArrowHead {}

#[derive(Copy, Clone, Default, Debug, Hash, Eq, PartialEq)]
pub struct ArrowSize;
impl AttributeName for ArrowSize {
    fn name() -> &'static str { "arrowsize" } 
}
impl Attribute<f64> for ArrowSize {}

#[derive(Copy, Clone, Default, Debug, Hash, Eq, PartialEq)]
pub struct ArrowTail;
impl AttributeName for ArrowTail {
    fn name() -> &'static str { "arrowtail" } 
}
impl Attribute<[crate::val::ArrowShape; 1]> for ArrowTail {}
impl Attribute<[crate::val::ArrowShape; 2]> for ArrowTail {}
impl Attribute<[crate::val::ArrowShape; 3]> for ArrowTail {}
impl Attribute<[crate::val::ArrowShape; 4]> for ArrowTail {}
impl Attribute<crate::val::ArrowShape> for ArrowTail {}

#[derive(Copy, Clone, Default, Debug, Hash, Eq, PartialEq)]
pub struct BgColor;
impl AttributeName for BgColor {
    fn name() -> &'static str { "bgcolor" } 
}
impl Attribute<&str> for BgColor {}

#[derive(Copy, Clone, Default, Debug, Hash, Eq, PartialEq)]
pub struct Center;
impl AttributeName for Center {
    fn name() -> &'static str { "center" } 
}
impl Attribute<bool> for Center {}

#[derive(Copy, Clone, Default, Debug, Hash, Eq, PartialEq)]
pub struct Charset;
impl AttributeName for Charset {
    fn name() -> &'static str { "charset" } 
}
impl Attribute<&str> for Charset {}

#[derive(Copy, Clone, Default, Debug, Hash, Eq, PartialEq)]
pub struct Class;
impl AttributeName for Class {
    fn name() -> &'static str { "class" } 
}
impl Attribute<&str> for Class {}

#[derive(Copy, Clone, Default, Debug, Hash, Eq, PartialEq)]
pub struct ClusterRank;
impl AttributeName for ClusterRank {
    fn name() -> &'static str { "clusterrank" } 
}
impl Attribute<crate::val::ClusterMode> for ClusterRank {}

#[derive(Copy, Clone, Default, Debug, Hash, Eq, PartialEq)]
pub struct Color;
impl AttributeName for Color {
    fn name() -> &'static str { "color" } 
}
impl Attribute<&str> for Color {}

#[derive(Copy, Clone, Default, Debug, Hash, Eq, PartialEq)]
pub struct ColorScheme;
impl AttributeName for ColorScheme {
    fn name() -> &'static str { "colorscheme" } 
}
impl Attribute<&str> for ColorScheme {}

#[derive(Copy, Clone, Default, Debug, Hash, Eq, PartialEq)]
pub struct Comment;
impl AttributeName for Comment {
    fn name() -> &'static str { "comment" } 
}
impl Attribute<&str> for Comment {}

#[derive(Copy, Clone, Default, Debug, Hash, Eq, PartialEq)]
pub struct Compound;
impl AttributeName for Compound {
    fn name() -> &'static str { "compound" } 
}
impl Attribute<bool> for Compound {}

#[derive(Copy, Clone, Default, Debug, Hash, Eq, PartialEq)]
pub struct Concentrate;
impl AttributeName for Concentrate {
    fn name() -> &'static str { "concentrate" } 
}
impl Attribute<bool> for Concentrate {}

#[derive(Copy, Clone, Default, Debug, Hash, Eq, PartialEq)]
pub struct Constraint;
impl AttributeName for Constraint {
    fn name() -> &'static str { "constraint" } 
}
impl Attribute<bool> for Constraint {}

#[derive(Copy, Clone, Default, Debug, Hash, Eq, PartialEq)]
pub struct Damping;
impl AttributeName for Damping {
    fn name() -> &'static str { "Damping" } 
}
impl Attribute<f64> for Damping {}

#[derive(Copy, Clone, Default, Debug, Hash, Eq, PartialEq)]
pub struct Decorate;
impl AttributeName for Decorate {
    fn name() -> &'static str { "decorate" } 
}
impl Attribute<bool> for Decorate {}

#[derive(Copy, Clone, Default, Debug, Hash, Eq, PartialEq)]
pub struct DefaultDist;
impl AttributeName for DefaultDist {
    fn name() -> &'static str { "defaultdist" } 
}
impl Attribute<f64> for DefaultDist {}

#[derive(Copy, Clone, Default, Debug, Hash, Eq, PartialEq)]
pub struct Dim;
impl AttributeName for Dim {
    fn name() -> &'static str { "dim" } 
}
impl Attribute<i16> for Dim {}
impl Attribute<i32> for Dim {}
impl Attribute<i64> for Dim {}
impl Attribute<i8> for Dim {}
impl Attribute<isize> for Dim {}
impl Attribute<u16> for Dim {}
impl Attribute<u32> for Dim {}
impl Attribute<u64> for Dim {}
impl Attribute<u8> for Dim {}
impl Attribute<usize> for Dim {}

#[derive(Copy, Clone, Default, Debug, Hash, Eq, PartialEq)]
pub struct Dimen;
impl AttributeName for Dimen {
    fn name() -> &'static str { "dimen" } 
}
impl Attribute<i16> for Dimen {}
impl Attribute<i32> for Dimen {}
impl Attribute<i64> for Dimen {}
impl Attribute<i8> for Dimen {}
impl Attribute<isize> for Dimen {}
impl Attribute<u16> for Dimen {}
impl Attribute<u32> for Dimen {}
impl Attribute<u64> for Dimen {}
impl Attribute<u8> for Dimen {}
impl Attribute<usize> for Dimen {}

#[derive(Copy, Clone, Default, Debug, Hash, Eq, PartialEq)]
pub struct Dir;
impl AttributeName for Dir {
    fn name() -> &'static str { "dir" } 
}
impl Attribute<crate::val::DirType> for Dir {}

#[derive(Copy, Clone, Default, Debug, Hash, Eq, PartialEq)]
pub struct DirEdgeConstraints;
impl AttributeName for DirEdgeConstraints {
    fn name() -> &'static str { "diredgeconstraints" } 
}
impl Attribute<&str> for DirEdgeConstraints {}
impl Attribute<bool> for DirEdgeConstraints {}

#[derive(Copy, Clone, Default, Debug, Hash, Eq, PartialEq)]
pub struct DistOrtion;
impl AttributeName for DistOrtion {
    fn name() -> &'static str { "distortion" } 
}
impl Attribute<f64> for DistOrtion {}

#[derive(Copy, Clone, Default, Debug, Hash, Eq, PartialEq)]
pub struct Dpi;
impl AttributeName for Dpi {
    fn name() -> &'static str { "dpi" } 
}
impl Attribute<f64> for Dpi {}

#[derive(Copy, Clone, Default, Debug, Hash, Eq, PartialEq)]
pub struct EdgeHref;
impl AttributeName for EdgeHref {
    fn name() -> &'static str { "edgehref" } 
}
impl Attribute<&str> for EdgeHref {}

#[derive(Copy, Clone, Default, Debug, Hash, Eq, PartialEq)]
pub struct EdgeTarget;
impl AttributeName for EdgeTarget {
    fn name() -> &'static str { "edgetarget" } 
}
impl Attribute<&str> for EdgeTarget {}

#[derive(Copy, Clone, Default, Debug, Hash, Eq, PartialEq)]
pub struct EdgeTooltip;
impl AttributeName for EdgeTooltip {
    fn name() -> &'static str { "edgetooltip" } 
}
impl Attribute<&str> for EdgeTooltip {}

#[derive(Copy, Clone, Default, Debug, Hash, Eq, PartialEq)]
pub struct EdgeUrl;
impl AttributeName for EdgeUrl {
    fn name() -> &'static str { "edgeURL" } 
}
impl Attribute<&str> for EdgeUrl {}

#[derive(Copy, Clone, Default, Debug, Hash, Eq, PartialEq)]
pub struct Epsilon;
impl AttributeName for Epsilon {
    fn name() -> &'static str { "epsilon" } 
}
impl Attribute<f64> for Epsilon {}

#[derive(Copy, Clone, Default, Debug, Hash, Eq, PartialEq)]
pub struct FillColor;
impl AttributeName for FillColor {
    fn name() -> &'static str { "fillcolor" } 
}
impl Attribute<&str> for FillColor {}

#[derive(Copy, Clone, Default, Debug, Hash, Eq, PartialEq)]
pub struct Fixedsize;
impl AttributeName for Fixedsize {
    fn name() -> &'static str { "fixedsize" } 
}
impl Attribute<&str> for Fixedsize {}
impl Attribute<bool> for Fixedsize {}

#[derive(Copy, Clone, Default, Debug, Hash, Eq, PartialEq)]
pub struct FontColor;
impl AttributeName for FontColor {
    fn name() -> &'static str { "fontcolor" } 
}
impl Attribute<&str> for FontColor {}

#[derive(Copy, Clone, Default, Debug, Hash, Eq, PartialEq)]
pub struct FontName;
impl AttributeName for FontName {
    fn name() -> &'static str { "fontname" } 
}
impl Attribute<&str> for FontName {}

#[derive(Copy, Clone, Default, Debug, Hash, Eq, PartialEq)]
pub struct FontNames;
impl AttributeName for FontNames {
    fn name() -> &'static str { "fontnames" } 
}
impl Attribute<&str> for FontNames {}

#[derive(Copy, Clone, Default, Debug, Hash, Eq, PartialEq)]
pub struct FontPath;
impl AttributeName for FontPath {
    fn name() -> &'static str { "fontpath" } 
}
impl Attribute<&str> for FontPath {}

#[derive(Copy, Clone, Default, Debug, Hash, Eq, PartialEq)]
pub struct FontSize;
impl AttributeName for FontSize {
    fn name() -> &'static str { "fontsize" } 
}
impl Attribute<f64> for FontSize {}

#[derive(Copy, Clone, Default, Debug, Hash, Eq, PartialEq)]
pub struct ForceLabelS;
impl AttributeName for ForceLabelS {
    fn name() -> &'static str { "forcelabels" } 
}
impl Attribute<bool> for ForceLabelS {}

#[derive(Copy, Clone, Default, Debug, Hash, Eq, PartialEq)]
pub struct Gradientangle;
impl AttributeName for Gradientangle {
    fn name() -> &'static str { "gradientangle" } 
}
impl Attribute<i16> for Gradientangle {}
impl Attribute<i32> for Gradientangle {}
impl Attribute<i64> for Gradientangle {}
impl Attribute<i8> for Gradientangle {}
impl Attribute<isize> for Gradientangle {}
impl Attribute<u16> for Gradientangle {}
impl Attribute<u32> for Gradientangle {}
impl Attribute<u64> for Gradientangle {}
impl Attribute<u8> for Gradientangle {}
impl Attribute<usize> for Gradientangle {}

#[derive(Copy, Clone, Default, Debug, Hash, Eq, PartialEq)]
pub struct Group;
impl AttributeName for Group {
    fn name() -> &'static str { "group" } 
}
impl Attribute<&str> for Group {}

#[derive(Copy, Clone, Default, Debug, Hash, Eq, PartialEq)]
pub struct HeadClip;
impl AttributeName for HeadClip {
    fn name() -> &'static str { "headclip" } 
}
impl Attribute<bool> for HeadClip {}

#[derive(Copy, Clone, Default, Debug, Hash, Eq, PartialEq)]
pub struct HeadHref;
impl AttributeName for HeadHref {
    fn name() -> &'static str { "headhref" } 
}
impl Attribute<&str> for HeadHref {}

#[derive(Copy, Clone, Default, Debug, Hash, Eq, PartialEq)]
pub struct HeadLabel;
impl AttributeName for HeadLabel {
    fn name() -> &'static str { "headlabel" } 
}
impl Attribute<&str> for HeadLabel {}
impl Attribute<crate::attr::Html<'_>> for HeadLabel {}

#[derive(Copy, Clone, Default, Debug, Hash, Eq, PartialEq)]
pub struct HeadTarget;
impl AttributeName for HeadTarget {
    fn name() -> &'static str { "headtarget" } 
}
impl Attribute<&str> for HeadTarget {}

#[derive(Copy, Clone, Default, Debug, Hash, Eq, PartialEq)]
pub struct HeadTooltip;
impl AttributeName for HeadTooltip {
    fn name() -> &'static str { "headtooltip" } 
}
impl Attribute<&str> for HeadTooltip {}

#[derive(Copy, Clone, Default, Debug, Hash, Eq, PartialEq)]
pub struct HeadUrl;
impl AttributeName for HeadUrl {
    fn name() -> &'static str { "headURL" } 
}
impl Attribute<&str> for HeadUrl {}

#[derive(Copy, Clone, Default, Debug, Hash, Eq, PartialEq)]
pub struct Height;
impl AttributeName for Height {
    fn name() -> &'static str { "height" } 
}
impl Attribute<f64> for Height {}

#[derive(Copy, Clone, Default, Debug, Hash, Eq, PartialEq)]
pub struct Href;
impl AttributeName for Href {
    fn name() -> &'static str { "href" } 
}
impl Attribute<&str> for Href {}

#[derive(Copy, Clone, Default, Debug, Hash, Eq, PartialEq)]
pub struct Id;
impl AttributeName for Id {
    fn name() -> &'static str { "id" } 
}
impl Attribute<&str> for Id {}

#[derive(Copy, Clone, Default, Debug, Hash, Eq, PartialEq)]
pub struct Image;
impl AttributeName for Image {
    fn name() -> &'static str { "image" } 
}
impl Attribute<&str> for Image {}

#[derive(Copy, Clone, Default, Debug, Hash, Eq, PartialEq)]
pub struct Imagepath;
impl AttributeName for Imagepath {
    fn name() -> &'static str { "imagepath" } 
}
impl Attribute<&str> for Imagepath {}

#[derive(Copy, Clone, Default, Debug, Hash, Eq, PartialEq)]
pub struct Imagepos;
impl AttributeName for Imagepos {
    fn name() -> &'static str { "imagepos" } 
}
impl Attribute<&str> for Imagepos {}

#[derive(Copy, Clone, Default, Debug, Hash, Eq, PartialEq)]
pub struct Imagescale;
impl AttributeName for Imagescale {
    fn name() -> &'static str { "imagescale" } 
}
impl Attribute<&str> for Imagescale {}
impl Attribute<bool> for Imagescale {}

#[derive(Copy, Clone, Default, Debug, Hash, Eq, PartialEq)]
pub struct Inputscale;
impl AttributeName for Inputscale {
    fn name() -> &'static str { "inputscale" } 
}
impl Attribute<f64> for Inputscale {}

#[derive(Copy, Clone, Default, Debug, Hash, Eq, PartialEq)]
pub struct K;
impl AttributeName for K {
    fn name() -> &'static str { "K" } 
}
impl Attribute<f64> for K {}

#[derive(Copy, Clone, Default, Debug, Hash, Eq, PartialEq)]
pub struct Label;
impl AttributeName for Label {
    fn name() -> &'static str { "label" } 
}
impl Attribute<&str> for Label {}
impl Attribute<crate::attr::Html<'_>> for Label {}

#[derive(Copy, Clone, Default, Debug, Hash, Eq, PartialEq)]
pub struct LabelScheme;
impl AttributeName for LabelScheme {
    fn name() -> &'static str { "label_scheme" } 
}
impl Attribute<i16> for LabelScheme {}
impl Attribute<i32> for LabelScheme {}
impl Attribute<i64> for LabelScheme {}
impl Attribute<i8> for LabelScheme {}
impl Attribute<isize> for LabelScheme {}
impl Attribute<u16> for LabelScheme {}
impl Attribute<u32> for LabelScheme {}
impl Attribute<u64> for LabelScheme {}
impl Attribute<u8> for LabelScheme {}
impl Attribute<usize> for LabelScheme {}

#[derive(Copy, Clone, Default, Debug, Hash, Eq, PartialEq)]
pub struct LabelAngle;
impl AttributeName for LabelAngle {
    fn name() -> &'static str { "labelangle" } 
}
impl Attribute<f64> for LabelAngle {}

#[derive(Copy, Clone, Default, Debug, Hash, Eq, PartialEq)]
pub struct LabelDistAnce;
impl AttributeName for LabelDistAnce {
    fn name() -> &'static str { "labeldistance" } 
}
impl Attribute<f64> for LabelDistAnce {}

#[derive(Copy, Clone, Default, Debug, Hash, Eq, PartialEq)]
pub struct LabelFloat;
impl AttributeName for LabelFloat {
    fn name() -> &'static str { "labelfloat" } 
}
impl Attribute<bool> for LabelFloat {}

#[derive(Copy, Clone, Default, Debug, Hash, Eq, PartialEq)]
pub struct LabelFontColor;
impl AttributeName for LabelFontColor {
    fn name() -> &'static str { "labelfontcolor" } 
}
impl Attribute<&str> for LabelFontColor {}

#[derive(Copy, Clone, Default, Debug, Hash, Eq, PartialEq)]
pub struct LabelFontName;
impl AttributeName for LabelFontName {
    fn name() -> &'static str { "labelfontname" } 
}
impl Attribute<&str> for LabelFontName {}

#[derive(Copy, Clone, Default, Debug, Hash, Eq, PartialEq)]
pub struct LabelFontSize;
impl AttributeName for LabelFontSize {
    fn name() -> &'static str { "labelfontsize" } 
}
impl Attribute<f64> for LabelFontSize {}

#[derive(Copy, Clone, Default, Debug, Hash, Eq, PartialEq)]
pub struct LabelHref;
impl AttributeName for LabelHref {
    fn name() -> &'static str { "labelhref" } 
}
impl Attribute<&str> for LabelHref {}

#[derive(Copy, Clone, Default, Debug, Hash, Eq, PartialEq)]
pub struct LabelJust;
impl AttributeName for LabelJust {
    fn name() -> &'static str { "labeljust" } 
}
impl Attribute<&str> for LabelJust {}

#[derive(Copy, Clone, Default, Debug, Hash, Eq, PartialEq)]
pub struct LabelLoc;
impl AttributeName for LabelLoc {
    fn name() -> &'static str { "labelloc" } 
}
impl Attribute<&str> for LabelLoc {}

#[derive(Copy, Clone, Default, Debug, Hash, Eq, PartialEq)]
pub struct LabelTarget;
impl AttributeName for LabelTarget {
    fn name() -> &'static str { "labeltarget" } 
}
impl Attribute<&str> for LabelTarget {}

#[derive(Copy, Clone, Default, Debug, Hash, Eq, PartialEq)]
pub struct LabelTooltip;
impl AttributeName for LabelTooltip {
    fn name() -> &'static str { "labeltooltip" } 
}
impl Attribute<&str> for LabelTooltip {}

#[derive(Copy, Clone, Default, Debug, Hash, Eq, PartialEq)]
pub struct LabelUrl;
impl AttributeName for LabelUrl {
    fn name() -> &'static str { "labelURL" } 
}
impl Attribute<&str> for LabelUrl {}

#[derive(Copy, Clone, Default, Debug, Hash, Eq, PartialEq)]
pub struct Landscape;
impl AttributeName for Landscape {
    fn name() -> &'static str { "landscape" } 
}
impl Attribute<bool> for Landscape {}

#[derive(Copy, Clone, Default, Debug, Hash, Eq, PartialEq)]
pub struct Layerlistsep;
impl AttributeName for Layerlistsep {
    fn name() -> &'static str { "layerlistsep" } 
}
impl Attribute<&str> for Layerlistsep {}

#[derive(Copy, Clone, Default, Debug, Hash, Eq, PartialEq)]
pub struct Layersep;
impl AttributeName for Layersep {
    fn name() -> &'static str { "layersep" } 
}
impl Attribute<&str> for Layersep {}

#[derive(Copy, Clone, Default, Debug, Hash, Eq, PartialEq)]
pub struct Layout;
impl AttributeName for Layout {
    fn name() -> &'static str { "layout" } 
}
impl Attribute<&str> for Layout {}

#[derive(Copy, Clone, Default, Debug, Hash, Eq, PartialEq)]
pub struct Len;
impl AttributeName for Len {
    fn name() -> &'static str { "len" } 
}
impl Attribute<f64> for Len {}

#[derive(Copy, Clone, Default, Debug, Hash, Eq, PartialEq)]
pub struct Levels;
impl AttributeName for Levels {
    fn name() -> &'static str { "levels" } 
}
impl Attribute<i16> for Levels {}
impl Attribute<i32> for Levels {}
impl Attribute<i64> for Levels {}
impl Attribute<i8> for Levels {}
impl Attribute<isize> for Levels {}
impl Attribute<u16> for Levels {}
impl Attribute<u32> for Levels {}
impl Attribute<u64> for Levels {}
impl Attribute<u8> for Levels {}
impl Attribute<usize> for Levels {}

#[derive(Copy, Clone, Default, Debug, Hash, Eq, PartialEq)]
pub struct Levelsgap;
impl AttributeName for Levelsgap {
    fn name() -> &'static str { "levelsgap" } 
}
impl Attribute<f64> for Levelsgap {}

#[derive(Copy, Clone, Default, Debug, Hash, Eq, PartialEq)]
pub struct LHead;
impl AttributeName for LHead {
    fn name() -> &'static str { "lhead" } 
}
impl Attribute<&str> for LHead {}

#[derive(Copy, Clone, Default, Debug, Hash, Eq, PartialEq)]
pub struct Lheight;
impl AttributeName for Lheight {
    fn name() -> &'static str { "lheight" } 
}
impl Attribute<f64> for Lheight {}

#[derive(Copy, Clone, Default, Debug, Hash, Eq, PartialEq)]
pub struct LTail;
impl AttributeName for LTail {
    fn name() -> &'static str { "ltail" } 
}
impl Attribute<&str> for LTail {}

#[derive(Copy, Clone, Default, Debug, Hash, Eq, PartialEq)]
pub struct LWidth;
impl AttributeName for LWidth {
    fn name() -> &'static str { "lwidth" } 
}
impl Attribute<f64> for LWidth {}

#[derive(Copy, Clone, Default, Debug, Hash, Eq, PartialEq)]
pub struct Margin;
impl AttributeName for Margin {
    fn name() -> &'static str { "margin" } 
}
impl Attribute<f64> for Margin {}

#[derive(Copy, Clone, Default, Debug, Hash, Eq, PartialEq)]
pub struct Maxiter;
impl AttributeName for Maxiter {
    fn name() -> &'static str { "maxiter" } 
}
impl Attribute<i16> for Maxiter {}
impl Attribute<i32> for Maxiter {}
impl Attribute<i64> for Maxiter {}
impl Attribute<i8> for Maxiter {}
impl Attribute<isize> for Maxiter {}
impl Attribute<u16> for Maxiter {}
impl Attribute<u32> for Maxiter {}
impl Attribute<u64> for Maxiter {}
impl Attribute<u8> for Maxiter {}
impl Attribute<usize> for Maxiter {}

#[derive(Copy, Clone, Default, Debug, Hash, Eq, PartialEq)]
pub struct Mclimit;
impl AttributeName for Mclimit {
    fn name() -> &'static str { "mclimit" } 
}
impl Attribute<f64> for Mclimit {}

#[derive(Copy, Clone, Default, Debug, Hash, Eq, PartialEq)]
pub struct MinDist;
impl AttributeName for MinDist {
    fn name() -> &'static str { "mindist" } 
}
impl Attribute<f64> for MinDist {}

#[derive(Copy, Clone, Default, Debug, Hash, Eq, PartialEq)]
pub struct Minlen;
impl AttributeName for Minlen {
    fn name() -> &'static str { "minlen" } 
}
impl Attribute<i16> for Minlen {}
impl Attribute<i32> for Minlen {}
impl Attribute<i64> for Minlen {}
impl Attribute<i8> for Minlen {}
impl Attribute<isize> for Minlen {}
impl Attribute<u16> for Minlen {}
impl Attribute<u32> for Minlen {}
impl Attribute<u64> for Minlen {}
impl Attribute<u8> for Minlen {}
impl Attribute<usize> for Minlen {}

#[derive(Copy, Clone, Default, Debug, Hash, Eq, PartialEq)]
pub struct Mode;
impl AttributeName for Mode {
    fn name() -> &'static str { "mode" } 
}
impl Attribute<&str> for Mode {}

#[derive(Copy, Clone, Default, Debug, Hash, Eq, PartialEq)]
pub struct Model;
impl AttributeName for Model {
    fn name() -> &'static str { "model" } 
}
impl Attribute<&str> for Model {}

#[derive(Copy, Clone, Default, Debug, Hash, Eq, PartialEq)]
pub struct Mosek;
impl AttributeName for Mosek {
    fn name() -> &'static str { "mosek" } 
}
impl Attribute<bool> for Mosek {}

#[derive(Copy, Clone, Default, Debug, Hash, Eq, PartialEq)]
pub struct NewRank;
impl AttributeName for NewRank {
    fn name() -> &'static str { "newrank" } 
}
impl Attribute<bool> for NewRank {}

#[derive(Copy, Clone, Default, Debug, Hash, Eq, PartialEq)]
pub struct Nodesep;
impl AttributeName for Nodesep {
    fn name() -> &'static str { "nodesep" } 
}
impl Attribute<f64> for Nodesep {}

#[derive(Copy, Clone, Default, Debug, Hash, Eq, PartialEq)]
pub struct Nojustify;
impl AttributeName for Nojustify {
    fn name() -> &'static str { "nojustify" } 
}
impl Attribute<bool> for Nojustify {}

#[derive(Copy, Clone, Default, Debug, Hash, Eq, PartialEq)]
pub struct Normalize;
impl AttributeName for Normalize {
    fn name() -> &'static str { "normalize" } 
}
impl Attribute<bool> for Normalize {}
impl Attribute<f64> for Normalize {}

#[derive(Copy, Clone, Default, Debug, Hash, Eq, PartialEq)]
pub struct Notranslate;
impl AttributeName for Notranslate {
    fn name() -> &'static str { "notranslate" } 
}
impl Attribute<bool> for Notranslate {}

#[derive(Copy, Clone, Default, Debug, Hash, Eq, PartialEq)]
pub struct Nslimit;
impl AttributeName for Nslimit {
    fn name() -> &'static str { "nslimit" } 
}
impl Attribute<f64> for Nslimit {}

#[derive(Copy, Clone, Default, Debug, Hash, Eq, PartialEq)]
pub struct Nslimit1;
impl AttributeName for Nslimit1 {
    fn name() -> &'static str { "nslimit1" } 
}
impl Attribute<f64> for Nslimit1 {}

#[derive(Copy, Clone, Default, Debug, Hash, Eq, PartialEq)]
pub struct Ordering;
impl AttributeName for Ordering {
    fn name() -> &'static str { "ordering" } 
}
impl Attribute<&str> for Ordering {}

#[derive(Copy, Clone, Default, Debug, Hash, Eq, PartialEq)]
pub struct Orientation;
impl AttributeName for Orientation {
    fn name() -> &'static str { "orientation" } 
}
impl Attribute<&str> for Orientation {}
impl Attribute<f64> for Orientation {}

#[derive(Copy, Clone, Default, Debug, Hash, Eq, PartialEq)]
pub struct Overlap;
impl AttributeName for Overlap {
    fn name() -> &'static str { "overlap" } 
}
impl Attribute<&str> for Overlap {}
impl Attribute<bool> for Overlap {}

#[derive(Copy, Clone, Default, Debug, Hash, Eq, PartialEq)]
pub struct OverlapScaling;
impl AttributeName for OverlapScaling {
    fn name() -> &'static str { "overlap_scaling" } 
}
impl Attribute<f64> for OverlapScaling {}

#[derive(Copy, Clone, Default, Debug, Hash, Eq, PartialEq)]
pub struct OverlapShrink;
impl AttributeName for OverlapShrink {
    fn name() -> &'static str { "overlap_shrink" } 
}
impl Attribute<bool> for OverlapShrink {}

#[derive(Copy, Clone, Default, Debug, Hash, Eq, PartialEq)]
pub struct Pack;
impl AttributeName for Pack {
    fn name() -> &'static str { "pack" } 
}
impl Attribute<bool> for Pack {}
impl Attribute<i16> for Pack {}
impl Attribute<i32> for Pack {}
impl Attribute<i64> for Pack {}
impl Attribute<i8> for Pack {}
impl Attribute<isize> for Pack {}
impl Attribute<u16> for Pack {}
impl Attribute<u32> for Pack {}
impl Attribute<u64> for Pack {}
impl Attribute<u8> for Pack {}
impl Attribute<usize> for Pack {}

#[derive(Copy, Clone, Default, Debug, Hash, Eq, PartialEq)]
pub struct Pad;
impl AttributeName for Pad {
    fn name() -> &'static str { "pad" } 
}
impl Attribute<f64> for Pad {}

#[derive(Copy, Clone, Default, Debug, Hash, Eq, PartialEq)]
pub struct Page;
impl AttributeName for Page {
    fn name() -> &'static str { "page" } 
}
impl Attribute<f64> for Page {}

#[derive(Copy, Clone, Default, Debug, Hash, Eq, PartialEq)]
pub struct PenColor;
impl AttributeName for PenColor {
    fn name() -> &'static str { "pencolor" } 
}
impl Attribute<&str> for PenColor {}

#[derive(Copy, Clone, Default, Debug, Hash, Eq, PartialEq)]
pub struct PenWidth;
impl AttributeName for PenWidth {
    fn name() -> &'static str { "penwidth" } 
}
impl Attribute<f64> for PenWidth {}

#[derive(Copy, Clone, Default, Debug, Hash, Eq, PartialEq)]
pub struct Peripheries;
impl AttributeName for Peripheries {
    fn name() -> &'static str { "peripheries" } 
}
impl Attribute<i16> for Peripheries {}
impl Attribute<i32> for Peripheries {}
impl Attribute<i64> for Peripheries {}
impl Attribute<i8> for Peripheries {}
impl Attribute<isize> for Peripheries {}
impl Attribute<u16> for Peripheries {}
impl Attribute<u32> for Peripheries {}
impl Attribute<u64> for Peripheries {}
impl Attribute<u8> for Peripheries {}
impl Attribute<usize> for Peripheries {}

#[derive(Copy, Clone, Default, Debug, Hash, Eq, PartialEq)]
pub struct Pin;
impl AttributeName for Pin {
    fn name() -> &'static str { "pin" } 
}
impl Attribute<bool> for Pin {}

#[derive(Copy, Clone, Default, Debug, Hash, Eq, PartialEq)]
pub struct Quadtree;
impl AttributeName for Quadtree {
    fn name() -> &'static str { "quadtree" } 
}
impl Attribute<bool> for Quadtree {}

#[derive(Copy, Clone, Default, Debug, Hash, Eq, PartialEq)]
pub struct Quantum;
impl AttributeName for Quantum {
    fn name() -> &'static str { "quantum" } 
}
impl Attribute<f64> for Quantum {}

#[derive(Copy, Clone, Default, Debug, Hash, Eq, PartialEq)]
pub struct Rank;
impl AttributeName for Rank {
    fn name() -> &'static str { "rank" } 
}
impl Attribute<crate::val::RankType> for Rank {}

#[derive(Copy, Clone, Default, Debug, Hash, Eq, PartialEq)]
pub struct RankDir;
impl AttributeName for RankDir {
    fn name() -> &'static str { "rankdir" } 
}
impl Attribute<crate::val::RankDir> for RankDir {}

#[derive(Copy, Clone, Default, Debug, Hash, Eq, PartialEq)]
pub struct RankSep;
impl AttributeName for RankSep {
    fn name() -> &'static str { "ranksep" } 
}
impl Attribute<f64> for RankSep {}

#[derive(Copy, Clone, Default, Debug, Hash, Eq, PartialEq)]
pub struct Ratio;
impl AttributeName for Ratio {
    fn name() -> &'static str { "ratio" } 
}
impl Attribute<&str> for Ratio {}
impl Attribute<f64> for Ratio {}

#[derive(Copy, Clone, Default, Debug, Hash, Eq, PartialEq)]
pub struct Regular;
impl AttributeName for Regular {
    fn name() -> &'static str { "regular" } 
}
impl Attribute<bool> for Regular {}

#[derive(Copy, Clone, Default, Debug, Hash, Eq, PartialEq)]
pub struct Remincross;
impl AttributeName for Remincross {
    fn name() -> &'static str { "remincross" } 
}
impl Attribute<bool> for Remincross {}

#[derive(Copy, Clone, Default, Debug, Hash, Eq, PartialEq)]
pub struct RepulsiveForce;
impl AttributeName for RepulsiveForce {
    fn name() -> &'static str { "repulsiveforce" } 
}
impl Attribute<f64> for RepulsiveForce {}

#[derive(Copy, Clone, Default, Debug, Hash, Eq, PartialEq)]
pub struct Resolution;
impl AttributeName for Resolution {
    fn name() -> &'static str { "resolution" } 
}
impl Attribute<f64> for Resolution {}

#[derive(Copy, Clone, Default, Debug, Hash, Eq, PartialEq)]
pub struct Root;
impl AttributeName for Root {
    fn name() -> &'static str { "root" } 
}
impl Attribute<&str> for Root {}
impl Attribute<bool> for Root {}

#[derive(Copy, Clone, Default, Debug, Hash, Eq, PartialEq)]
pub struct Rotate;
impl AttributeName for Rotate {
    fn name() -> &'static str { "rotate" } 
}
impl Attribute<i16> for Rotate {}
impl Attribute<i32> for Rotate {}
impl Attribute<i64> for Rotate {}
impl Attribute<i8> for Rotate {}
impl Attribute<isize> for Rotate {}
impl Attribute<u16> for Rotate {}
impl Attribute<u32> for Rotate {}
impl Attribute<u64> for Rotate {}
impl Attribute<u8> for Rotate {}
impl Attribute<usize> for Rotate {}

#[derive(Copy, Clone, Default, Debug, Hash, Eq, PartialEq)]
pub struct Rotation;
impl AttributeName for Rotation {
    fn name() -> &'static str { "rotation" } 
}
impl Attribute<f64> for Rotation {}

#[derive(Copy, Clone, Default, Debug, Hash, Eq, PartialEq)]
pub struct SameHead;
impl AttributeName for SameHead {
    fn name() -> &'static str { "samehead" } 
}
impl Attribute<&str> for SameHead {}

#[derive(Copy, Clone, Default, Debug, Hash, Eq, PartialEq)]
pub struct SameTail;
impl AttributeName for SameTail {
    fn name() -> &'static str { "sametail" } 
}
impl Attribute<&str> for SameTail {}

#[derive(Copy, Clone, Default, Debug, Hash, Eq, PartialEq)]
pub struct Samplepoints;
impl AttributeName for Samplepoints {
    fn name() -> &'static str { "samplepoints" } 
}
impl Attribute<i16> for Samplepoints {}
impl Attribute<i32> for Samplepoints {}
impl Attribute<i64> for Samplepoints {}
impl Attribute<i8> for Samplepoints {}
impl Attribute<isize> for Samplepoints {}
impl Attribute<u16> for Samplepoints {}
impl Attribute<u32> for Samplepoints {}
impl Attribute<u64> for Samplepoints {}
impl Attribute<u8> for Samplepoints {}
impl Attribute<usize> for Samplepoints {}

#[derive(Copy, Clone, Default, Debug, Hash, Eq, PartialEq)]
pub struct Scale;
impl AttributeName for Scale {
    fn name() -> &'static str { "scale" } 
}
impl Attribute<f64> for Scale {}

#[derive(Copy, Clone, Default, Debug, Hash, Eq, PartialEq)]
pub struct Searchsize;
impl AttributeName for Searchsize {
    fn name() -> &'static str { "searchsize" } 
}
impl Attribute<i16> for Searchsize {}
impl Attribute<i32> for Searchsize {}
impl Attribute<i64> for Searchsize {}
impl Attribute<i8> for Searchsize {}
impl Attribute<isize> for Searchsize {}
impl Attribute<u16> for Searchsize {}
impl Attribute<u32> for Searchsize {}
impl Attribute<u64> for Searchsize {}
impl Attribute<u8> for Searchsize {}
impl Attribute<usize> for Searchsize {}

#[derive(Copy, Clone, Default, Debug, Hash, Eq, PartialEq)]
pub struct Shape;
impl AttributeName for Shape {
    fn name() -> &'static str { "shape" } 
}
impl Attribute<crate::val::Shape> for Shape {}

#[derive(Copy, Clone, Default, Debug, Hash, Eq, PartialEq)]
pub struct Shapefile;
impl AttributeName for Shapefile {
    fn name() -> &'static str { "shapefile" } 
}
impl Attribute<&str> for Shapefile {}

#[derive(Copy, Clone, Default, Debug, Hash, Eq, PartialEq)]
pub struct Showboxes;
impl AttributeName for Showboxes {
    fn name() -> &'static str { "showboxes" } 
}
impl Attribute<i16> for Showboxes {}
impl Attribute<i32> for Showboxes {}
impl Attribute<i64> for Showboxes {}
impl Attribute<i8> for Showboxes {}
impl Attribute<isize> for Showboxes {}
impl Attribute<u16> for Showboxes {}
impl Attribute<u32> for Showboxes {}
impl Attribute<u64> for Showboxes {}
impl Attribute<u8> for Showboxes {}
impl Attribute<usize> for Showboxes {}

#[derive(Copy, Clone, Default, Debug, Hash, Eq, PartialEq)]
pub struct Sides;
impl AttributeName for Sides {
    fn name() -> &'static str { "sides" } 
}
impl Attribute<i16> for Sides {}
impl Attribute<i32> for Sides {}
impl Attribute<i64> for Sides {}
impl Attribute<i8> for Sides {}
impl Attribute<isize> for Sides {}
impl Attribute<u16> for Sides {}
impl Attribute<u32> for Sides {}
impl Attribute<u64> for Sides {}
impl Attribute<u8> for Sides {}
impl Attribute<usize> for Sides {}

#[derive(Copy, Clone, Default, Debug, Hash, Eq, PartialEq)]
pub struct Size;
impl AttributeName for Size {
    fn name() -> &'static str { "size" } 
}
impl Attribute<f64> for Size {}

#[derive(Copy, Clone, Default, Debug, Hash, Eq, PartialEq)]
pub struct Skew;
impl AttributeName for Skew {
    fn name() -> &'static str { "skew" } 
}
impl Attribute<f64> for Skew {}

#[derive(Copy, Clone, Default, Debug, Hash, Eq, PartialEq)]
pub struct Sortv;
impl AttributeName for Sortv {
    fn name() -> &'static str { "sortv" } 
}
impl Attribute<i16> for Sortv {}
impl Attribute<i32> for Sortv {}
impl Attribute<i64> for Sortv {}
impl Attribute<i8> for Sortv {}
impl Attribute<isize> for Sortv {}
impl Attribute<u16> for Sortv {}
impl Attribute<u32> for Sortv {}
impl Attribute<u64> for Sortv {}
impl Attribute<u8> for Sortv {}
impl Attribute<usize> for Sortv {}

#[derive(Copy, Clone, Default, Debug, Hash, Eq, PartialEq)]
pub struct Splines;
impl AttributeName for Splines {
    fn name() -> &'static str { "splines" } 
}
impl Attribute<&str> for Splines {}
impl Attribute<bool> for Splines {}

#[derive(Copy, Clone, Default, Debug, Hash, Eq, PartialEq)]
pub struct Stylesheet;
impl AttributeName for Stylesheet {
    fn name() -> &'static str { "stylesheet" } 
}
impl Attribute<&str> for Stylesheet {}

#[derive(Copy, Clone, Default, Debug, Hash, Eq, PartialEq)]
pub struct TailClip;
impl AttributeName for TailClip {
    fn name() -> &'static str { "tailclip" } 
}
impl Attribute<bool> for TailClip {}

#[derive(Copy, Clone, Default, Debug, Hash, Eq, PartialEq)]
pub struct TailHref;
impl AttributeName for TailHref {
    fn name() -> &'static str { "tailhref" } 
}
impl Attribute<&str> for TailHref {}

#[derive(Copy, Clone, Default, Debug, Hash, Eq, PartialEq)]
pub struct TailLabel;
impl AttributeName for TailLabel {
    fn name() -> &'static str { "taillabel" } 
}
impl Attribute<&str> for TailLabel {}
impl Attribute<crate::attr::Html<'_>> for TailLabel {}

#[derive(Copy, Clone, Default, Debug, Hash, Eq, PartialEq)]
pub struct TailTarget;
impl AttributeName for TailTarget {
    fn name() -> &'static str { "tailtarget" } 
}
impl Attribute<&str> for TailTarget {}

#[derive(Copy, Clone, Default, Debug, Hash, Eq, PartialEq)]
pub struct TailTooltip;
impl AttributeName for TailTooltip {
    fn name() -> &'static str { "tailtooltip" } 
}
impl Attribute<&str> for TailTooltip {}

#[derive(Copy, Clone, Default, Debug, Hash, Eq, PartialEq)]
pub struct TailUrl;
impl AttributeName for TailUrl {
    fn name() -> &'static str { "tailURL" } 
}
impl Attribute<&str> for TailUrl {}

#[derive(Copy, Clone, Default, Debug, Hash, Eq, PartialEq)]
pub struct Target;
impl AttributeName for Target {
    fn name() -> &'static str { "target" } 
}
impl Attribute<&str> for Target {}

#[derive(Copy, Clone, Default, Debug, Hash, Eq, PartialEq)]
pub struct Tooltip;
impl AttributeName for Tooltip {
    fn name() -> &'static str { "tooltip" } 
}
impl Attribute<&str> for Tooltip {}

#[derive(Copy, Clone, Default, Debug, Hash, Eq, PartialEq)]
pub struct TrueColor;
impl AttributeName for TrueColor {
    fn name() -> &'static str { "truecolor" } 
}
impl Attribute<bool> for TrueColor {}

#[derive(Copy, Clone, Default, Debug, Hash, Eq, PartialEq)]
pub struct Url;
impl AttributeName for Url {
    fn name() -> &'static str { "URL" } 
}
impl Attribute<&str> for Url {}

#[derive(Copy, Clone, Default, Debug, Hash, Eq, PartialEq)]
pub struct VoroMargin;
impl AttributeName for VoroMargin {
    fn name() -> &'static str { "voro_margin" } 
}
impl Attribute<f64> for VoroMargin {}

#[derive(Copy, Clone, Default, Debug, Hash, Eq, PartialEq)]
pub struct Weight;
impl AttributeName for Weight {
    fn name() -> &'static str { "weight" } 
}
impl Attribute<f64> for Weight {}
impl Attribute<i16> for Weight {}
impl Attribute<i32> for Weight {}
impl Attribute<i64> for Weight {}
impl Attribute<i8> for Weight {}
impl Attribute<isize> for Weight {}
impl Attribute<u16> for Weight {}
impl Attribute<u32> for Weight {}
impl Attribute<u64> for Weight {}
impl Attribute<u8> for Weight {}
impl Attribute<usize> for Weight {}

#[derive(Copy, Clone, Default, Debug, Hash, Eq, PartialEq)]
pub struct Width;
impl AttributeName for Width {
    fn name() -> &'static str { "width" } 
}
impl Attribute<f64> for Width {}

#[derive(Copy, Clone, Default, Debug, Hash, Eq, PartialEq)]
pub struct Xdotversion;
impl AttributeName for Xdotversion {
    fn name() -> &'static str { "xdotversion" } 
}
impl Attribute<&str> for Xdotversion {}

#[derive(Copy, Clone, Default, Debug, Hash, Eq, PartialEq)]
pub struct XLabel;
impl AttributeName for XLabel {
    fn name() -> &'static str { "xlabel" } 
}
impl Attribute<&str> for XLabel {}
impl Attribute<crate::attr::Html<'_>> for XLabel {}

#[derive(Copy, Clone, Default, Debug, Hash, Eq, PartialEq)]
pub struct Z;
impl AttributeName for Z {
    fn name() -> &'static str { "z" } 
}
impl Attribute<f64> for Z {}
