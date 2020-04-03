/// A struct that represents a color.
#[derive(Debug)]
pub struct Color {
    hex: String,
}

impl Color {
    /// Generate a color scheme from a string.
    /// Useful when displaying a single dataset that requires one color.
    pub fn from_vec_of_hex_strings(color_strings: Vec<&str>) -> Vec<Self> {
        let mut colors = Vec::new();

        for color in color_strings.iter() {
            colors.push(Color { hex: String::from(*color) })
        }

        colors
    }

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

    /// An array of ten categorical colors authored by Tableau as part of
    /// [Tableau 10](https://www.tableau.com/about/blog/2016/7/colors-upgrade-tableau-10-56782).
    pub fn color_scheme_tableau_10() -> Vec<Self> {
        vec!(
            Color { hex: "#4e79a7".to_string() },
            Color { hex: "#f28e2c".to_string() },
            Color { hex: "#e15759".to_string() },
            Color { hex: "#76b7b2".to_string() },
            Color { hex: "#59a14f".to_string() },
            Color { hex: "#edc949".to_string() },
            Color { hex: "#af7aa1".to_string() },
            Color { hex: "#ff9da7".to_string() },
            Color { hex: "#9c755f".to_string() },
            Color { hex: "#bab0ab".to_string() },
        )
    }

    /// An array of eight categorical colors
    pub fn color_scheme_dark() -> Vec<Self> {
        vec!(
            Color { hex: "#1b9e77".to_string() },
            Color { hex: "#d95f02".to_string() },
            Color { hex: "#7570b3".to_string() },
            Color { hex: "#e7298a".to_string() },
            Color { hex: "#66a61e".to_string() },
            Color { hex: "#e6ab02".to_string() },
            Color { hex: "#a6761d".to_string() },
            Color { hex: "#666666".to_string() },
        )
    }

    /// Represent a color as a hex string.
    pub fn as_hex(&self) -> String {
        String::from(&self.hex)
    }
}
