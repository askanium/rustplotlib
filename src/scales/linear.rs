use crate::utils::Range;
use std::cmp::max;
use crate::scales::{Scale, ScaleType};
use std::fmt;
use std::hash::Hash;
use std::collections::HashMap;

/// The scale to represent categorical data.
#[derive(Debug)]
pub struct ScaleLinear<T> {
    /// The domain limits of the dataset that the scale is going to represent.
    domain: Vec<T>,
    /// The range limits of the drawable area on the chart.
    range: Range,
}

impl<T> ScaleLinear<T> {
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

impl<T> Scale<T> for ScaleLinear<T> {
    /// Set the domain limits for the scale band.
    fn set_domain(&mut self, range: Vec<T>) {
        self.domain = range;
        self.rescale();
    }

    /// Get the domain limits of the scale.
    fn domain(&self) -> &Vec<T> {
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
    fn scale(&self, domain: T) -> f32 {
        1f32
    }

    /// Get the bandwidth (if present).
    fn bandwidth(&self) -> Option<f32> {
        None
    }
}