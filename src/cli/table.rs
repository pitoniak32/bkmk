use prettytable::format::consts::FORMAT_BOX_CHARS;
use prettytable::{row, Table};

use crate::bookmark::Bookmark;

pub fn build_table(bookmarks: &[&Bookmark]) -> Table {
    let mut table = Table::new();

    table.add_row(row!["Name", "Url", "Tags"]);
    bookmarks.iter().for_each(|bm| {
        table.add_row(row![
            bm.name,
            bm.url.clone().unwrap_or("NONE".to_string()),
            bm.tags.join(","),
        ]);
    });
    table.set_format(*FORMAT_BOX_CHARS);

    table
}
