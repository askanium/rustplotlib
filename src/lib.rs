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

mod chart;
// mod view;
mod scales;
mod views;
mod components;
mod colors;
mod axis;
mod legend;

pub use crate::chart::Chart;
pub use crate::scales::band::ScaleBand;
pub use crate::scales::linear::ScaleLinear;
pub use crate::scales::Scale;
pub use crate::views::vertical_bar::VerticalBarView;
pub use crate::views::horizontal_bar::HorizontalBarView;
pub use crate::views::scatter::ScatterView;
pub use crate::views::line::LineSeriesView;
pub use crate::views::area::AreaSeriesView;
pub use crate::views::datum::{BarDatum, PointDatum};
pub use crate::axis::{Axis, AxisPosition};
pub use crate::components::bar::BarLabelPosition;
pub use crate::components::line::LineSeries;
pub use crate::components::scatter::{MarkerType, PointLabelPosition};
pub use crate::colors::Color;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
