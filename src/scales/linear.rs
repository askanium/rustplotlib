use std::cmp::max;
use std::fmt;
use std::hash::Hash;
use std::collections::HashMap;
use crate::utils::{Range, Orientation};
use crate::scales::{Scale, ScaleType};
use crate::components::axis::AxisTick;

/// The scale to represent categorical data.
#[derive(Debug)]
pub struct ScaleLinear {
    /// The domain limits of the dataset that the scale is going to represent.
    domain: Vec<f32>,
    /// The range limits of the drawable area on the chart.
    range: Range,
    /// The amount of ticks to display.
    tick_count: usize,
}

impl ScaleLinear {
    /// Create a new linear scale with default values.
    pub fn new() -> Self {
        let mut scale = Self {
            domain: Vec::new(),
            range: Range::default(),
            tick_count: 10,
        };
        scale
    }

    /// Set the domain limits for the scale band.
    pub fn set_domain(mut self, range: Vec<f32>) -> Self {
        self.domain = range;
        self
    }

    /// Get the domain limits of the scale.
    pub fn domain(&self) -> &Vec<f32> {
        &self.domain
    }

    /// Set the range limits for the scale band.
    pub fn set_range(mut self, range: Range) -> Self {
        if range.0 > range.1 {
            self.range = Range(range.1, range.0);
        } else {
            self.range = range;
        }
        self
    }

    /// Get the range limits of the scale.
    pub fn range(&self) -> &Range {
        &self.range
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

    /// Compute the distance between the ticks.
    fn tick_step(&self, start: f32, stop: f32) -> f32 {
        let e10 = 50_f32.sqrt();
        let e5 = 10_f32.sqrt();
        let e2 = 2_f32.sqrt();
        let step = (stop - start) / max(0, self.tick_count) as f32;
        let power = (step.ln() / 10_f32.ln()).trunc() as i32;
        let error = step / 10_f32.powi(power);
        let dynamic = if error >= e10 {
            10
        } else if error >= e5 {
            5
        } else if error >= e2 {
            2
        } else {
            1
        };

        let step = match power.cmp(&0) {
            Less => -10_f32.powi(-power) / dynamic as f32,
            _ => dynamic as f32 * 10_f32.powi(power),
        };

        step
    }
}

impl Scale<f32> for ScaleLinear {
    /// Get the type of the scale.
    fn get_type(&self) -> ScaleType {
        ScaleType::Linear
    }

    /// Get the range value for the given domain entry.
    fn scale(&self, domain: &f32) -> f32 {
        let a = self.domain[0];
        let b = self.domain[1];
        let normalized = self.normalize(a, b, *domain);
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
    fn get_ticks(&self) -> Vec<f32> {
        let mut ticks = Vec::new();

        if self.domain[0] == self.domain[1] && self.tick_count > 0 {
            ticks.push(self.domain[0]);
            return ticks;
        }

        let step = self.tick_step(self.domain[0], self.domain[1]);
        let mut i = 0;
        if step > 0_f32 {
            let start = (self.domain[0] / step).ceil();
            let stop = (self.domain[1] / step).floor();
            let nr_of_ticks = (stop - start + 1_f32).ceil() as i32;
            while i < nr_of_ticks {
                ticks.push((start + i as f32) * step);
                i += 1;
            }
        } else {
            let start = (self.domain[0] * step).floor();
            let stop = (self.domain[1] * step).ceil();
            let nr_of_ticks = (start - stop + 1_f32).ceil() as i32;
            while i < nr_of_ticks {
                ticks.push((start - i as f32) / step);
                i += 1;
            }
        }

        ticks
    }
}