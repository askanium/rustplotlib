use std::fmt::Display;
use svg::node::element::{Group, Circle, Rectangle, Line};
use svg::node::Node;
use svg::node::Text as TextNode;
use svg::node::element::Text;
use crate::components::DatumRepresentation;

/// Define the possible types of points in a scatter plot.
#[derive(Debug, Copy, Clone)]
pub enum MarkerType {
    Circle,
    Square,
    X,
}

/// Define the possible locations of a point's label.
#[derive(Debug, Copy, Clone)]
pub enum PointLabelPosition {
    N,
    NE,
    E,
    SE,
    S,
    SW,
    W,
    NW
}

/// Represents a point in a scatter plot.
#[derive(Debug)]
pub struct ScatterPoint<T: Display, U: Display> {
    label_position: PointLabelPosition,
    label_visible: bool,
    point_visible: bool,
    marker_type: MarkerType,
    marker_size: usize,
    x: f32,
    y: f32,
    x_label: T,
    y_label: U,
    color: String,
}

impl<T: Display, U: Display> ScatterPoint<T, U> {
    pub fn new(
        x: f32,
        y: f32,
        marker_type: MarkerType,
        marker_size: usize,
        x_label: T,
        y_label: U,
        label_position: PointLabelPosition,
        label_visible: bool,
        point_visible: bool,
        color: String
    ) -> Self {
        Self {
            label_position,
            label_visible,
            point_visible,
            marker_type,
            marker_size,
            x,
            y,
            x_label,
            y_label,
            color,
        }
    }

    /// Return the x coordinate of the point.
    pub fn get_x(&self) -> f32 {
        self.x
    }

    /// Return the y coordinate of the point.
    pub fn get_y(&self) -> f32 {
        self.y
    }
}

impl<T: Display, U: Display> DatumRepresentation for ScatterPoint<T, U> {

    fn to_svg(&self) -> Result<Group, String> {
        let mut group = Group::new()
            .set("transform", format!("translate({},{})", self.x, self.y))
            .set("class", "scatter-point");

        match self.marker_type {
            MarkerType::Circle if self.point_visible => {
                group.append(
                    Circle::new()
                        .set("cx", 0)
                        .set("cy", 0)
                        .set("r", self.marker_size)
                        .set("fill", self.color.as_ref())
                );
            },
            MarkerType::Square if self.point_visible => {
                group.append(
                    Rectangle::new()
                        .set("x", -(self.marker_size as i32))
                        .set("y", -(self.marker_size as i32))
                        .set("width", 2 * self.marker_size)
                        .set("height", 2 * self.marker_size)
                        .set("fill", self.color.as_ref())
                );
            },
            MarkerType::X if self.point_visible => {
                group.append(
                    Group::new()
                        .add(
                            Line::new()
                                .set("x1", -(self.marker_size as i32))
                                .set("y1", -(self.marker_size as i32))
                                .set("x2", self.marker_size)
                                .set("y2", self.marker_size)
                                .set("stroke-width", "2px")
                                .set("stroke", self.color.as_ref())
                        )
                        .add(
                            Line::new()
                                .set("x1", self.marker_size)
                                .set("y1", -(self.marker_size as i32))
                                .set("x2", -(self.marker_size as i32))
                                .set("y2", self.marker_size)
                                .set("stroke-width", "2px")
                                .set("stroke", self.color.as_ref())
                        )
                );
            },
            _ => {},
        };

        if self.label_visible {
            let mut point_label = Text::new()
                .set("dy", ".35em")
                .set("font-family", "sans-serif")
                .set("fill", "#333")
                .set("font-size", "14px")
                .add(TextNode::new(format!("({}, {})", self.x_label, self.y_label)));

            let label_offset = self.marker_size as isize;
            match self.label_position {
                PointLabelPosition::N => {
                    point_label.assign("x", 0);
                    point_label.assign("y", -label_offset - 12);
                    point_label.assign("text-anchor", "middle");
                },
                PointLabelPosition::NE => {
                    point_label.assign("x", label_offset + 4);
                    point_label.assign("y", -label_offset - 8);
                    point_label.assign("text-anchor", "start");
                },
                PointLabelPosition::E => {
                    point_label.assign("x", label_offset + 8);
                    point_label.assign("y", 0);
                    point_label.assign("text-anchor", "start");
                },
                PointLabelPosition::SE => {
                    point_label.assign("x", label_offset + 4);
                    point_label.assign("y", label_offset + 8);
                    point_label.assign("text-anchor", "start");
                },
                PointLabelPosition::S => {
                    point_label.assign("x", 0);
                    point_label.assign("y", label_offset + 12);
                    point_label.assign("text-anchor", "middle");
                },
                PointLabelPosition::SW => {
                    point_label.assign("x", -label_offset - 4);
                    point_label.assign("y", label_offset + 8);
                    point_label.assign("text-anchor", "end");
                },
                PointLabelPosition::W => {
                    point_label.assign("x", -label_offset - 8);
                    point_label.assign("y", 0);
                    point_label.assign("text-anchor", "end");
                },
                PointLabelPosition::NW => {
                    point_label.assign("x", -label_offset - 4);
                    point_label.assign("y", -label_offset - 8);
                    point_label.assign("text-anchor", "end");
                },
            }
            group.append(point_label);
        }

        Ok(group)
    }
}