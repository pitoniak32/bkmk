use clap::Parser;

use std::collections::HashMap;
use std::fs::File;
use std::path::{Path, PathBuf};

use crate::bookmark::Bookmark;
use crate::cli::run;
use crate::cli::types::Cli;

pub mod bookmark;
pub mod cli;

fn main() {
    let cli = Cli::parse();

    env_logger::builder()
        .filter_level(cli.verbosity.log_level_filter())
        .init();

    log::trace!("{cli:#?}");

    let file_path = PathBuf::from("./bookmarks.json");
    let mut bookmarks: HashMap<String, Bookmark> = load(&file_path);

    let result = run::run(cli, &mut bookmarks);

    save(&bookmarks, &file_path);

    if result.is_err() {
        log::error!("{result:#?}");
        std::process::exit(1);
    }
}

fn save(bookmarks: &HashMap<String, Bookmark>, file_path: &Path) {
    serde_json::to_writer_pretty(
        File::create(file_path).expect("file should be able to be created"),
        &bookmarks,
    )
    .expect("should be able to be serialized into a writer");
}

fn load(file_path: &Path) -> HashMap<String, Bookmark> {
    let bookmarks: HashMap<String, Bookmark> = serde_json::from_str(
        &std::fs::read_to_string(file_path).expect("file should be able to be read"),
    )
    .expect("should be able to be deserialized into map of bookmarks");
    bookmarks
}
