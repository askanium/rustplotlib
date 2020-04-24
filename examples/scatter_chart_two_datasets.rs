use charts::{Chart, ScaleLinear, ScatterView, MarkerType, PointLabelPosition, Color, AxisPosition};

fn main() {
    // Define chart related sizes.
    let width = 800;
    let height = 600;
    let (top, right, bottom, left) = (90, 40, 80, 60);

    // Create a band scale that will interpolate values in [0, 200] to values in the
    // [0, availableWidth] range (the width of the chart without the margins).
    let x = ScaleLinear::new()
        .set_domain(vec![0_f32, 200_f32])
        .set_range(vec![0, width - left - right]);

    // Create a linear scale that will interpolate values in [0, 100] range to corresponding
    // values in [availableHeight, 0] range (the height of the chart without the margins).
    // The [availableHeight, 0] range is inverted because SVGs coordinate system's origin is
    // in top left corner, while chart's origin is in bottom left corner, hence we need to invert
    // the range on Y axis for the chart to display as though its origin is at bottom left.
    let y = ScaleLinear::new()
        .set_domain(vec![0_f32, 100_f32])
        .set_range(vec![height - top - bottom, 0]);

    // You can use your own iterable as data as long as its items implement the `PointDatum` trait.
    let scatter_data_1 = vec![(20, 90), (12, 54), (25, 70), (33, 40)];
    let scatter_data_2 = vec![(120, 10), (143, 34), (170, 14), (190, 13)];

    // Create Scatter view that is going to represent the data as points.
    let scatter_view_1 = ScatterView::new()
        .set_x_scale(&x)
        .set_y_scale(&y)
        .set_marker_type(MarkerType::Circle)
        .set_label_position(PointLabelPosition::N)
        .set_custom_data_label("Apples".to_owned())
        .load_data(&scatter_data_1).unwrap();

    // Create Scatter view that is going to represent the data as points.
    let scatter_view_2 = ScatterView::new()
        .set_x_scale(&x)
        .set_y_scale(&y)
        .set_marker_type(MarkerType::Square)
        .set_label_position(PointLabelPosition::N)
        .set_custom_data_label("Oranges".to_owned())
        .set_colors(Color::from_vec_of_hex_strings(vec!["#aa0000"]))
        .load_data(&scatter_data_2).unwrap();

    // Generate and save the chart.
    Chart::new()
        .set_width(width)
        .set_height(height)
        .set_margins(top, right, bottom, left)
        .add_title(String::from("Scatter Chart"))
        .add_view(&scatter_view_1)
        .add_view(&scatter_view_2)
        .add_axis_bottom(&x)
        .add_axis_left(&y)
        .add_left_axis_label("Custom X Axis Label")
        .add_bottom_axis_label("Custom Y Axis Label")
        .add_legend_at(AxisPosition::Bottom)
        .save("scatter-chart-two-datasets.svg").unwrap();
}