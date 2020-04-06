use std::fmt::Display;
use svg::node::element::{Group, Path};
use svg::node::element::path::Data;
use svg::node::Node;
use crate::components::DatumRepresentation;
use crate::components::scatter::ScatterPoint;

/// Represents a point in a scatter plot.
#[derive(Debug)]
pub struct LineSeries<T: Display, U: Display> {
    points: Vec<ScatterPoint<T, U>>,
    color: String,
}

impl<T: Display, U: Display> LineSeries<T, U> {
    pub fn new(
        points: Vec<ScatterPoint<T, U>>,
        color: String
    ) -> Self {
        Self {
            points,
            color,
        }
    }
}

impl<T: Display, U: Display> DatumRepresentation for LineSeries<T, U> {

    fn to_svg(&self) -> Result<Group, String> {
        let mut group = Group::new()
            .set("class", "line");

        let mut data = Data::new();

        for (i, point) in self.points.iter().enumerate() {
            if i == 0 {
                data = data.move_to((point.get_x(), point.get_y()));
            } else {
                data = data.line_to((point.get_x(), point.get_y()));
            }
        }

        let line = Path::new()
            .set("fill", "none")
            .set("stroke", self.color.as_ref())
            .set("stroke-width", 2)
            .set("d", data);

        group.append(line);

        for point in self.points.iter() {
            group.append(point.to_svg()?);
        }

        Ok(group)
    }
}
