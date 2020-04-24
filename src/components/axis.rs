use svg::node::element::{Group, Line};
use svg::node::Text as TextNode;
use svg::node::element::Text;
use svg::Node;
use format_num::NumberFormat;
use crate::axis::AxisPosition;

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
            .set("shape-rendering", "crispEdges")
            .set("stroke-width", 1)
            .set("stroke", "#bbbbbb");

        Ok(line)
    }
}

/// A struct to represent an axis tick
pub struct AxisTick {
    axis_position: AxisPosition,
    label_offset: usize,
    label_rotation: isize,
    tick_offset: f32,
    label: String,
    label_format: Option<String>
}

impl AxisTick {
    /// Create a new instance of AxisTick.
    pub fn new(tick_offset: f32, label_offset: usize, label_rotation: isize, label: String, axis_position: AxisPosition) -> Self {
        Self {
            label_offset,
            tick_offset,
            label_rotation,
            label,
            axis_position,
            label_format: None,
        }
    }

    /// Set label rotation.
    pub fn set_label_rotation(&mut self, rotation: isize) {
        self.label_rotation = rotation;
    }

    /// Set label rotation.
    pub fn set_label_format(&mut self, format: &str) {
        self.label_format = Some(format.to_owned());
    }

    /// Render the axis tick to svg.
    pub fn to_svg(&self) -> Result<Group, String> {
        let formatted_label = if self.label_format.is_some() {
            let formatter = NumberFormat::new();
            formatter.format(self.label_format.as_ref().unwrap(), self.label.parse::<f64>().unwrap()).replace('G', "B")
        } else {
            self.label.to_owned()
        };
        let offsets: (f32, f32);
        let tick_line_p2: (isize, isize);
        let tick_label_offset: (isize, isize);
        let tick_label_text_anchor: &str;

        match self.axis_position {
            AxisPosition::Left => {
                offsets = (0_f32, self.tick_offset);
                tick_line_p2 = (-6, 0);
                tick_label_offset = (-(self.label_offset as isize), 0);
                tick_label_text_anchor = "end";
            },
            AxisPosition::Bottom => {
                offsets = (self.tick_offset, 0_f32);
                tick_line_p2 = (0, 6);
                tick_label_offset = (0, self.label_offset as isize);
                tick_label_text_anchor = "middle";
            },
            AxisPosition::Right => {
                offsets = (0_f32, self.tick_offset);
                tick_line_p2 = (6, 0);
                tick_label_offset = (self.label_offset as isize, 0);
                tick_label_text_anchor = "start";
            },
            AxisPosition::Top => {
                offsets = (self.tick_offset, 0_f32);
                tick_line_p2 = (0, -6);
                tick_label_offset = (0, -(self.label_offset as isize));
                tick_label_text_anchor = "middle";
            },
        };

        let mut group = Group::new()
            .set("class", "tick")
            .set("transform", format!("translate({},{})", offsets.0, offsets.1));

        let tick_line = Line::new()
            .set("x1", 0)
            .set("y1", 0)
            .set("x2", tick_line_p2.0)
            .set("y2", tick_line_p2.1)
            .set("shape-rendering", "crispEdges")
            .set("stroke", "#bbbbbb")
            .set("stroke-width", "1px");

        let tick_label = Text::new()
            .set("transform", format!("rotate({},{},{})", self.label_rotation, tick_label_offset.0, tick_label_offset.1))
            .set("x", tick_label_offset.0)
            .set("y", tick_label_offset.1)
            .set("dy", ".35em")
            .set("text-anchor", tick_label_text_anchor)
            .set("font-size", "12px")
            .set("font-family", "sans-serif")
            .set("fill", "#777")
            .add(TextNode::new(formatted_label));

        group.append(tick_line);
        group.append(tick_label);

        Ok(group)
    }
}