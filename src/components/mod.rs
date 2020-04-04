use svg::parser::Error;
use svg::node::element::Group;

pub(crate) mod bar;
pub(crate) mod axis;
pub(crate) mod scatter;
pub(crate) mod line;

/// A trait that defines behavior of chart components.
pub trait DatumRepresentation {
    fn to_svg(&self) -> Result<Group, Error>;
}