use crate::components::legend::LegendEntry;
use svg::node::element::Group;

pub mod area;
pub mod datum;
pub mod horizontal_bar;
pub mod line;
pub mod scatter;
pub mod vertical_bar;

/// A trait that defines a View of a dataset that can be rendered within a chart.
pub trait View<'a> {
    fn to_svg(&self) -> Result<Group, String>;

    fn get_legend_entries(&self) -> Vec<LegendEntry>;
}
