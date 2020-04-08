use svg::node::Node;
use svg::node::element::Group;
use crate::components::scatter::{ScatterPoint, MarkerType, PointLabelPosition};
use crate::colors::Color;
use crate::Scale;
use crate::views::datum::PointDatum;
use crate::views::View;
use crate::components::DatumRepresentation;
use std::fmt::Display;
use crate::components::legend::{LegendEntry, LegendMarkerType};
use crate::components::area::AreaSeries;

/// A View that represents data as a scatter plot.
pub struct AreaSeriesView<'a, T: Display + Clone, U: Display + Clone> {
    labels_visible: bool,
    label_position: PointLabelPosition,
    marker_type: MarkerType,
    entries: Vec<AreaSeries<T, U>>,
    colors: Vec<Color>,
    x_scale: Option<&'a dyn Scale<T>>,
    y_scale: Option<&'a dyn Scale<U>>,
    custom_data_label: String,
}

impl<'a, T: Display + Clone, U: Display + Clone> AreaSeriesView<'a, T, U> {
    /// Create a new empty instance of the view.
    pub fn new() -> Self {
        Self {
            labels_visible: true,
            label_position: PointLabelPosition::NW,
            marker_type: MarkerType::Circle,
            entries: Vec::new(),
            colors: Color::color_scheme_10(),
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

        // Compute corresponding offsets to apply in case there is a non-zero bandwidth.
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

        let mut points = data.iter().map(|datum| {
            let scaled_x = self.x_scale.unwrap().scale(&datum.get_x());
            let scaled_y = self.y_scale.unwrap().scale(&datum.get_y());
            ScatterPoint::new(scaled_x + x_bandwidth_offset, scaled_y + y_bandwidth_offset, self.marker_type, 5, datum.get_x(), datum.get_y(), self.label_position, self.labels_visible, true, self.colors[0].as_hex())
        }).collect::<Vec<ScatterPoint<T, U>>>();

        let y_origin = {
            if self.y_scale.unwrap().is_range_reversed() {
                self.y_scale.unwrap().range_start()
            } else {
                self.y_scale.unwrap().range_end()
            }
        };
        let first = data.first().unwrap();
        let last = data.last().unwrap();
        points.push(ScatterPoint::new(self.x_scale.unwrap().scale(&last.get_x()) + x_bandwidth_offset, y_origin, self.marker_type, 5, data[0].get_x(), data[0].get_y(), self.label_position, false, false, "#fff".to_string()));
        points.push(ScatterPoint::new(self.x_scale.unwrap().scale(&first.get_x()) + x_bandwidth_offset, y_origin, self.marker_type, 5, data[0].get_x(), data[0].get_y(), self.label_position, false, false, "#fff".to_string()));

        self.entries.push(AreaSeries::new(points, self.colors[0].as_hex()));

        Ok(self)
    }
}

impl<'a, T: Display + Clone, U: Display + Clone> View<'a> for AreaSeriesView<'a, T, U> {
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

        // Area series currently does not support multiple keys per dataset,
        // hence when displaying a legend, it will display the custom data label
        // as the legend label.
        entries.push(LegendEntry::new(LegendMarkerType::Square, self.colors[0].as_hex(), String::from("none"), self.custom_data_label.clone()));

        entries
    }
}
