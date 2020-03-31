use svg::node::element::Group;
use svg::parser::Error;

pub mod vertical_bar;
pub mod datum;

pub trait Dataset<'a> {
    fn to_svg(&self) -> Result<Group, Error>;
}
