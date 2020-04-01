use svg::node::element::Group;
use svg::parser::Error;

pub mod vertical_bar;
pub mod horizontal_bar;
pub mod datum;

/// A trait that defines a View of a dataset that can be rendered within a chart.
pub trait View<'a> {
    fn to_svg(&self) -> Result<Group, Error>;
}
