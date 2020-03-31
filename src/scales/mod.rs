use crate::utils::{Range, Orientation};
use crate::components::axis::AxisTick;

pub mod band;
pub mod linear;

#[derive(PartialEq)]
pub enum ScaleType {
    Band,
    Ordinal,
    Linear,
}

/// The Scale trait defines common operations on all scales.
pub trait Scale<T> {
    /// Get the type of the scale.
    fn get_type(&self) -> ScaleType;

    /// Get the range value for the given domain entry.
    fn scale(&self, domain: &T) -> f32;

    /// Get the bandwidth (if present).
    fn bandwidth(&self) -> Option<f32>;

    /// Get the max value of the range.
    fn max_range(&self) -> f32;

    /// Get the list of ticks that represent the scale on a chart axis.
    fn get_ticks(&self) -> Vec<T>;
}