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

    /// Takes a value x in [a, b] and returns the corresponding value in [0, 1].
    fn normalize(&self, a: f32, b: f32, x: f32) -> f32 {
        let b = b - a;
        match b {
            // If a == b then return 0.5
            0f32 => 0.5,
            _ => (x - a) / b
        }
    }

    /// Takes a value t in [0, 1] and returns the corresponding range in [a, b].
    fn interpolate(&self, a: f32, b: f32, t: f32) -> f32 {
        (b - a) * t + a
    }

    fn rescale(&mut self) {
    }
}

impl Scale<f32> for ScaleLinear {
    /// Set the domain limits for the scale band.
    fn set_domain(&mut self, range: Vec<f32>) {
        self.domain = range;
    }

    /// Get the domain limits of the scale.
    fn domain(&self) -> &Vec<f32> {
        &self.domain
    }

    /// Set the range limits for the scale band.
    fn set_range(&mut self, range: Range) {
        if range.0 > range.1 {
            self.range = Range(range.1, range.0);
        } else {
            self.range = range;
        }
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
        let a = self.domain[0];
        let b = self.domain[1];
        let normalized = self.normalize(a, b, domain);
        let Range(a, b) = self.range;
        let scaled = self.interpolate(a, b, normalized);

        scaled
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