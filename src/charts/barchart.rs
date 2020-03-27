//! # BarChart
//!
//! Represents a horizontal or vertical bar chart.
//!
//! ## Details
//!
//! A [BarChart] should have at least one [ScaleBand] or [ScaleOrdinal] axis.

use std::ffi::OsStr;
use std::path::Path;
use svg;
use svg::parser::Error;
use svg::node::element::Group;
use crate::{view, View, VerticalBarDataset};
use crate::datasets::Dataset;
use crate::components::DatumRepresentation;

/// The BarChart struct definition.
pub struct BarChart<'a> {
    view: &'a View<'a>,
}

impl<'a> BarChart<'a> {
    pub fn with_view(view: &'a View<'a>) -> Self {
        Self {
            view,
        }
    }

    pub fn add_view(&mut self, view: &'a View<'a>) {
        self.view = view;
    }

    pub fn to_svg(&self) -> Result<Group, Error> {
        let mut group = Group::new()
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
                            .set("width", 800)
                            .set("height", 400)
                            .set("viewBox", (0, 0, 800, 400))
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
