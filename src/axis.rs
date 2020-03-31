use std::string::ToString;
use svg::node::element::Group;
use svg::parser::Error;
use svg::Node;
use crate::utils::Orientation;
use crate::{Scale, Chart};
use crate::components::axis::{AxisLine, AxisTick};
use crate::scales::ScaleType;

/// Enum of possible axis positions on the chart.
#[derive(Copy, Clone)]
pub enum AxisPosition {
    Top,
    Right,
    Bottom,
    Left,
}

/// An axis struct that represents an axis along a dimension of the chart.
pub struct Axis {
    ticks: Vec<AxisTick>,
    axis_line: AxisLine,
    position: AxisPosition,
}

impl Axis {
    /// Create a new instance of an axis for a chart based on the provided scale and position.
    fn new<'a, T: ToString>(scale: &'a dyn Scale<T>, position: AxisPosition, chart: &Chart<'a>) -> Self {
        Self {
            ticks: Self::generate_ticks(scale, position, chart),
            position,
            axis_line: Self::get_axis_line(position, chart),
        }
    }

    /// Create a new axis at the top of the chart.
    pub fn new_top_axis<'a, T: ToString>(scale: &'a dyn Scale<T>, chart: &Chart<'a>) -> Self {
        Self::new(scale, AxisPosition::Top, chart)
    }

    /// Create a new axis to the right of the chart.
    pub fn new_right_axis<'a, T: ToString>(scale: &'a dyn Scale<T>, chart: &Chart<'a>) -> Self {
        Self::new(scale, AxisPosition::Right, chart)
    }

    /// Create a new axis at the bottom of the chart.
    pub fn new_bottom_axis<'a, T: ToString>(scale: &'a dyn Scale<T>, chart: &Chart<'a>) -> Self {
        Self::new(scale, AxisPosition::Bottom, chart)
    }

    /// Create a new axis to the left of the chart.
    pub fn new_left_axis<'a, T: ToString>(scale: &'a dyn Scale<T>, chart: &Chart<'a>) -> Self {
        Self::new(scale, AxisPosition::Left, chart)
    }

    /// Generate svg for the axis.
    pub fn to_svg(&self) -> Result<Group, Error> {
        let axis_class = match self.position {
            AxisPosition::Top => "x-axis",
            AxisPosition::Bottom => "x-axis",
            AxisPosition::Left => "y-axis",
            AxisPosition::Right => "y-axis",
        };

        let mut group = Group::new()
            .set("class", axis_class)
            .add(self.axis_line.to_svg().unwrap());

        for tick in self.ticks.iter() {
            group.append(tick.to_svg().unwrap());
        }

        Ok(group)
    }

    /// Generate ticks for the axis based on the scale and position.
    fn generate_ticks<'a, T: ToString>(scale: &'a dyn Scale<T>, position: AxisPosition, chart: &Chart<'a>) -> Vec<AxisTick> {
        let orientation = match position {
            AxisPosition::Top => Orientation::Horizontal,
            AxisPosition::Bottom => Orientation::Horizontal,
            AxisPosition::Left => Orientation::Vertical,
            AxisPosition::Right => Orientation::Vertical,
        };
        let mut ticks = Vec::new();

        for tick in scale.get_ticks() {
            let tick_offset = match position {
                AxisPosition::Bottom if scale.get_type() == ScaleType::Band => scale.scale(&tick) + scale.bandwidth().unwrap() / 2_f32,
                AxisPosition::Bottom => scale.scale(&tick),
                AxisPosition::Left if scale.get_type() == ScaleType::Band => chart.get_view_height() as f32 - scale.scale(&tick) - scale.bandwidth().unwrap() / 2_f32,
                AxisPosition::Left => chart.get_view_height() as f32 - scale.scale(&tick),
                AxisPosition::Top if scale.get_type() == ScaleType::Band => scale.scale(&tick) + scale.bandwidth().unwrap() / 2_f32,
                AxisPosition::Top => scale.scale(&tick),
                AxisPosition::Right if scale.get_type() == ScaleType::Band => chart.get_view_height() as f32 - scale.scale(&tick) - scale.bandwidth().unwrap() / 2_f32,
                AxisPosition::Right => chart.get_view_height() as f32 - scale.scale(&tick),
            };
            let axis_tick = AxisTick::new(tick_offset, 16, tick.to_string(), orientation);
            ticks.push(axis_tick);
        }

        ticks
    }

    /// Generate the line that represents the axis.
    fn get_axis_line<'a>(position: AxisPosition, chart: &Chart<'a>) -> AxisLine {
        match position {
            AxisPosition::Top => AxisLine::new(0_f32, 0_f32, chart.get_view_width() as f32, 0_f32),
            AxisPosition::Right => AxisLine::new(0_f32, 0_f32, 0_f32, chart.get_view_height() as f32),
            AxisPosition::Bottom => AxisLine::new(0_f32, 0_f32, chart.get_view_width() as f32, 0_f32),
            AxisPosition::Left => AxisLine::new(0_f32, 0_f32, 0_f32, chart.get_view_height() as f32),
        }
    }
}
