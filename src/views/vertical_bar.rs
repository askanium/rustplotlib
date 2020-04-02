use std::collections::HashMap;
use svg::parser::Error;
use svg::node::Node;
use svg::node::element::Group;
use crate::components::bar::{Bar, BarBlock, BarLabelPosition};
use crate::colors::Color;
use crate::{Scale, BarDatum};
use crate::scales::ScaleType;
use crate::components::DatumRepresentation;
use crate::views::View;
use crate::utils::Orientation;

/// A Dataset that represents data that should be visualized.
pub struct VerticalBarView<'a> {
    label_position: BarLabelPosition,
    labels_visible: bool,
    entries: Vec<Bar>,
    keys: Vec<String>,
    colors: Vec<Color>,
    color_map: HashMap<String, String>,
    x_scale: Option<&'a dyn Scale<String>>,
    y_scale: Option<&'a dyn Scale<f32>>,
}

impl<'a> VerticalBarView<'a> {
    /// Create a new empty instance of the dataset.
    pub fn new() -> Self {
        Self {
            label_position: BarLabelPosition::EndOutside,
            labels_visible: true,
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

    /// Set the positioning of the labels.
    pub fn set_label_position(mut self, label_position: BarLabelPosition) -> Self {
        self.label_position = label_position;
        self
    }
    /// Hide labels on the chart.
    pub fn do_not_show_labels(mut self) -> Self {
        self.labels_visible = false;
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
        let y_range_is_reversed = self.y_scale.unwrap().is_range_reversed();

        for (category, key_value_pairs) in categories.iter_mut() {
            let mut bar_blocks = Vec::new();
            let mut stacked_start = self.y_scale.unwrap().range_end();
            let mut stacked_end = stacked_start;

            for (key, value) in key_value_pairs.iter() {
                let scaled_value;

                // If Y axis' scale has the range in reversed order, then adjust the computation of
                // the start and end positions to account for SVG coordinate system origin.
                if y_range_is_reversed {
                    scaled_value = self.y_scale.unwrap().range_start() - self.y_scale.unwrap().scale(value);
                    stacked_start = stacked_end;
                    stacked_end += scaled_value;
                } else {
                    scaled_value = self.y_scale.unwrap().scale(value);
                    stacked_end = stacked_start;
                    stacked_start -= scaled_value;
                }
                bar_blocks.push(BarBlock::new(stacked_start, stacked_end, *value, self.color_map.get(*key).unwrap().clone()));
            }

            let bar = Bar::new(bar_blocks, Orientation::Vertical, category.to_string(), self.label_position, self.labels_visible, self.x_scale.unwrap().bandwidth().unwrap(), self.x_scale.unwrap().scale(category));
            bars.push(bar);
        }

        for bar in bars {
            self.add_bar(bar);
        }

        Ok(self)
    }

    /// Extract the list of keys to use when stacking and coloring the bars.
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

    /// Add a [Bar] entry to the dataset entries list.
    fn add_bar(&mut self, bar: Bar) {
        self.entries.push(bar);
    }

}

impl<'a> View<'a> for VerticalBarView<'a> {
    /// Generate the SVG representation of the view.
    fn to_svg(&self) -> Result<Group, Error> {
        let mut group = Group::new();

        for entry in self.entries.iter() {
            let child_svg = entry.to_svg()?;
            group.append(child_svg);
        }

        Ok(group)
    }
}
