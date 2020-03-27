//! # View
//!
//! A [View] is a wrapper for several [Dataset]s. Its sole purpose is to
//! render the contained datasets in a chart.

use crate::datasets::Dataset;
use crate::components::DatumRepresentation;
use svg::node::element::Group;
use crate::Render;
use svg::parser::Error;
use svg;
use svg::Node;

/// The view struct that holds datasets to be rendered in the chart.
pub struct View<'a> {
    datasets: Vec<&'a dyn Dataset<'a>>,
}

impl<'a> View<'a> {
    pub fn new() -> Self {
        Self {
            datasets: Vec::new(),
        }
    }

    pub fn add_dataset(&mut self, dataset: &'a dyn Dataset<'a>) {
        self.datasets.push(dataset);
    }

    pub fn datasets(&self) -> &Vec<&'a dyn Dataset<'a>> {
        &self.datasets
    }
}

impl Render for View<'_> {
    fn to_svg(&self) -> Result<Group, Error> {
        let mut group = Group::new();

        for dataset in self.datasets.iter() {
            group.append(dataset.to_svg()?);
        }

        svg::save("view.svg", &group);

        Ok(group)
    }
}