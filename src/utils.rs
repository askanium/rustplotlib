/// A struct to represent a range.
#[derive(Debug)]
pub struct Range(pub(crate) f32, pub(crate) f32);

impl Range {
    pub fn new(start: f32, end: f32) -> Self {
        Self(start, end)
    }

    pub fn default() -> Self {
        Self(0f32, 1f32)
    }
}

/// Define the orientation enum to aid in rendering and business logic.
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Orientation {
    Horizontal,
    Vertical,
}
