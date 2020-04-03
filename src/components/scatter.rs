use svg::parser::Error;
use svg::node::element::{Group, Circle, Rectangle, Line};
use svg::node::Node;
use crate::components::DatumRepresentation;

/// Define the possible types of points in a scatter plot.
#[derive(Debug, Copy, Clone)]
pub enum MarkerType {
    Circle,
    Square,
    X,
}

/// Represents a point in a scatter plot.
#[derive(Debug)]
pub struct ScatterPoint {
    marker_type: MarkerType,
    marker_size: usize,
    x: f32,
    y: f32,
    color: String,
}

impl ScatterPoint {
    pub fn new(x: f32, y: f32, marker_type: MarkerType, marker_size: usize, color: String) -> Self {
        Self {
            marker_type,
            marker_size,
            x,
            y,
            color,
        }
    }
}

impl DatumRepresentation for ScatterPoint {

    fn to_svg(&self) -> Result<Group, Error> {
        let mut group = Group::new()
            .set("transform", format!("translate({},{})", self.x, self.y))
            .set("class", "scatter-point");

        match self.marker_type {
            MarkerType::Circle => {
                group.append(
                    Circle::new()
                        .set("cx", 0)
                        .set("cy", 0)
                        .set("r", self.marker_size)
                        .set("fill", self.color.as_ref())
                );
            },
            MarkerType::Square => {
                group.append(
                    Rectangle::new()
                        .set("x", -(self.marker_size as i32))
                        .set("y", -(self.marker_size as i32))
                        .set("width", 2 * self.marker_size)
                        .set("height", 2 * self.marker_size)
                        .set("fill", self.color.as_ref())
                );
            },
            MarkerType::X => {
                group.append(
                    Group::new()
                        .add(
                            Line::new()
                                .set("x1", -(self.marker_size as i32))
                                .set("y1", -(self.marker_size as i32))
                                .set("x2", self.marker_size)
                                .set("y2", self.marker_size)
                                .set("stroke", self.color.as_ref())
                        )
                        .add(
                            Line::new()
                                .set("x1", self.marker_size)
                                .set("y1", -(self.marker_size as i32))
                                .set("x2", -(self.marker_size as i32))
                                .set("y2", self.marker_size)
                                .set("stroke", self.color.as_ref())
                        )
                );
            },
        };

        Ok(group)
    }
}