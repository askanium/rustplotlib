use charts::{Chart, VerticalBarView, ScaleBand, ScaleLinear};
use std::fs::File;
use std::io::Read;

fn main() -> std::io::Result<()> {
    let mut file = File::open("./sources/letter_frequency.txt")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    // Convert the data into a vec of (&str, f32) tuples for which the `BarDatum`
    // trait is implemented and which can be displayed as a bar chart.
    let mut data = contents.split("\n")
        .collect::<Vec<&str>>()
        .iter_mut()
        .map(|row| {
            let letter_freq_pair = row.split(",").collect::<Vec<&str>>();
            (letter_freq_pair[0], letter_freq_pair[1].parse::<f32>().unwrap() * 100_f32)
        })
        .collect::<Vec<(&str, f32)>>();

    data.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());

    let width = 800;
    let height = 600;
    let (top, right, bottom, left) = (70, 10, 50, 60);

    let x = ScaleBand::new()
        .set_domain(data.iter().map(|d| String::from(d.0)).collect())
        .set_range(vec![0, width - left - right]);

    let y = ScaleLinear::new()
        .set_domain(vec![0_f32, data.iter().map(|d| d.1.ceil() as isize).max().unwrap() as f32])
        .set_range(vec![height - top - bottom, 0]);

    let view = VerticalBarView::new()
        .set_x_scale(&x)
        .set_y_scale(&y)
        .set_label_rounding_precision(1)
        .load_data(&data).unwrap();

    Chart::new()
        .set_width(width)
        .set_height(height)
        .set_margins(top, right, bottom, left)
        .add_title(String::from("Frequency of English Letters"))
        .add_view(&view)
        .add_axis_bottom(&x)
        .add_axis_left(&y)
        .add_left_axis_label("Frequency (%)")
        .save("letter-frequency.svg").unwrap();

    Ok(())
}