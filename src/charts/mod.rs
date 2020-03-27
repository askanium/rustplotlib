//! # Charts
//!
//! A chart consists of a [View] and between 2 and 4 [Axis]
//! that define the bounds of the [Dataset]s present in the [View].
//!
//! Charts are the smallest self-sufficient entities that can be saved as a file.

use std::path::Path;
use svg;
use svg::parser::Error;
use std::ffi::OsStr;

pub mod barchart;

/// A Chart trait that defines the visualization behavior.
pub trait Render {
    type SVGNode;

    /// Generate the svg.
    fn to_svg(&self) -> Result<Self::SVGNode, Error>;
}
