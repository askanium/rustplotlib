use charts::{Chart, HorizontalBarView, ScaleBand, ScaleLinear, BarLabelPosition};

fn main() {
    // Define chart related sizes.
    let width = 800;
    let height = 600;
    let (top, right, bottom, left) = (90, 40, 50, 60);

    // Create a linear scale that will interpolate values in [0, 100] range to corresponding
    // values in [0, availableWidth] range (the width of the chart without the margins).
    let x = ScaleLinear::new()
        .set_domain(vec![0_f32, 100_f32])
        .set_range(vec![0, width - left - right]);

    // Create a band scale that maps ["A", "B", "C"] categories to values in the [0, availableHeight]
    // range (the height of the chart without the margins).
    let y = ScaleBand::new()
        .set_domain(vec![String::from("A"), String::from("B"), String::from("C")])
        .set_range(vec![0, height - top - bottom]);

    // You can use your own iterable as data as long as its items implement the `BarDatum` trait.
    let data = vec![("A", 70, "foo"), ("B", 10, "foo"), ("C", 30, "foo"), ("A", 20, "bar"), ("A", 5, "baz")];

    // Create VerticalBar view that is going to represent the data as vertical bars.
    let view = HorizontalBarView::new()
        .set_x_scale(&x)
        .set_y_scale(&y)
        .set_label_position(BarLabelPosition::Center)
        .load_data(&data).unwrap();

    // Generate and save the chart.
    Chart::new()
        .set_width(width)
        .set_height(height)
        .set_margins(top, right, bottom, left)
        .add_title(String::from("Horizontal Stacked Bar Chart"))
        .add_view(&view)
        .add_axis_bottom(&x)
        .add_axis_left(&y)
        .add_left_axis_label("Y Axis Custom Label")
        .add_bottom_axis_label("X Axis Custom Label")
        .save("stacked-horizontal-bar-chart.svg").unwrap();
}