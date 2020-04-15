use svg::node::element::{Group, Circle, Rectangle, Line};
use svg::Node;
use svg::node::Text as TextNode;
use svg::node::element::Text;
use crate::MarkerType;

/// Represents the possible marker types that a legend entry can have.
pub enum LegendMarkerType {
    Circle,
    Square,
    X,
    Line,
}

impl From<MarkerType> for LegendMarkerType {
    fn from(marker_type: MarkerType) -> Self {
        match marker_type {
            MarkerType::Circle => LegendMarkerType::Circle,
            MarkerType::Square => LegendMarkerType::Square,
            MarkerType::X => LegendMarkerType::X,
        }
    }
}

/// Represents an entry in the chart's legend.
pub struct LegendEntry {
    marker_type: LegendMarkerType,
    marker_size: usize,
    marker_to_label_gap: usize,
    color: String,
    stroke_type: String,
    label: String,
}

impl LegendEntry {
    /// Create a new legend entry.
    pub fn new(marker_type: LegendMarkerType, color: String, stroke_type: String, label: String) -> Self {
        Self {
            marker_type,
            marker_size: 7,
            marker_to_label_gap: 6,
            color,
            stroke_type,
            label,
        }
    }

    /// Return legend entry width to compute the placement of legend entries on the chart.
    pub fn get_width(&self) -> usize {
        // TODO ideally, compute the length of the given `label` in the given font and size
        let avg_letter_width = 7; // this is for the default sans-serif 12px font + some buffer
        avg_letter_width * self.label.len() + self.marker_size * 2 + self.marker_to_label_gap
    }

    pub fn to_svg(&self) -> Result<Group, String> {
        let mut group = Group::new()
            .set("class", "legend-entry");

        match self.marker_type {
            LegendMarkerType::Circle => group.append(
                Circle::new()
                    .set("cx", self.marker_size)
                    .set("cy", self.marker_size)
                    .set("r", self.marker_size)
                    .set("fill", self.color.as_ref())
                    .set("stroke", "none")
            ),
            LegendMarkerType::Square => group.append(
                Rectangle::new()
                    .set("x", 0)
                    .set("y", 0)
                    .set("width", 2 * self.marker_size)
                    .set("height", 2 * self.marker_size)
                    .set("fill", self.color.as_ref())
                    .set("stroke", "none")
            ),
            LegendMarkerType::X => {
                group.append(
                    Line::new()
                        .set("x1", 0)
                        .set("y1", 0)
                        .set("x2", 2 * self.marker_size)
                        .set("y2", 2 * self.marker_size)
                        .set("stroke", self.color.as_ref())
                        .set("stroke-width", "2px")
                );
                group.append(
                    Line::new()
                        .set("x1", 2 * self.marker_size)
                        .set("y1", 0)
                        .set("x2", 0)
                        .set("y2", 2 * self.marker_size)
                        .set("stroke", self.color.as_ref())
                        .set("stroke-width", "2px")
                )
            },
            LegendMarkerType::Line => group.append(
                Line::new()
                    .set("x1", 0)
                    .set("y1", self.marker_size)
                    .set("x2", 2 * self.marker_size)
                    .set("y2", self.marker_size)
                    .set("stroke", self.color.as_ref())
                    .set("stroke-width", "2px")
                    .set("stroke-dasharray", self.stroke_type.as_ref())
            ),
        }

        group.append(
            Text::new()
                .set("x", 2 * self.marker_size + self.marker_to_label_gap)
                .set("y", self.marker_size)
                .set("dy", ".35em")
                .set("font-family", "sans-serif")
                .set("fill", "#777")
                .set("font-size", "12px")
                .add(TextNode::new(self.label.clone()))
        );

        Ok(group)
    }
}