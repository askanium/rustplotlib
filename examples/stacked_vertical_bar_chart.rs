use charts::{Chart, VerticalBarView, ScaleBand, ScaleLinear, BarLabelPosition};

fn main() {
    // Define chart related sizes.
    let width = 800;
    let height = 600;
    let (top, right, bottom, left) = (90, 40, 50, 60);

    // Create a band scale that maps ["A", "B", "C"] categories to values in [0, availableWidth]
    // range (the width of the chart without the margins).
    let x = ScaleBand::new()
        .set_domain(vec![String::from("A"), String::from("B"), String::from("C")])
        .set_range(vec![0, width - left - right]);

    // Create a linear scale that will interpolate values in [0, 100] range to corresponding
    // values in [availableHeight, 0] range (the height of the chart without the margins).
    // The [availableHeight, 0] range is inverted because SVGs coordinate system's origin is
    // in the top left corner, while chart's origin is in bottom left corner, hence we need to
    // invert the range on Y axis for the chart to display as though its origin is at bottom left.
    let y = ScaleLinear::new()
        .set_domain(vec![0_f32, 100_f32])
        .set_range(vec![height - top - bottom, 0]);

    // You can use your own iterable as data as long as its items implement the `BarDatum` trait.
    let data = vec![("A", 70, "foo"), ("B", 10, "foo"), ("C", 30, "foo"), ("A", 20, "bar"), ("A", 5, "baz")];

    // Create VerticalBar view that is going to represent the data as vertical bars.
    let view = VerticalBarView::new()
        .set_x_scale(&x)
        .set_y_scale(&y)
        // .set_label_visibility(false)  // <-- uncomment this line to hide bar value labels
        .set_label_position(BarLabelPosition::Center)
        .load_data(&data).unwrap();

    // Generate and save the chart.
    Chart::new()
        .set_width(width)
        .set_height(height)
        .set_margins(top, right, bottom, left)
        .add_title(String::from("Stacked Bar Chart"))
        .add_view(&view)
        .add_axis_bottom(&x)
        .add_axis_left(&y)
        .add_left_axis_label("Units of Measurement")
        .add_bottom_axis_label("Categories")
        .save("stacked-vertical-bar-chart.svg").unwrap();
}