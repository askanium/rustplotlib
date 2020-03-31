//! # Chart
//!
//! A chart consists of a [View] and between 2 and 4 [Axis]
//! that define the bounds of the [Dataset]s present in the [View].
//!
//! Charts are the smallest self-sufficient entities that can be saved as a file.

use std::string::ToString;
use std::ffi::OsStr;
use std::path::Path;
use svg;
use svg::parser::Error;
use svg::node::element::Group;
use svg::Node;
use crate::view::View;
use crate::{Axis, Scale};
use crate::datasets::Dataset;
use crate::axis::AxisPosition;

/// The BarChart struct definition.
pub struct Chart<'a> {
    margin_top: usize,
    margin_bottom: usize,
    margin_right: usize,
    margin_left: usize,
    width: usize,
    height: usize,
    x_axis_top: Option<Axis>,
    x_axis_bottom: Option<Axis>,
    y_axis_left: Option<Axis>,
    y_axis_right: Option<Axis>,
    view: View<'a>,
}

impl<'a> Chart<'a> {
    /// Create a new instance of a chart with default sizes.
    pub fn new() -> Self {
        Self {
            margin_top: 20,
            margin_bottom: 50,
            margin_right: 20,
            margin_left: 60,
            width: 800,
            height: 600,
            x_axis_top: None,
            x_axis_bottom: None,
            y_axis_left: None,
            y_axis_right: None,
            view: View::new(),
        }
    }

    /// Set the margins of the chart to provided values.
    pub fn set_margins(mut self, top: usize, right: usize, bottom: usize, left: usize) -> Self {
        self.margin_top = top;
        self.margin_right = right;
        self.margin_bottom = bottom;
        self.margin_left = left;
        self
    }

    /// Add the dataset to the chart's view.
    pub fn add_dataset(mut self, dataset: &'a dyn Dataset<'a>) -> Self {
        self.view.add_dataset(dataset);
        self
    }

    /// Add an axis at the bottom of the chart.
    pub fn add_axis_bottom<T: ToString>(mut self, scale: &'a dyn Scale<T>) -> Self {
        self.x_axis_bottom = Some(Axis::new_bottom_axis(scale, &self));
        self
    }

    /// Add an axis at the bottom of the chart.
    pub fn add_axis_left<T: ToString>(mut self, scale: &'a dyn Scale<T>) -> Self {
        self.y_axis_left = Some(Axis::new_left_axis(scale, &self));
        self
    }

    /// Return the offset from the left where the view starts.
    pub fn get_view_horizontal_start_offset(&self) -> usize {
        self.margin_left
    }

    /// Return the offset from the left where the view ends.
    pub fn get_view_horizontal_end_offset(&self) -> usize {
        self.width - self.margin_right
    }

    /// Return the offset from the left where the view starts.
    pub fn get_view_vertical_start_offset(&self) -> usize {
        self.margin_top
    }

    /// Return the offset from the left where the view ends.
    pub fn get_view_vertical_end_offset(&self) -> usize {
        self.width - self.margin_bottom
    }

    /// Return the width of the view.
    pub fn get_view_width(&self) -> usize {
        self.width - self.margin_left - self.margin_right
    }

    /// Return the height of the view.
    pub fn get_view_height(&self) -> usize {
        self.height - self.margin_top - self.margin_bottom
    }

    fn to_svg(&self) -> Result<Group, Error> {
        let mut group = Group::new()
            .set("class", "g-chart");

        let mut view_group = self.view.to_svg()?;
        view_group.assign("transform", format!("translate({},{})", self.margin_left, self.margin_top));
        group.append(view_group);

        if let Some(ref axis) = self.x_axis_bottom {
            let mut axis_group = axis.to_svg().unwrap();
            axis_group.assign("transform", format!("translate({},{})", self.margin_left, self.height - self.margin_bottom));
            group.append(axis_group);
        };

        if let Some(ref axis) = self.y_axis_left {
            let mut axis_group = axis.to_svg().unwrap();
            axis_group.assign("transform", format!("translate({},{})", self.margin_left, self.margin_top));
            group.append(axis_group);
        };

        Ok(group)
    }

    /// Save the chart to a file
    pub fn save(self, path: &dyn AsRef<Path>) -> Result<(), String> {
        match path.as_ref().extension().and_then(OsStr::to_str) {
            Some("svg") => {
                match self.to_svg() {
                    Ok(svg_content) => {
                        let document = svg::Document::new()
                            .set("width", self.width)
                            .set("height", self.height)
                            .set("viewBox", (0, 0, self.width, self.height))
                            .add(svg_content);

                        svg::save(path, &document).unwrap()
                    },
                    Err(e) => return Err(format!("Encountered an error while saving the chart: {:?}", e)),
                }
            },
            _ => {},
        };
        Ok(())
    }
}
