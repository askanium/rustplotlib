use crate::datasets::vertical_bar::VerticalBarDataset;
use crate::components::DatumRepresentation;
use svg::node::element::Group;
use svg::parser::Error;

pub mod vertical_bar;
pub mod datum;

// pub trait Dataset<'a> {
pub trait Dataset<'a> {
    fn to_svg(&self) -> Result<Group, Error>;
}

// impl<'a> Render for dyn Dataset<'a> {
//     type SVGNode = svg::node::element::Group;
//
//     fn to_svg(&self) -> Result<svg::node::element::Group, Error> {
//         let group = svg::node::element::Group::new();
//
//         for entry in self.get_entries() {
//             group.add(entry.to_svg());
//         }
//         Ok(group)
//     }
// }
