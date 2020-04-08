use charts::{Chart, VerticalBarView, ScaleBand, ScaleLinear, ScatterView, MarkerType, Color, PointLabelPosition};

fn main() {
    // Define chart related sizes.
    let width = 800;
    let height = 600;
    let (top, right, bottom, left) = (90, 40, 50, 60);

    // Create a band scale that maps ["A", "B", "C"] categories to values in the [0, availableWidth]
    // range (the width of the chart without the margins).
    let x = ScaleBand::new()
        .set_domain(vec![String::from("A"), String::from("B"), String::from("C")])
        .set_range(vec![0, width - left - right]);

    // Create a linear scale that will interpolate values in [0, 100] range to corresponding
    // values in [availableHeight, 0] range (the height of the chart without the margins).
    // The [availableHeight, 0] range is inverted because SVGs coordinate system's origin is
    // in top left corner, while chart's origin is in bottom left corner, hence we need to invert
    // the range on Y axis for the chart to display as though its origin is at bottom left.
    let y = ScaleLinear::new()
        .set_domain(vec![0_f32, 100_f32])
        .set_range(vec![height - top - bottom, 0]);

    // You can use your own iterable as data as long as its items implement the `BarDatum` trait.
    let bar_data = vec![("A", 70), ("B", 10), ("C", 30)];

    // You can use your own iterable as data as long as its items implement the `PointDatum` trait.
    let scatter_data = vec![(String::from("A"), 90.3), (String::from("B"), 20.1), (String::from("C"), 10.8)];

    // Create VerticalBar view that is going to represent the data as vertical bars.
    let bar_view = VerticalBarView::new()
        .set_x_scale(&x)
        .set_y_scale(&y)
        .load_data(&bar_data).unwrap();

    // Create Scatter view that is going to represent the data as points.
    let scatter_view = ScatterView::new()
        .set_x_scale(&x)
        .set_y_scale(&y)
        .set_label_position(PointLabelPosition::NE)
        .set_marker_type(MarkerType::Circle)
        .set_colors(Color::from_vec_of_hex_strings(vec!["#FF4700"]))
        .load_data(&scatter_data).unwrap();

    // Generate and save the chart.
    Chart::new()
        .set_width(width)
        .set_height(height)
        .set_margins(top, right, bottom, left)
        .add_title(String::from("Composite Bar + Scatter Chart"))
        .add_view(&bar_view)                            // <-- add bar view
        .add_view(&scatter_view)                        // <-- add scatter view
        .add_axis_bottom(&x)
        .add_axis_left(&y)
        .add_left_axis_label("Units of Measurement")
        .add_bottom_axis_label("Categories")
        .save("composite-bar-and-scatter-chart.svg").unwrap();
}