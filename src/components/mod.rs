use svg::node::element::Group;

pub(crate) mod bar;
pub(crate) mod axis;
pub(crate) mod scatter;
pub(crate) mod line;
pub(crate) mod legend;
pub(crate) mod area;

/// A trait that defines behavior of chart components.
pub trait DatumRepresentation {
    fn to_svg(&self) -> Result<Group, String>;
}