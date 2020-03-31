use std::collections::HashMap;
use svg::parser::Error;
use svg::node::Node;
use svg::node::element::Group;
use crate::components::bar::{Bar, BarBlock};
use crate::colors::Color;
use crate::{Scale, BarDatum};
use crate::scales::ScaleType;
use crate::components::DatumRepresentation;
use crate::datasets::Dataset;
use crate::utils::Orientation;

/// A Dataset that represents data that should be visualized.
pub struct VerticalBarDataset<'a> {
    entries: Vec<Bar>,
    keys: Vec<String>,
    colors: Vec<Color>,
    color_map: HashMap<String, String>,
    x_scale: Option<&'a dyn Scale<String>>,
    y_scale: Option<&'a dyn Scale<f32>>,
}

impl<'a> VerticalBarDataset<'a> {
    /// Create a new empty instance of the dataset.
    pub fn new() -> Self {
        Self {
            entries: Vec::new(),
            keys: Vec::new(),
            colors: Color::color_scheme_10(),
            color_map: HashMap::new(),
            x_scale: None,
            y_scale: None,
        }
    }

    /// Set the scale for the X dimension.
    pub fn set_x_scale(mut self, scale: &'a impl Scale<String>) -> Self {
        self.x_scale = Some(scale);
        self
    }

    /// Set the scale for the Y dimension.
    pub fn set_y_scale(mut self, scale: &'a impl Scale<f32>) -> Self {
        self.y_scale = Some(scale);
        self
    }

    /// Set the keys in case of a stacked bar chart.
    pub fn set_keys(mut self, keys: Vec<String>) -> Self {
        self.keys = keys;
        self
    }

    /// Load and process a dataset of BarDatum points.
    pub fn load_data(mut self, data: &Vec<impl BarDatum>) -> Result<Self, String> {
        match self.x_scale {
            Some(scale) if scale.get_type() == ScaleType::Band => {},
            _ => return Err("The X axis scale should be a Band scale.".to_string()),
        }
        match self.y_scale {
            Some(scale) if scale.get_type() == ScaleType::Linear => {},
            _ => return Err("The Y axis scale should be a Linear scale.".to_string()),
        }

        // If no keys were explicitly provided, extract the keys from the data.
        if self.keys.len() == 0 {
            self.keys = Self::extract_keys(&data);
        }

        // HashMap to group all data related to a category. This is needed when there
        // are many data entries under a single category as in a stacked bar chart.
        let mut categories: HashMap<String, Vec<(&String, f32)>> = HashMap::new();

        // Organize entries based on the order of the keys first, since displayed data
        // should keep the order defined in the `keys` attribute.
        for (i, key) in self.keys.iter_mut().enumerate() {
            // Map the key to the corresponding color.
            self.color_map.insert(key.clone(), self.colors[i].as_hex());

            for entry in data.iter() {
                if entry.get_key() == *key {
                    let entry_category = entry.get_category();

                    if !categories.contains_key(&entry_category) {
                        categories.insert(entry.get_category(), Vec::new());
                    }
                    if let Some(category_entries) = categories.get_mut(&entry_category) {
                        category_entries.push((key, entry.get_value()));
                    }
                }
            }
        }

        // Create a Bar entry for each category data that was grouped in the previous step.
        let mut bars = Vec::new();
        for (category, key_value_pairs) in categories.iter_mut() {
            let mut bar_blocks = Vec::new();
            let mut start = 530f32; // TODO set this to the height of the chart since SVG coordinate system is inversed.

            for (key, value) in key_value_pairs.iter() {
                let scaled_value = self.y_scale.unwrap().scale(value);
                bar_blocks.push(BarBlock::new(start - scaled_value, start, scaled_value, self.color_map.get(*key).unwrap().clone()));
                start -= scaled_value;
            }

            let bar = Bar::new(bar_blocks, Orientation::Vertical, category.to_string(), start.to_string(), self.x_scale.unwrap().bandwidth().unwrap(), self.x_scale.unwrap().scale(category));
            bars.push(bar);
        }

        for bar in bars {
            self.add_bar(bar);
        }

        Ok(self)
    }

    /// A shortcut method that will take care of creating the scales based on the data provided.
    pub fn from_data(data: &Vec<impl BarDatum>) -> Self {
        // TODO implement this method properly.
        Self {
            entries: Vec::new(),
            categories: Vec::new(),
            keys: Vec::new(),
            colors: Vec::new(),
            color_map: HashMap::new(),
            x_scale: None,
            y_scale: None,
        }
    }

    fn extract_keys(data: &Vec<impl BarDatum>) -> Vec<String> {
        let mut keys = Vec::new();
        let mut map = HashMap::new();

        for datum in data.iter() {
            match map.insert(datum.get_key(), 0) {
                Some(_) => {},
                None => keys.push(datum.get_key()),
            }
        }

        keys
    }

    /// Generate the SVG representation of the dataset.
    pub fn to_svg(&self) -> Result<svg::node::element::Group, Error> {
        let mut group = svg::node::element::Group::new();

        for entry in self.entries.iter() {
            let child_svg = entry.to_svg()?;
            group.append(child_svg);
        }

        Ok(group)
    }

    /// Add a [Bar] entry to the dataset entries list.
    fn add_bar(&mut self, bar: Bar) {
        self.entries.push(bar);
    }

}

impl<'a> Dataset<'a> for VerticalBarDataset<'a> {
    fn to_svg(&self) -> Result<Group, Error> {
        Ok(self.to_svg().unwrap())
    }
}
