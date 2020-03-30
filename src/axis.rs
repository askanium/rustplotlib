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

/// A categorical axis struct that represents a specific scale on the X dimension.
pub struct YAxisLinear<'a> {
    scale: &'a dyn Scale<f32>,
    ticks: Vec<AxisTick>,
    axis_line: AxisLine,
}

impl<'a> XAxisCategorical<'a> {
    pub fn new(scale: &'a dyn Scale<String>) -> Self {
        Self {
            scale,
            ticks: Self::generate_ticks(scale),
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

    fn generate_ticks(scale: &'a dyn Scale<String>) -> Vec<AxisTick> {
        let mut ticks = Vec::new();

        for tick in scale.get_ticks() {
            let axis_tick = AxisTick::new(scale.scale(tick.to_string()) + scale.bandwidth().unwrap() / 2_f32, 16, tick, Orientation::Horizontal);
            ticks.push(axis_tick);
        }

        ticks
    }
}

impl<'a> YAxisLinear<'a> {
    pub fn new(scale: &'a dyn Scale<f32>) -> Self {
        Self {
            scale,
            ticks: Self::generate_ticks(scale),
            axis_line: AxisLine::new(0_f32, 0_f32, 0_f32, scale.max_range())
        }
    }

    pub fn to_svg(&self) -> Result<Group, Error> {
        let mut group = Group::new()
            .set("class", "y-axis")
            .add(self.axis_line.to_svg().unwrap());

        for tick in self.ticks.iter() {
            group.append(tick.to_svg().unwrap());
        }

        Ok(group)
    }

    fn generate_ticks(scale: &'a dyn Scale<f32>) -> Vec<AxisTick> {
        let mut ticks = Vec::new();

        for tick in scale.get_ticks() {
            let axis_tick = AxisTick::new(scale.max_range() - scale.scale(tick), 16, tick.to_string(), Orientation::Vertical);
            ticks.push(axis_tick);
        }

        ticks
    }
}

pub trait XAxis {
    fn to_svg(&self) -> Result<Group, Error>;
}

impl<'a> XAxis for XAxisCategorical<'a> {
    fn to_svg(&self) -> Result<Group, Error> {
        self.to_svg()
    }
}

pub trait YAxis {
    fn to_svg(&self) -> Result<Group, Error>;
}

impl<'a> YAxis for YAxisLinear<'a> {
    fn to_svg(&self) -> Result<Group, Error> {
        self.to_svg()
    }
}