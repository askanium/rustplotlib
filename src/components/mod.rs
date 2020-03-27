use svg::parser::Error;

pub mod bar;

/// A trait that defines behavior of chart components.
pub trait DatumRepresentation {
    type Node;

    fn to_svg(&self) -> Result<Self::Node, Error>;
}