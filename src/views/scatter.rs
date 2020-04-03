use std::collections::HashMap;
use svg::parser::Error;
use svg::node::Node;
use svg::node::element::Group;
use crate::components::scatter::{ScatterPoint, MarkerType};
use crate::colors::Color;
use crate::Scale;
use crate::views::datum::PointDatum;
use crate::views::View;
use crate::components::DatumRepresentation;

/// A View that represents data as a scatter plot.
pub struct ScatterView<'a, T, U> {
    labels_visible: bool,
    marker_type: MarkerType,
    entries: Vec<ScatterPoint>,
    colors: Vec<Color>,
    keys: Vec<String>,
    color_map: HashMap<String, String>,
    x_scale: Option<&'a dyn Scale<T>>,
    y_scale: Option<&'a dyn Scale<U>>,
}

impl<'a, T, U> ScatterView<'a, T, U> {
    /// Create a new empty instance of the view.
    pub fn new() -> Self {
        Self {
            labels_visible: true,
            marker_type: MarkerType::Circle,
            entries: Vec::new(),
            keys: Vec::new(),
            colors: Color::color_scheme_10(),
            color_map: HashMap::new(),
            x_scale: None,
            y_scale: None,
        }
    }

    /// Set the scale for the X dimension.
    pub fn set_x_scale(mut self, scale: &'a impl Scale<T>) -> Self {
        self.x_scale = Some(scale);
        self
    }

    /// Set the scale for the Y dimension.
    pub fn set_y_scale(mut self, scale: &'a impl Scale<U>) -> Self {
        self.y_scale = Some(scale);
        self
    }

    /// Set the keys in case of a stacked bar chart.
    pub fn set_keys(mut self, keys: Vec<String>) -> Self {
        self.keys = keys;
        self
    }

    /// Set the keys in case of a stacked bar chart.
    pub fn set_marker_type(mut self, marker_type: MarkerType) -> Self {
        self.marker_type = marker_type;
        self
    }

    /// Hide labels on the chart.
    pub fn do_not_show_labels(mut self) -> Self {
        self.labels_visible = false;
        self
    }

    /// Load and process a dataset of BarDatum points.
    pub fn load_data(mut self, data: &Vec<impl PointDatum<T, U>>) -> Result<Self, String> {
        match self.x_scale {
            Some(_) => {},
            _ => return Err("Please provide a scale for the X dimension before loading data".to_string()),
        }
        match self.y_scale {
            Some(_) => {},
            _ => return Err("Please provide a scale for the Y dimension before loading data".to_string()),
        }

        // If no keys were explicitly provided, extract the keys from the data.
        if self.keys.len() == 0 {
            self.keys = Self::extract_keys(&data);
        }

        // Organize entries based on the order of the keys first, since displayed data
        // should keep the order defined in the `keys` attribute.
        for (i, key) in self.keys.iter_mut().enumerate() {
            // Map the key to the corresponding color.
            self.color_map.insert(key.clone(), self.colors[i].as_hex());
        }

        for datum in data.iter() {
            let scaled_x = self.x_scale.unwrap().scale(&datum.get_x());
            let scaled_y = self.y_scale.unwrap().scale(&datum.get_y());
            let y_bandwidth_offset = {
                if self.y_scale.unwrap().is_range_reversed() {
                    -self.y_scale.unwrap().bandwidth().unwrap() / 2_f32
                } else {
                    self.y_scale.unwrap().bandwidth().unwrap() / 2_f32
                }
            };
            let x_bandwidth_offset = {
                if self.x_scale.unwrap().is_range_reversed() {
                    -self.x_scale.unwrap().bandwidth().unwrap() / 2_f32
                } else {
                    self.x_scale.unwrap().bandwidth().unwrap() / 2_f32
                }
            };
            self.entries.push(ScatterPoint::new(scaled_x + x_bandwidth_offset, scaled_y + y_bandwidth_offset, self.marker_type, 5, self.color_map.get(&datum.get_key()).unwrap().clone()));
        }

        Ok(self)
    }

    /// Extract the list of keys to use when stacking and coloring the bars.
    fn extract_keys(data: &Vec<impl PointDatum<T, U>>) -> Vec<String> {
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

}

impl<'a, T, U> View<'a> for ScatterView<'a, T, U> {
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
