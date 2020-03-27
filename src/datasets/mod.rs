use crate::components::DatumRepresentation;
use crate::components::bar::{Bar, BarBlock, BarOrientation};
use crate::{Render, ScaleBand};
use std::path::Path;
use svg::parser::Error;
use svg::node::Node;
use crate::datasets::datum::BarDatum;
use crate::scales::{Scale, ScaleType};
use std::collections::HashMap;
use crate::colors::Color;

mod datum;

/// A Dataset that represents data that should be visualized.
pub struct VerticalBarChartDataset<'a, T: AsRef<str>> {
    entries: Vec<Bar<T>>,
    categories: Vec<T>,
    keys: Vec<String>,
    colors: Vec<Color>,
    color_map: HashMap<String, String>,
    x_scale: Option<&'a dyn Scale<T>>,
    y_scale: Option<&'a dyn Scale<T>>,
}

impl<'a, T: std::cmp::Eq + std::hash::Hash + Copy + AsRef<str>> VerticalBarChartDataset<'a, T> {
    pub fn new() -> Self {
        Self {
            entries: Vec::new(),
            categories: Vec::new(),
            keys: vec!["".to_string()],
            colors: Color::color_scheme_10(),
            color_map: HashMap::new(),
            x_scale: None,
            y_scale: None,
        }
    }

    pub fn set_x_scale(&mut self, scale: &'a impl Scale<T>) {
        self.x_scale = Some(scale);
    }

    pub fn set_y_scale(&mut self, scale: &'a impl Scale<T>) {
        self.y_scale = Some(scale);
    }

    pub fn set_keys(&mut self, keys: Vec<String>) {
        self.keys = keys;
    }

    pub fn load_data(&mut self, data: &Vec<impl BarDatum<T>>) -> Result<(), &str> {
        match self.x_scale {
            Some(scale) if scale.get_type() == ScaleType::Band => {},
            _ => return Err("The X axis scale should be a Band scale."),
        }
        match self.y_scale {
            Some(scale) if scale.get_type() == ScaleType::Linear => {},
            _ => return Err("The Y axis scale should be a Linear scale."),
        }

        // HashMap to group all data related to a category. This is needed when there
        // are many data entries under a single category as in a stacked bar chart.
        let mut categories: HashMap<T, Vec<(&String, &f32)>> = HashMap::new();

        // Organize entries based on the order of the keys first, since displayed data
        // should keep the order defined in the `keys` attribute.
        for (i, key) in self.keys.iter_mut().enumerate() {
            // Map the key to the corresponding color.
            self.color_map.insert(key.clone(), self.colors[i].as_hex());

            for entry in data.iter() {
                if entry.get_key() == key {
                    if !categories.contains_key(entry.get_category()) {
                        categories.insert(*entry.get_category(), Vec::new());
                    }
                    if let Some(category_entries) = categories.get_mut(entry.get_category()) {
                        category_entries.push((key, entry.get_value()));
                    }
                }
            }
        }

        // Create a Bar entry for each category data that was grouped in the previous step.
        let mut bars = Vec::new();
        for (category, key_value_pairs) in categories.iter_mut() {
            let mut bar_blocks = Vec::new();
            let mut start = 200f32; // TODO set this to the height of the chart since SVG coordinate system is inversed.

            for (key, value) in key_value_pairs.iter() {
                start -= **value;
                bar_blocks.push(BarBlock::new(start, **value, self.color_map.get(*key).unwrap().clone()));
            }

            let bar = Bar::new(bar_blocks, BarOrientation::Vertical, *category, start.to_string(), self.x_scale.unwrap().bandwidth().unwrap(), self.x_scale.unwrap().scale(*category));
            bars.push(bar);
        }

        for bar in bars {
            self.add_bar(bar);
        }

        Ok(())
    }

    fn add_bar(&mut self, bar: Bar<T>) {
        self.entries.push(bar);
    }

    pub fn from_data(data: &Vec<impl BarDatum<T>>) -> Self {
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

    pub fn to_svg(&self) -> Result<svg::node::element::Group, Error> {
        let mut group = svg::node::element::Group::new();

        for entry in self.entries.iter() {
            let child_svg = entry.to_svg()?;
            group.append(child_svg);
        }
        svg::save("dataset.svg", &group);

        Ok(group)
    }

}

pub trait Dataset<'a> {
// pub trait Dataset<'a, T: DatumComponent> {
    // fn get_entries(&self) -> &Vec<T>;
}

// impl<'a> Render for dyn Dataset<'a> {
//     type SVGNode = svg::node::element::Group;
//
//     fn to_svg(&self) -> Result<svg::node::element::Group, Error> {
//         let group = svg::node::element::Group::new();
//
//         for entry in self.get_entries() {
//             group.add(entry.to_svg());
//         }
//         Ok(group)
//     }
// }

impl<'a, T: AsRef<str>> Dataset<'a> for VerticalBarChartDataset<'a, T> {
//     fn get_entries(&self) -> &Vec<Bar<String>> {
//         &self.entries
//     }
}