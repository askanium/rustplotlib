use svg::node::element::Group;
use crate::components::legend::LegendEntry;

pub mod vertical_bar;
pub mod horizontal_bar;
pub mod scatter;
pub mod datum;
pub mod line;
pub mod area;

/// A trait that defines a View of a dataset that can be rendered within a chart.
pub trait View<'a> {
    fn to_svg(&self) -> Result<Group, String>;

    fn get_legend_entries(&self) -> Vec<LegendEntry>;
}
