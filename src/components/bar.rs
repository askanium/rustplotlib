use svg::parser::Error;
use svg::node::Node;
use svg::node::element::Group;
use crate::components::DatumRepresentation;
use crate::utils::Orientation;

/// Represents a block within a bar.
/// The first tuple element represents the starting position, the second
/// one is the size of that block and the third one is the color.
#[derive(Debug)]
pub struct BarBlock(f32, f32, String);

impl BarBlock {
    pub fn new(start: f32, size: f32, color: String) -> Self {
        Self(start, size, color)
    }
}

#[derive(Debug)]
pub struct Bar {
    blocks: Vec<BarBlock>,
    orientation: Orientation,
    category: String,
    label: String,
    bar_width: f32,
    offset: f32,
}

impl Bar {
    pub fn new(blocks: Vec<BarBlock>, orientation: Orientation, category: String, label: String, bar_width: f32, offset: f32) -> Self {
        Self {
            blocks,
            orientation,
            category,
            label,
            bar_width,
            offset,
        }
    }
}

impl DatumRepresentation for Bar {

    fn to_svg(&self) -> Result<Group, Error> {
        let mut group = Group::new()
            .set("transform", format!("translate({},0)", self.offset));

        let (x_attr, y_attr, width_attr, height_attr) = match self.orientation {
            Orientation::Horizontal => ("x", "y", "width", "height"),
            Orientation::Vertical => ("y", "x", "height", "width"),
        };

        for block in self.blocks.iter() {
            let block = svg::node::element::Rectangle::new()
                .set(x_attr, block.0)
                .set(y_attr, 0)
                .set(width_attr, block.1)
                .set(height_attr, self.bar_width)
                .set("fill", block.2.as_ref());

            group.append(block);
        }

        // svg::save("bar-vert.svg", &group).unwrap();

        Ok(group)
    }
}