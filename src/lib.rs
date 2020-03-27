//! # rustplotlib
//!
//! A visualization library for Rust inspired by D3.js.
//!
//! ## Features
//!
//! This is a WIP, but so far the library supports the following chart types:
//!
//! 1. Bar Chart (horizontal and vertical)
//! 2. Stacked Bar Chart (horizontal and vertical)
//!
//! ## Abstraction Layers
//!
//! There are several abstractions at the foundation of this visualization library:
//!
//!   Page
//!   └- Grid
//!      └- Chart
//!         ├- Axes
//!         └- View
//!            └- Dataset
//!
//! TODO represent the structure visually

mod charts;
mod view;
mod scales;
pub mod utils;
mod datasets;
pub mod components;
mod colors;

pub use crate::charts::barchart::BarChart;
pub use crate::view::View;
pub use crate::scales::band::ScaleBand;
pub use crate::scales::linear::ScaleLinear;
pub use crate::scales::Scale;
pub use crate::charts::Render;
pub use crate::datasets::VerticalBarChartDataset;
pub use crate::datasets::datum::BarDatum;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
