use svg::node::element::Group;
use svg::Node;
use crate::components::legend::LegendEntry;

pub(crate) struct Legend {
    width: usize,
    entries: Vec<LegendEntry>,
}

impl Legend {
    /// Create a new legend instance.
    pub fn new(entries: Vec<LegendEntry>, width: usize) -> Self {
        Self {
            entries,
            width,
        }
    }

    pub fn to_svg(&self) -> Result<Group, String> {
        let mut group = Group::new().set("class", "g-legend");
        let max_entry_length = match self.entries.iter().map(|entry| entry.get_width()).max() {
            None => return Ok(group),
            Some(len) => len,
        };
        let gap_between_legend_entries = 10;
        let legend_row_height = 20;
        let mut current_row_offset = 0;
        let mut acc_row_width = 0;

        for entry in self.entries.iter() {
            if acc_row_width + max_entry_length > self.width && acc_row_width > 0 {
                acc_row_width = 0;
                current_row_offset += 1;
            }

            let mut entry_group = entry.to_svg()?;
            entry_group.assign("transform", format!("translate({},{})", acc_row_width, current_row_offset * legend_row_height));
            group.append(entry_group);

            acc_row_width += max_entry_length + gap_between_legend_entries;
        }

        Ok(group)
    }
}