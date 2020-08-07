use std::string::ToString;
use std::ffi::OsStr;
use std::path::Path;
use svg;
use svg::node::element::Group;
use svg::Node;
use svg::node::Text as TextNode;
use svg::node::element::Text;
use crate::{Axis, Scale};
use crate::views::View;
use crate::axis::AxisPosition;
use crate::legend::Legend;
use crate::components::legend::LegendEntry;

/// Define the orientation enum to aid in rendering and business logic.
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Orientation {
    Horizontal,
    Vertical,
}

/// The Chart struct definition.
/// A Chart is the smallest entity that can be saved (the bigger one is a Page (TBD)).
pub struct Chart<'a> {
    margin_top: isize,
    margin_bottom: isize,
    margin_right: isize,
    margin_left: isize,
    width: isize,
    height: isize,
    x_axis_top: Option<Axis>,
    x_axis_bottom: Option<Axis>,
    y_axis_left: Option<Axis>,
    y_axis_right: Option<Axis>,
    legend_position: Option<AxisPosition>,
    views: Vec<&'a dyn View<'a>>,
    title: String,
}

impl<'a> Chart<'a> {
    /// Create a new instance of a chart with default sizes.
    pub fn new() -> Self {
        Self {
            margin_top: 90,
            margin_bottom: 50,
            margin_right: 40,
            margin_left: 60,
            width: 800,
            height: 600,
            x_axis_top: None,
            x_axis_bottom: None,
            y_axis_left: None,
            y_axis_right: None,
            legend_position: None,
            views: Vec::new(),
            title: String::new(),
        }
    }

    /// Set chart width.
    pub fn set_width(mut self, width: isize) -> Self {
        self.width = width;
        self
    }

    /// Set chart height.
    pub fn set_height(mut self, height: isize) -> Self {
        self.height = height;
        self
    }

    /// Add chart title.
    pub fn add_title(mut self, title: String) -> Self {
        self.title = title;
        self
    }

    /// Set the margins of the chart to provided values.
    pub fn set_margins(mut self, top: isize, right: isize, bottom: isize, left: isize) -> Self {
        self.margin_top = top;
        self.margin_right = right;
        self.margin_bottom = bottom;
        self.margin_left = left;
        self
    }

    /// Add the dataset to the chart's view.
    pub fn add_view(mut self, view: &'a dyn View<'a>) -> Self {
        self.views.push(view);
        self
    }

    /// Add an axis at the bottom of the chart.
    pub fn add_axis_bottom<T: ToString>(mut self, scale: &'a dyn Scale<T>) -> Self {
        self.x_axis_bottom = Some(Axis::new_bottom_axis(scale, &self));
        self
    }

    /// Add an axis at the left of the chart.
    pub fn add_axis_left<T: ToString>(mut self, scale: &'a dyn Scale<T>) -> Self {
        self.y_axis_left = Some(Axis::new_left_axis(scale, &self));
        self
    }

    /// Add an axis at the top of the chart.
    pub fn add_axis_top<T: ToString>(mut self, scale: &'a dyn Scale<T>) -> Self {
        self.x_axis_top = Some(Axis::new_top_axis(scale, &self));
        self
    }

    /// Add an axis at the right of the chart.
    pub fn add_axis_right<T: ToString>(mut self, scale: &'a dyn Scale<T>) -> Self {
        self.y_axis_right = Some(Axis::new_right_axis(scale, &self));
        self
    }

    /// Add a label for the right of the chart.
    pub fn add_right_axis_label<T: ToString>(mut self, label: T) -> Self {
        if let Some(ref mut axis) = self.y_axis_right {
            axis.set_axis_label(label.to_string())
        } else {
            panic!("You cannot add a label to right axis without adding an axis first.")
        }
        self
    }

    /// Add a label for the left of the chart.
    pub fn add_left_axis_label<T: ToString>(mut self, label: T) -> Self {
        if let Some(ref mut axis) = self.y_axis_left {
            axis.set_axis_label(label.to_string())
        } else {
            panic!("You cannot add a label to left axis without adding an axis first.")
        }
        self
    }

    /// Add a label for the top of the chart.
    pub fn add_top_axis_label<T: ToString>(mut self, label: T) -> Self {
        if let Some(ref mut axis) = self.x_axis_top {
            axis.set_axis_label(label.to_string())
        } else {
            panic!("You cannot add a label to top axis without adding an axis first.")
        }
        self
    }

    /// Add a label for the bottom of the chart.
    pub fn add_bottom_axis_label<T: ToString>(mut self, label: T) -> Self {
        if let Some(ref mut axis) = self.x_axis_bottom {
            axis.set_axis_label(label.to_string())
        } else {
            panic!("You cannot add a label to bottom axis without adding an axis first.")
        }
        self
    }

    /// Return the offset from the left where the view starts.
    pub fn get_view_horizontal_start_offset(&self) -> isize {
        self.margin_left
    }

    /// Return the offset from the left where the view ends.
    pub fn get_view_horizontal_end_offset(&self) -> isize {
        self.width - self.margin_right
    }

    /// Return the offset from the left where the view starts.
    pub fn get_view_vertical_start_offset(&self) -> isize {
        self.margin_top
    }

    /// Return the offset from the left where the view ends.
    pub fn get_view_vertical_end_offset(&self) -> isize {
        self.width - self.margin_bottom
    }

    /// Return the width of the view.
    pub fn get_view_width(&self) -> isize {
        self.width - self.margin_left - self.margin_right
    }

    /// Return the width of the chart.
    pub fn get_chart_width(&self) -> isize {
        self.width
    }

    /// Return the height of the chart.
    pub fn get_chart_height(&self) -> isize {
        self.height
    }

    /// Return the height of the view.
    pub fn get_view_height(&self) -> isize {
        self.height - self.margin_top - self.margin_bottom
    }

    /// Set legend position at the specified side of the chart.
    pub fn add_legend_at(mut self, position: AxisPosition) -> Self {
        self.legend_position = Some(position);
        self
    }

    /// Set the rotation in degrees of the bottom axis tick labels.
    pub fn set_bottom_axis_tick_label_rotation(mut self, rotation: isize) -> Self {
        match &mut self.x_axis_bottom {
            Some(axis) => axis.set_tick_label_rotation(rotation),
            None => {},
        }
        self
    }

    /// Set the rotation in degrees of the top axis tick labels.
    pub fn set_top_axis_tick_label_rotation(mut self, rotation: isize) -> Self {
        match &mut self.x_axis_top {
            Some(axis) => axis.set_tick_label_rotation(rotation),
            None => {},
        }
        self
    }

    /// Set the rotation in degrees of the left axis tick labels.
    pub fn set_left_axis_tick_label_rotation(mut self, rotation: isize) -> Self {
        match &mut self.y_axis_left {
            Some(axis) => axis.set_tick_label_rotation(rotation),
            None => {},
        }
        self
    }

    /// Set the rotation in degrees of the right axis tick labels.
    pub fn set_right_axis_tick_label_rotation(mut self, rotation: isize) -> Self {
        match &mut self.y_axis_right {
            Some(axis) => axis.set_tick_label_rotation(rotation),
            None => {},
        }
        self
    }

    /// Set the format type of labels on the left axis.
    pub fn set_left_axis_tick_label_format(mut self, format: &str) -> Self {
        match &mut self.y_axis_left {
            Some(axis) => axis.set_tick_label_format(format),
            None => {},
        }
        self
    }

    /// Set the format type of labels on the right axis.
    pub fn set_right_axis_tick_label_format(mut self, format: &str) -> Self {
        match &mut self.y_axis_right {
            Some(axis) => axis.set_tick_label_format(format),
            None => {},
        }
        self
    }

    /// Set the format type of labels on the top axis.
    pub fn set_top_axis_tick_label_format(mut self, format: &str) -> Self {
        match &mut self.x_axis_top {
            Some(axis) => axis.set_tick_label_format(format),
            None => {},
        }
        self
    }

    /// Set the format type of labels on the bottom axis.
    pub fn set_bottom_axis_tick_label_format(mut self, format: &str) -> Self {
        match &mut self.x_axis_bottom {
            Some(axis) => axis.set_tick_label_format(format),
            None => {},
        }
        self
    }

    /// Generate the SVG for the chart and its components.
    pub fn to_svg(&self) -> Result<Group, String> {
        let mut group = Group::new()
            .set("class", "g-chart");

        // Add chart title
        if self.title.len() > 0 {
            let title_group = Group::new()
                .set("class", "g-title")
                .set("transform", format!("translate({},{})", self.width / 2, 25))
                .add(Text::new()
                    .set("x", 0)
                    .set("y", 0)
                    .set("dy", ".35em")
                    .set("fill", "#777")
                    .set("text-anchor", "middle")
                    .set("font-size", "24px")
                    .set("font-family", "sans-serif")
                    .add(TextNode::new(&self.title))
                );
            group.append(title_group);
        }

        if let Some(ref axis) = self.x_axis_top {
            let mut axis_group = axis.to_svg().unwrap();
            axis_group.assign("transform", format!("translate({},{})", self.margin_left, self.margin_top));
            group.append(axis_group);
        };

        if let Some(ref axis) = self.x_axis_bottom {
            let mut axis_group = axis.to_svg().unwrap();
            axis_group.assign("transform", format!("translate({},{})", self.margin_left, self.height - self.margin_bottom));
            group.append(axis_group);
        };

        if let Some(ref axis) = self.y_axis_left {
            let mut axis_group = axis.to_svg().unwrap();
            axis_group.assign("transform", format!("translate({},{})", self.margin_left, self.margin_top));
            group.append(axis_group);
        };

        if let Some(ref axis) = self.y_axis_right {
            let mut axis_group = axis.to_svg().unwrap();
            axis_group.assign("transform", format!("translate({},{})", self.width - self.margin_right, self.margin_top));
            group.append(axis_group);
        };

        let mut view_group = Group::new()
            .set("class", "g-view")
            .set("transform", format!("translate({},{})", self.margin_left, self.margin_top));

        for view in self.views.iter() {
            view_group.append(view.to_svg()?);
        }
        group.append(view_group);

        if let Some(legend_position) = self.legend_position {
            let width;
            let x_offset;
            let y_offset;

            match legend_position {
                AxisPosition::Top => {
                    let axis_height = {
                        if self.title.len() > 0 {
                            45
                        } else {
                            10
                        }
                    };
                    width = self.width - self.margin_right - self.margin_left;
                    x_offset = self.margin_left;
                    y_offset = axis_height;
                },
                AxisPosition::Bottom => {
                    // Compute the height of the bottom axis that should serve
                    // as an offset for the legend.
                    // These sizes are hardcoded and work with current hardcoded axis fonts.
                    // When a necessity will arise to have custom axis font sizes,
                    // these will have to be dynamically computed.
                    let axis_height = {
                        if let Some(ref axis) = self.x_axis_bottom {
                            if axis.has_label() {
                                52
                            } else {
                                36
                            }
                        } else {
                            16
                        }
                    };
                    width = self.width - self.margin_right - self.margin_left;
                    x_offset = self.margin_left;
                    y_offset = self.height - self.margin_bottom + axis_height;
                },
                AxisPosition::Left => {
                    let axis_width = {
                        if let Some(ref axis) = self.y_axis_left {
                            if axis.has_label() {
                                68
                            } else {
                                50
                            }
                        } else {
                            20
                        }
                    };
                    width = self.margin_left - axis_width - 10; // 10 is described in the comment below
                    x_offset = 10; // always have a 10px padding from the left of the chart
                    y_offset = self.margin_top;
                },
                AxisPosition::Right => {
                    let axis_width = {
                        if let Some(ref axis) = self.y_axis_right {
                            if axis.has_label() {
                                68
                            } else {
                                50
                            }
                        } else {
                            20
                        }
                    };
                    width = self.margin_right - axis_width;
                    x_offset = self.width - self.margin_right + axis_width;
                    y_offset = self.margin_top;
                }
            };

            let legend_entries = self.views.iter().map(|view| view.get_legend_entries()).flatten().collect::<Vec<LegendEntry>>();
            let legend = Legend::new(legend_entries, width as usize);
            let mut legend_group = legend.to_svg()?;
            legend_group.assign("transform", format!("translate({},{})", x_offset, y_offset));

            group.append(legend_group);
        }

        Ok(group)
    }

    /// Save the chart to a file
    pub fn save<P>(self, path: P) -> Result<(), String> where
        P: AsRef<Path>
    {
        match path.as_ref().extension().and_then(OsStr::to_str) {
            Some("svg") => {
                match self.to_svg() {
                    Ok(svg_content) => {
                        let document = svg::Document::new()
                            .set("width", self.width)
                            .set("height", self.height)
                            .set("viewBox", (0, 0, self.width, self.height))
                            .add(svg_content);

                        svg::save(path, &document).unwrap()
                    },
                    Err(e) => return Err(format!("Encountered an error while saving the chart: {:?}", e)),
                }
            },
            _ => {},
        };
        Ok(())
    }
}
