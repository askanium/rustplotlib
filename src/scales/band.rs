use std::collections::HashMap;
use std::collections::HashSet;
use crate::scales::{Scale, ScaleType};

/// The scale to represent categorical data.
#[derive(Debug)]
pub struct ScaleBand {
    /// The domain limits of the dataset that the scale is going to represent.
    domain: Vec<String>,
    /// The range limits of the drawable area on the chart.
    range: Vec<isize>,
    /// The offsets of each entry from domain.
    offsets: Vec<f32>,
    /// The hash map that maps domain keys with corresponding offset entries.
    index: HashMap<String, usize>,
    /// The distance between the start of the first bar and the start of the next one.
    step: f32,
    /// The width of a bar.
    bandwidth: f32,
    /// The distance between bars as a percentage of the step (between 0 and 1).
    padding_inner: f32,
    /// The distance from the beginning/end of the chart to the first/last bar (between 0 and 1).
    padding_outer: f32,
    /// The distribution of the outer padding between the first and last bars (between 0 and 1).
    /// An align value of 0.5 will distribute space evenly, while 0 will move all outer space to
    /// the right part, leaving no space on the left.
    align: f32,
    /// The start value of the range.
    r0: f32,
    /// The end value of the range.
    r1: f32,
}

impl ScaleBand {
    /// Create a new band scale with default values.
    pub fn new() -> Self {
        Self {
            domain: Vec::new(),
            range: vec![0, 1],
            offsets: Vec::new(),
            index: HashMap::new(),
            step: 1f32,
            bandwidth: 1f32,
            padding_inner: 0.1,
            padding_outer: 0.1,
            align: 0.5,
            r0: 0f32,
            r1: 0f32,
        }
    }

    /// Set the inner padding ratio.
    pub fn set_inner_padding(mut self, padding: f32) -> Self {
        self.padding_inner = padding;
        self.rescale();
        self
    }

    /// Set the outer padding ratio.
    pub fn set_outer_padding(mut self, padding: f32) -> Self {
        self.padding_outer = padding;
        self.rescale();
        self
    }

    /// Set the domain limits for the scale band.
    pub fn set_domain(mut self, range: Vec<String>) -> Self {
        // Deduplicate the domain range and keep order of entries.
        let mut unique = Vec::new();
        let mut set: HashSet<String> = HashSet::new();

        for el in range.into_iter() {
            let clone = el.clone();
            if !set.contains(&clone) {
                set.insert(clone);
                unique.push(el);
            }
        }

        self.domain = unique;
        self.rescale();
        self
    }

    /// Get the domain limits of the scale.
    pub fn domain(&self) -> &Vec<String> {
        &self.domain
    }

    /// Set the range limits for the scale band.
    pub fn set_range(mut self, range: Vec<isize>) -> Self {
        self.range = range;
        self.rescale();
        self
    }

    /// Get the range limits of the scale.
    pub fn range(&self) -> &Vec<isize> {
        &self.range
    }

    fn rescale(&mut self) {
        let n = self.domain.len();
        let r0 = self.range[0];
        let r1 = self.range[1];
        let reverse = r1 < r0;
        let mut start = r0 as f32;
        let mut stop = r1 as f32;

        if reverse {
            self.range = vec![r1, r0];
            start = r1 as f32;
            stop = r0 as f32;
        }

        let step_denominator = {
            let computed_step = n as f32 - self.padding_inner + self.padding_outer * 2f32;
            if computed_step > 1f32 {
                computed_step
            } else {
                1f32
            }
        };
        self.step = (stop - start) / step_denominator;

        // TODO implement rounding of step, start and bandwidth values if specified by user.

        start += (stop - start - self.step * (n as f32 - self.padding_inner)) * self.align;

        self.bandwidth = self.step * (1f32 - self.padding_inner);

        self.offsets.clear();
        for i in 0..n {
            self.offsets.push(start + self.step * i as f32);
        }

        if reverse {
            self.offsets.reverse();
        }

        self.index.clear();
        let mut processed_domains = Vec::new();
        for domain in self.domain.iter() {
            // Check for already existing keys to remove any duplicates in the domain vector.
            if !self.index.contains_key(domain) {
                self.index.insert(domain.clone(), processed_domains.len());
                processed_domains.push(domain.clone());
            }
        }
        // Re-assign domains with any duplicates removed.
        self.domain.clear();
        self.domain = processed_domains;
    }
}

impl Scale<String> for ScaleBand {
    /// Get the type of the scale.
    fn get_type(&self) -> ScaleType {
        ScaleType::Band
    }

    /// Get the range value for the given domain entry.
    fn scale(&self, domain: &String) -> f32 {
        self.offsets[*self.index.get(domain).unwrap()]
    }

    /// Get the bandwidth (if present).
    fn bandwidth(&self) -> Option<f32> {
        Some(self.bandwidth)
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
    fn get_ticks(&self) -> Vec<String> {
        self.domain.clone()
    }
}