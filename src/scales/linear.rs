use crate::utils::{Range, Orientation};
use std::cmp::max;
use crate::scales::{Scale, ScaleType};
use std::fmt;
use std::hash::Hash;
use std::collections::HashMap;
use crate::components::axis::AxisTick;

/// The scale to represent categorical data.
#[derive(Debug)]
pub struct ScaleLinear {
    /// The domain limits of the dataset that the scale is going to represent.
    domain: Vec<f32>,
    /// The range limits of the drawable area on the chart.
    range: Range,
}

impl ScaleLinear {
    /// Create a new linear scale with default values.
    pub fn new() -> Self {
        let mut scale = Self {
            domain: Vec::new(),
            range: Range::default(),
        };
        scale
    }

    fn rescale(&mut self) {
    }
}

impl Scale<f32> for ScaleLinear {
    /// Set the domain limits for the scale band.
    fn set_domain(&mut self, range: Vec<f32>) {
        self.domain = range;
        self.rescale();
    }

    /// Get the domain limits of the scale.
    fn domain(&self) -> &Vec<f32> {
        &self.domain
    }

    /// Set the range limits for the scale band.
    fn set_range(&mut self, range: Range) {
        self.range = range;
        self.rescale();
    }

    /// Get the range limits of the scale.
    fn range(&self) -> &Range {
        &self.range
    }

    /// Get the type of the scale.
    fn get_type(&self) -> ScaleType {
        ScaleType::Linear
    }

    /// Get the range value for the given domain entry.
    fn scale(&self, domain: f32) -> f32 {
        1f32
    }

    /// Get the bandwidth (if present).
    fn bandwidth(&self) -> Option<f32> {
        None
    }

    /// Get the max value of the range.
    fn max_range(&self) -> f32 {
        self.range.1
    }

    /// Get the list of ticks that represent the scale on a chart axis.
    fn get_ticks(&self, orientation: Orientation) -> Vec<AxisTick> {
        Vec::new()
    }

}