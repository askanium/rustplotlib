//! # Chart
//!
//! A chart consists of a [View] and between 2 and 4 [Axis]
//! that define the bounds of the [Dataset]s present in the [View].
//!
//! Charts are the smallest self-sufficient entities that can be saved as a file.

use std::ffi::OsStr;
use std::path::Path;
use svg;
use svg::parser::Error;
use svg::node::element::Group;
use crate::view::View;

/// The BarChart struct definition.
pub struct Chart<'a> {
    margin_top: usize,
    margin_bottom: usize,
    margin_right: usize,
    margin_left: usize,
    width: usize,
    height: usize,
    view: &'a View<'a>,
}

impl<'a> Chart<'a> {
    pub fn with_view(view: &'a View<'a>) -> Self {
        Self {
            margin_top: 20,
            margin_bottom: 50,
            margin_right: 20,
            margin_left: 60,
            width: 600,
            height: 400,
            view,
        }
    }

    pub fn add_view(&mut self, view: &'a View<'a>) {
        self.view = view;
    }

    pub fn to_svg(&self) -> Result<Group, Error> {
        let mut group = Group::new()
            .set("transform", format!("translate({},{})", self.margin_left, self.margin_top))
            .set("class", "g-chart")
            .add(self.view.to_svg()?);

        Ok(group)
    }

    /// Save the chart to a file
    pub fn save(&self, path: &dyn AsRef<Path>) -> Result<(), String> {
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
