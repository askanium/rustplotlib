use charts::{Chart, VerticalBarView, ScaleBand, ScaleLinear, AxisPosition, Color};
use std::fs::File;
use std::io::Read;

fn main() -> std::io::Result<()> {
    // source [RIAA](https://www.riaa.com/u-s-sales-database/)
    let mut file = File::open("./sources/music.csv")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    // Convert the data into a vec of (&str, f32, &str) tuples for which the `BarDatum`
    // trait is implemented and which can be displayed as a stacked bar chart.
    let data = contents.split("\n")
        .collect::<Vec<&str>>()
        .iter_mut()
        .enumerate()
        .filter_map(|(i, row)| {
            if i > 0 {
                let cells = row.split(",").collect::<Vec<&str>>();
                Some((cells[1], cells[4].parse::<isize>().unwrap(), cells[0]))
            } else {
                None
            }
        })
        .collect::<Vec<(&str, isize, &str)>>();

    let width = 960;
    let height = 600;
    let (top, right, bottom, left) = (180, 10, 50, 60);

    let x = ScaleBand::new()
        .set_domain(data.iter().map(|d| String::from(d.0)).collect())
        .set_range(vec![0, width - left - right]);

    let y = ScaleLinear::new()
        .set_domain(vec![0_f32, 22_000_000_000_f32])
        .set_range(vec![height - top - bottom, 0]);

    let view = VerticalBarView::new()
        .set_x_scale(&x)
        .set_y_scale(&y)
        .set_label_visibility(false)
        .set_keys(vec![String::from("LP/EP"), String::from("Vinyl Single"), String::from("8 - Track"), String::from("Cassette"), String::from("Cassette Single"), String::from("Other Tapes"), String::from("Kiosk"), String::from("CD"), String::from("CD Single"), String::from("SACD"), String::from("DVD Audio"), String::from("Music Video (Physical)"), String::from("Download Album"), String::from("Download Single"), String::from("Ringtones and Ringbacks"), String::from("Download Music Video"), String::from("Other Digital"), String::from("Synchronization"), String::from("Paid Subscription"), String::from("On-Demand Streaming (Ad-Supported)"), String::from("Other Ad-Supported Streaming"), String::from("SoundExchange Distributions"), String::from("Limited Tier Paid Subscription")])
        .set_colors(Color::from_vec_of_hex_strings(vec!["#2A5784", "#43719F", "#5B8DB8", "#7AAAD0", "#9BC7E4", "#BADDF1", "#E1575A", "#EE7423", "#F59D3D", "#FFC686", "#9D7760", "#F1CF63", "#7C4D79", "#9B6A97", "#BE89AC", "#D5A5C4", "#EFC9E6", "#BBB1AC", "#24693D", "#398949", "#61AA57", "#7DC470", "#B4E0A7"]))
        .load_data(&data).unwrap();

    Chart::new()
        .set_width(width)
        .set_height(height)
        .set_margins(top, right, bottom, left)
        .add_view(&view)
        .add_axis_bottom(&x)
        .add_axis_left(&y)
        .add_legend_at(AxisPosition::Top)
        .set_bottom_axis_tick_label_rotation(-90)
        .set_left_axis_tick_label_format(".2s")
        .add_left_axis_label("Revenue ($)")
        .save("revenue-by-music-format.svg").unwrap();

    Ok(())
}