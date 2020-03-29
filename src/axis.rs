use svg::node::element::Group;
use svg::parser::Error;
use svg::Node;
use crate::utils::{Range, Orientation};
use crate::Scale;
use crate::components::axis::{AxisLine, AxisTick};

/// A categorical axis struct that represents a specific scale on the X dimension.
pub struct XAxisCategorical<'a> {
    scale: &'a dyn Scale<String>,
    ticks: Vec<AxisTick>,
    axis_line: AxisLine,
}

impl<'a> XAxisCategorical<'a> {
    pub fn new(scale: &'a dyn Scale<String>) -> Self {
        Self {
            scale,
            ticks: scale.get_ticks(Orientation::Horizontal),
            axis_line: AxisLine::new(0f32, 0f32, scale.max_range(), 0f32)
        }
    }

    pub fn to_svg(&self) -> Result<Group, Error> {
        let mut group = Group::new()
            .set("class", "x-axis")
            .add(self.axis_line.to_svg().unwrap());

        for tick in self.ticks.iter() {
            group.append(tick.to_svg().unwrap());
        }

        Ok(group)
    }
}

pub trait XAxis {
    fn to_svg(&self) -> Result<Group, Error>;
}

impl<'a> XAxis for XAxisCategorical<'a> {
    fn to_svg(&self) -> Result<Group, Error> {
        Ok(self.to_svg().unwrap())
    }
}