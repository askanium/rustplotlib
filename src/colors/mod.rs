/// A struct that represents a color.
#[derive(Debug)]
pub struct Color {
    hex: String,
}

impl Color {
    /// Generate a color scheme made of 10 colors.
    pub fn color_scheme_10() -> Vec<Self> {
        vec!(
            Color { hex: "#1f77b4".to_string() },
            Color { hex: "#ff7f0e".to_string() },
            Color { hex: "#2ca02c".to_string() },
            Color { hex: "#d62728".to_string() },
            Color { hex: "#9467bd".to_string() },
            Color { hex: "#8c564b".to_string() },
            Color { hex: "#e377c2".to_string() },
            Color { hex: "#7f7f7f".to_string() },
            Color { hex: "#bcbd22".to_string() },
            Color { hex: "#17becf".to_string() },
        )
    }

    /// Represent a color as a hex string.
    pub fn as_hex(&self) -> String {
        String::from(&self.hex)
    }
}
