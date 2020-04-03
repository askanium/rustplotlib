use svg::parser::Error;
use svg::node::element::{Group, Circle, Rectangle, Line};
use svg::node::Node;
use crate::components::DatumRepresentation;

/// Define the possible types of points in a scatter plot.
#[derive(Debug, Copy, Clone)]
pub enum PointType {
    Circle,
    Square,
    X,
}

/// Represents a point in a scatter plot.
#[derive(Debug)]
pub struct ScatterPoint {
    point_type: PointType,
    point_size: usize,
    x: f32,
    y: f32,
    color: String,
}

impl ScatterPoint {
    pub fn new(x: f32, y: f32, point_type: PointType, point_size: usize, color: String) -> Self {
        Self {
            point_type,
            point_size,
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

        match self.point_type {
            PointType::Circle => {
                group.append(
                    Circle::new()
                        .set("cx", 0)
                        .set("cy", 0)
                        .set("r", self.point_size)
                        .set("fill", self.color.as_ref())
                );
            },
            PointType::Square => {
                group.append(
                    Rectangle::new()
                        .set("x", -(self.point_size as i32))
                        .set("y", -(self.point_size as i32))
                        .set("width", 2 * self.point_size)
                        .set("height", 2 * self.point_size)
                        .set("fill", self.color.as_ref())
                );
            },
            PointType::X => {
                group.append(
                    Group::new()
                        .add(
                            Line::new()
                                .set("x1", -(self.point_size as i32))
                                .set("y1", -(self.point_size as i32))
                                .set("x2", self.point_size)
                                .set("y2", self.point_size)
                                .set("stroke", self.color.as_ref())
                        )
                        .add(
                            Line::new()
                                .set("x1", self.point_size)
                                .set("y1", -(self.point_size as i32))
                                .set("x2", -(self.point_size as i32))
                                .set("y2", self.point_size)
                                .set("stroke", self.color.as_ref())
                        )
                );
            },
        };

        Ok(group)
    }
}