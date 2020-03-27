//! # View
//!
//! A [View] is a wrapper for several [Dataset]s. Its sole purpose is to
//! render the contained datasets in a chart.

use crate::datasets::Dataset;

/// The view struct that holds datasets to be rendered in the chart.
pub struct View<'a> {
    datasets: Vec<Box<dyn Dataset<'a>>>,
}

impl<'a> View<'a> {
    pub fn new() -> Self {
        Self {
            datasets: Vec::new(),
        }
    }

    pub fn datasets(&self) -> &Vec<Box<dyn Dataset<'a>>> {
        &self.datasets
    }
}