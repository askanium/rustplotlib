//! # BarChart
//!
//! Represents a horizontal or vertical bar chart.
//!
//! ## Details
//!
//! A [BarChart] should have at least one [ScaleBand] or [ScaleOrdinal] axis.

use crate::{view, View, VerticalBarChartDataset};
use crate::charts::Render;
use std::path::Path;
use std::ffi::OsStr;
use svg;
use svg::parser::Error;
use crate::datasets::Dataset;
use crate::components::DatumRepresentation;

/// The BarChart struct definition.
pub struct BarChart<'a> {
    view: view::View<'a>,
}

impl<'a> BarChart<'a> {
    pub fn new() -> Self {
        Self {
            view: View::new(),
        }
    }

    /// Save the chart to a file
    fn save(&self, path: &dyn AsRef<Path>) -> Result<(), String> {
        // match path.as_ref().extension().and_then(OsStr::to_str) {
        //     Some("svg") => {
        //         match self.to_svg() {
        //             Ok(svg_content) => svg::save(path, &svg_content).unwrap(),
        //             Err(e) => return Err(format!("Encountered an error while saving the chart: {:?}", e)),
        //         }
        //     },
        //     _ => {},
        // };
        Ok(())
    }

}

// impl<'a> Render for BarChart<'a> {
//     type SVGNode = svg::Document;
//
//     fn to_svg(&self) -> Result<svg::Document, Error> {
//         let rect = svg::node::element::Rectangle::new()
//             .set("x", 0)
//             .set("y", 0)
//             .set("width", 100)
//             .set("height", 50)
//             .set("fill", "blue");
//
//         let mut document = svg::Document::new()
//             .set("viewBox", (0, 0, 400, 200))
//             .add(rect);
//
//         for boxed_dataset in self.view.datasets() {
//             document.add(boxed_dataset.to_svg().unwrap());
//         }
//
//         Ok(document)
//     }
// }
