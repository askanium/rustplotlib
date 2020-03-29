use svg::node::element::{Group, Line};
use svg::node::Text as TextNode;
use svg::node::element::Text;
use svg::Node;
use crate::utils::Orientation;

/// A simple struct that represents an axis line.
pub(crate) struct AxisLine {
    x1: f32,
    y1: f32,
    x2: f32,
    y2: f32,
}

impl AxisLine {
    /// Create a new instance of axis line.
    pub fn new(x1: f32, y1: f32, x2: f32, y2: f32) -> Self {
        Self { x1, y1, x2, y2 }
    }

    /// Render the axis line to svg.
    pub fn to_svg(&self) -> Result<Line, String> {
        let line = Line::new()
            .set("x1", self.x1)
            .set("y1", self.y1)
            .set("x2", self.x2)
            .set("y2", self.y2)
            .set("stroke-width", 1)
            .set("stroke", "#bbbbbb");

        Ok(line)
    }
}

/// A struct to represent an axis tick
pub struct AxisTick {
    axis_orientation: Orientation,
    label_offset: usize,
    tick_offset: f32,
    label: String,
}

impl AxisTick {
    /// Create a new instance of AxisTick.
    pub fn new(tick_offset: f32, label_offset: usize, label: String, axis_orientation: Orientation) -> Self {
        Self {
            label_offset,
            tick_offset,
            label,
            axis_orientation,
        }
    }

    /// Render the axis tick to svg.
    pub fn to_svg(&self) -> Result<Group, String> {
        let mut group = Group::new()
            .set("class", "tick")
            .set("transform", format!("translate({},{})", self.tick_offset, 0));

        let tick_line = Line::new()
            .set("x1", 0)
            .set("y1", 0)
            .set("x2", 0)
            .set("y2", 6)
            .set("stroke", "#bbbbbb")
            .set("stroke-width", "1px");

        let mut tick_label = Text::new()
            .set("x", 0)
            .set("y", self.label_offset)
            .set("dy", ".35em")
            .set("text-anchor", "middle")
            .set("font-size", "14px")
            .set("font-family", "sans-serif")
            .set("fill", "#777")
            .add(TextNode::new(&self.label));

        group.append(tick_line);
        group.append(tick_label);

        Ok(group)
    }
}