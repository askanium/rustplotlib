use std::cmp::{max, Ordering};
use crate::scales::{Scale, ScaleType};

/// The scale to represent categorical data.
#[derive(Debug)]
pub struct ScaleLinear {
    /// The domain limits of the dataset that the scale is going to represent.
    domain: Vec<f32>,
    /// The range limits of the drawable area on the chart.
    range: Vec<isize>,
    /// The amount of ticks to display.
    tick_count: usize,
}

impl ScaleLinear {
    /// Create a new linear scale with default values.
    pub fn new() -> Self {
        Self {
            domain: Vec::new(),
            range: vec![0, 1],
            tick_count: 10,
        }
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
    pub fn set_range(mut self, range: Vec<isize>) -> Self {
        self.range = range;
        self
    }

    /// Get the range limits of the scale.
    pub fn range(&self) -> &Vec<isize> {
        &self.range
    }

    /// Takes a value x in [a, b] and returns the corresponding value in [0, 1].
    fn normalize(&self, a: f32, b: f32, x: f32) -> f32 {
        // If a == b then return 0.5
        if a == b {
            0.5
        } else {
            let b = b - a;
            (x - a as f32) / b as f32
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
            Ordering::Less => -10_f32.powi(-power) / dynamic as f32,
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
        let a = self.range[0] as f32;
        let b = self.range[1] as f32;
        let scaled = self.interpolate(a, b, normalized);

        scaled
    }

    /// Get the bandwidth (if present).
    fn bandwidth(&self) -> Option<f32> {
        Some(0_f32)
    }

    /// Get the start range value.
    fn range_start(&self) -> f32 {
        self.range[0] as f32
    }

    /// Get the end range value.
    fn range_end(&self) -> f32 {
        self.range[1] as f32
    }

    /// Get the list of ticks that represent the scale on a chart axis.
    fn get_ticks(&self) -> Vec<f32> {
        let mut ticks = Vec::new();

        if self.domain[0] == self.domain[1] && self.tick_count > 0 {
            ticks.push(self.domain[0] as f32);
            return ticks;
        }

        let step = self.tick_step(self.domain[0] as f32, self.domain[1] as f32);
        let mut i = 0;
        if step > 0_f32 {
            let start = (self.domain[0] as f32 / step).ceil();
            let stop = (self.domain[1] as f32 / step).floor();
            let nr_of_ticks = (stop - start + 1_f32).ceil() as i32;
            while i < nr_of_ticks {
                ticks.push((start + i as f32) * step);
                i += 1;
            }
        } else {
            let start = (self.domain[0] as f32 * step).floor();
            let stop = (self.domain[1] as f32 * step).ceil();
            let nr_of_ticks = (start - stop + 1_f32).ceil() as i32;
            while i < nr_of_ticks {
                ticks.push((start - i as f32) / step);
                i += 1;
            }
        }

        ticks
    }
}