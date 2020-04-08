use svg::node::Node;
use svg::node::element::Group;
use svg::node::element::Rectangle;
use svg::node::Text as TextNode;
use svg::node::element::Text;
use crate::components::DatumRepresentation;
use crate::chart::Orientation;

/// Set the position of a bar's label.
#[derive(Copy, Clone, Debug)]
pub enum BarLabelPosition {
    StartOutside,
    StartInside,
    Center,
    EndInside,
    EndOutside,
}

/// Represents a block within a bar.
/// The first tuple element represents the starting position, the second
/// one is the size of that block and the third one is the color.
#[derive(Debug)]
pub struct BarBlock(f32, f32, f32, String);

impl BarBlock {
    pub fn new(start: f32, end: f32, size: f32, color: String) -> Self {
        Self(start, end, size, color)
    }
}

#[derive(Debug)]
pub struct Bar {
    blocks: Vec<BarBlock>,
    orientation: Orientation,
    label_position: BarLabelPosition,
    rounding_precision: Option<usize>,
    label_visible: bool,
    category: String,
    bar_width: f32,
    offset: f32,
}

impl Bar {
    pub fn new(
        blocks: Vec<BarBlock>,
        orientation: Orientation,
        category: String,
        label_position: BarLabelPosition,
        label_visible: bool,
        rounding_precision: Option<usize>,
        bar_width: f32,
        offset: f32
    ) -> Self {
        Self {
            blocks,
            orientation,
            label_position,
            rounding_precision,
            label_visible,
            category,
            bar_width,
            offset,
        }
    }
}

impl DatumRepresentation for Bar {

    fn to_svg(&self) -> Result<Group, String> {
        let (bar_group_offset_x, bar_group_offset_y) = {
            match self.orientation {
                Orientation::Vertical => (self.offset, 0_f32),
                Orientation::Horizontal => (0_f32, self.offset),
            }
        };

        let mut group = Group::new()
            .set("transform", format!("translate({},{})", bar_group_offset_x, bar_group_offset_y))
            .set("class", "bar");

        let (x_attr, y_attr, width_attr, height_attr) = match self.orientation {
            Orientation::Horizontal => ("x", "y", "width", "height"),
            Orientation::Vertical => ("y", "x", "height", "width"),
        };

        for block in self.blocks.iter() {
            let block_rect = Rectangle::new()
                .set(x_attr, block.0)
                .set(y_attr, 0)
                .set(width_attr, block.1 - block.0)
                .set(height_attr, self.bar_width)
                .set("shape-rendering", "crispEdges")
                .set("fill", block.3.as_ref());

            group.append(block_rect);

            // Display labels if needed.
            if self.label_visible {
                let (label_x_attr_value, text_anchor) = match self.label_position {
                    BarLabelPosition::StartOutside if self.orientation == Orientation::Horizontal => (block.0 - 12_f32, "end"),
                    BarLabelPosition::StartOutside if self.orientation == Orientation::Vertical => (block.1 + 16_f32, "middle"),
                    BarLabelPosition::StartInside if self.orientation == Orientation::Horizontal => (block.0 + 12_f32, "start"),
                    BarLabelPosition::StartInside if self.orientation == Orientation::Vertical => (block.1 - 16_f32, "middle"),
                    BarLabelPosition::Center if self.orientation == Orientation::Horizontal => (block.0 + (block.1 - block.0) / 2_f32, "middle"),
                    BarLabelPosition::Center if self.orientation == Orientation::Vertical => (block.0 + (block.1 - block.0) / 2_f32, "middle"),
                    BarLabelPosition::EndInside if self.orientation == Orientation::Horizontal => (block.1 - 12_f32, "end"),
                    BarLabelPosition::EndInside if self.orientation == Orientation::Vertical => (block.0 + 16_f32, "middle"),
                    BarLabelPosition::EndOutside if self.orientation == Orientation::Horizontal => (block.1 + 12_f32, "start"),
                    BarLabelPosition::EndOutside if self.orientation == Orientation::Vertical => (block.0 - 16_f32, "middle"),
                    _ => (0_f32, "middle"), // this is needed to get rid of compiler warning of exhaustively covering match pattern.
                };

                let label_text = match &self.rounding_precision {
                    None => block.2.to_string(),
                    Some(nr_of_digits) => format!("{:.1$}", block.2.to_string().parse::<f32>().unwrap(), nr_of_digits)
                };

                let label = Text::new()
                    .set(x_attr, label_x_attr_value)
                    .set(y_attr, self.bar_width / 2_f32)
                    .set("text-anchor", text_anchor)
                    .set("dy", ".35em")
                    .set("font-family", "sans-serif")
                    .set("fill", "#333")
                    .set("font-size", "14px")
                    .add(TextNode::new(label_text));

                group.append(label);
            }
        }

        // svg::save("bar-vert.svg", &group).unwrap();

        Ok(group)
    }
}