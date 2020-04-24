use std::string::ToString;
use svg::node::element::Group;
use svg::parser::Error;
use svg::Node;
use svg::node::Text as TextNode;
use svg::node::element::Text;
use crate::{Scale, Chart};
use crate::components::axis::{AxisLine, AxisTick};
use crate::scales::ScaleType;

/// Enum of possible axis positions on the chart.
#[derive(Copy, Clone, PartialEq)]
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
    label: String,
    label_rotation: isize,
    label_format: String,
    length: isize,
}

impl Axis {
    /// Create a new instance of an axis for a chart based on the provided scale and position.
    fn new<'a, T: ToString>(scale: &'a dyn Scale<T>, position: AxisPosition, chart: &Chart<'a>) -> Self {
        Self {
            ticks: Self::generate_ticks(scale, position),
            position,
            axis_line: Self::get_axis_line(position, chart),
            label: String::new(),
            label_rotation: 0,
            label_format: String::new(),
            length: Self::get_axis_length(position, chart),
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

    /// Set axis label.
    pub fn set_axis_label(&mut self, label: String) {
        self.label = label;
    }

    /// Set tick label rotation.
    pub fn set_tick_label_rotation(&mut self, rotation: isize) {
        self.label_rotation = rotation;
        self.ticks.iter_mut().for_each(|tick| tick.set_label_rotation(rotation));
    }

    /// Set the label format.
    pub fn set_tick_label_format(&mut self, format: &str) {
        self.label_format = String::from(format);
        let label_format = self.label_format.as_str();
        self.ticks.iter_mut().for_each(|tick| tick.set_label_format(label_format));
    }

    /// Return whether the axis has a label or not.
    pub fn has_label(&self) -> bool {
        self.label.len() > 0
    }

    /// Compute the length of the axis.
    fn get_axis_length<'a>(position: AxisPosition, chart: &Chart<'a>) -> isize {
        if position == AxisPosition::Top || position == AxisPosition::Bottom {
            chart.get_view_width()
        } else {
            chart.get_view_height()
        }
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

        if self.label.len() > 0 {
            let (x, y, rotate) = match self.position {
                AxisPosition::Top => ((self.length / 2) as i32, -32, 0),
                AxisPosition::Bottom => ((self.length / 2) as i32, 42, 0),
                AxisPosition::Left => (-(self.length as i32 / 2), -42, -90),
                AxisPosition::Right => ((self.length as i32 / 2), -42, 90),
            };
            let axis_label = Text::new()
                .set("x", x)
                .set("y", y)
                .set("text-anchor", "middle")
                .set("font-size", "14px")
                .set("font-family", "sans-serif")
                .set("fill", "#777")
                .set("transform", format!("rotate({})", rotate))
                .add(TextNode::new(&self.label));
            group.append(axis_label);
        }

        Ok(group)
    }

    /// Generate ticks for the axis based on the scale and position.
    fn generate_ticks<'a, T: ToString>(scale: &'a dyn Scale<T>, position: AxisPosition) -> Vec<AxisTick> {
        let mut ticks = Vec::new();
        let label_offset = {
            if position == AxisPosition::Top || position == AxisPosition::Bottom {
                16
            } else {
                12
            }
        };

        for tick in scale.get_ticks() {
            let tick_offset = match position {
                AxisPosition::Bottom if scale.get_type() == ScaleType::Band => scale.scale(&tick) + scale.bandwidth().unwrap() / 2_f32,
                AxisPosition::Bottom => scale.scale(&tick),
                AxisPosition::Left if scale.get_type() == ScaleType::Band => scale.scale(&tick) + scale.bandwidth().unwrap() / 2_f32,
                AxisPosition::Left => scale.scale(&tick),
                AxisPosition::Top if scale.get_type() == ScaleType::Band => scale.scale(&tick) + scale.bandwidth().unwrap() / 2_f32,
                AxisPosition::Top => scale.scale(&tick),
                AxisPosition::Right if scale.get_type() == ScaleType::Band => scale.scale(&tick) + scale.bandwidth().unwrap() / 2_f32,
                AxisPosition::Right => scale.scale(&tick),
            };
            let axis_tick = AxisTick::new(tick_offset, label_offset, 0, tick.to_string(), position);
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
