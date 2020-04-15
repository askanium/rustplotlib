use std::collections::HashMap;
use std::fmt::Display;
use svg::node::Node;
use svg::node::element::Group;
use crate::components::scatter::{ScatterPoint, MarkerType, PointLabelPosition};
use crate::colors::Color;
use crate::{Scale, LineSeries};
use crate::views::datum::PointDatum;
use crate::views::View;
use crate::components::DatumRepresentation;
use crate::components::legend::{LegendEntry, LegendMarkerType};

/// A View that represents data as a scatter plot.
pub struct LineSeriesView<'a, T: Display, U: Display> {
    labels_visible: bool,
    label_position: PointLabelPosition,
    marker_type: MarkerType,
    entries: Vec<LineSeries<T, U>>,
    colors: Vec<Color>,
    keys: Vec<String>,
    color_map: HashMap<String, String>,
    x_scale: Option<&'a dyn Scale<T>>,
    y_scale: Option<&'a dyn Scale<U>>,
    custom_data_label: String,
}

impl<'a, T: Display, U: Display> LineSeriesView<'a, T, U> {
    /// Create a new empty instance of the view.
    pub fn new() -> Self {
        Self {
            labels_visible: true,
            label_position: PointLabelPosition::NW,
            marker_type: MarkerType::Circle,
            entries: Vec::new(),
            keys: Vec::new(),
            colors: Color::color_scheme_10(),
            color_map: HashMap::new(),
            x_scale: None,
            y_scale: None,
            custom_data_label: String::new(),
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

    /// Set the positioning of the labels.
    pub fn set_label_position(mut self, label_position: PointLabelPosition) -> Self {
        self.label_position = label_position;
        self
    }

    /// Set the keys in case of a stacked bar chart.
    pub fn set_marker_type(mut self, marker_type: MarkerType) -> Self {
        self.marker_type = marker_type;
        self
    }

    /// Set the color palette of the view.
    pub fn set_colors(mut self, colors: Vec<Color>) -> Self {
        self.colors = colors;
        self
    }

    /// Set labels visibility.
    pub fn set_label_visibility(mut self, label_visibility: bool) -> Self {
        self.labels_visible = label_visibility;
        self
    }

    /// Set custom label for the dataset.
    /// This will work when the dataset represents only a single
    /// type of data (i.e. there are no different "keys" by which to
    /// differentiate data), otherwise, this will have no effect.
    pub fn set_custom_data_label(mut self, label: String) -> Self {
        self.custom_data_label = label;
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
            self.color_map.insert(key.clone(), self.colors[i % self.colors.len()].as_hex());
        }

        for key in self.keys.iter() {

            let points = data.iter().filter(|datum| &datum.get_key() == key).map(|datum| {
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
                ScatterPoint::new(scaled_x + x_bandwidth_offset, scaled_y + y_bandwidth_offset, self.marker_type, 5, datum.get_x(), datum.get_y(), self.label_position, self.labels_visible, true,self.color_map.get(&datum.get_key()).unwrap().clone())
            }).collect::<Vec<ScatterPoint<T, U>>>();

            self.entries.push(LineSeries::new(points, self.color_map.get(key).unwrap().clone()));
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

impl<'a, T: Display, U: Display> View<'a> for LineSeriesView<'a, T, U> {
    /// Generate the SVG representation of the view.
    fn to_svg(&self) -> Result<Group, String> {
        let mut group = Group::new();

        for entry in self.entries.iter() {
            let child_svg = entry.to_svg()?;
            group.append(child_svg);
        }

        Ok(group)
    }

    /// Return the legend entries that this view represents.
    fn get_legend_entries(&self) -> Vec<LegendEntry> {
        let mut entries = Vec::new();

        // If there is a single key and it is an empty string (meaning
        // the dataset consists only of X and Y dimension values), return
        // the custom data label.
        if self.keys.len() == 1 && self.keys[0].len() == 0 {
            entries.push(LegendEntry::new(LegendMarkerType::Line, self.color_map.get(&self.keys[0]).unwrap().clone(), String::from("none"), self.custom_data_label.clone()));
        } else {
            for key in self.keys.iter() {
                entries.push(LegendEntry::new(LegendMarkerType::Line, self.color_map.get(key).unwrap().clone(), String::from("none"), key.clone()));
            }
        }

        entries
    }
}
