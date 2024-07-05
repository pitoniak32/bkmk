use anyhow::Result;
use std::collections::HashMap;
use std::fs::File;
use std::io::Write;
use std::path::{Path, PathBuf};

use crate::bookmark::Bookmark;
use crate::cli::table::build_table;

use super::error::Error;
use super::types::{Cli, Command};

pub fn run(cli: Cli, bookmarks: &mut HashMap<String, Bookmark>) -> Result<()> {
    match cli.command {
        Command::Add {
            name,
            url,
            description,
            tags,
        } => {
            bookmarks.insert(name.clone(), Bookmark::new(name, url, description, tags));
        }
        Command::List { filter_tags, table } => {
            let filtered_bookmarks = bookmarks
                .iter()
                .filter(|(_k, v)| {
                    filter_tags.iter().any(|ft| {
                        v.tags
                            .iter()
                            .map(|vt| vt.to_lowercase())
                            .collect::<Vec<_>>()
                            .contains(&ft.to_lowercase())
                    })
                })
                .collect::<HashMap<_, _>>();

            log::debug!("filtered: {filtered_bookmarks:#?}",);

            if table {
                let bms = filtered_bookmarks.values().copied().collect::<Vec<_>>();

                println!("{}", build_table(&bms))
            }
        }
        Command::Edit { name } => {
            let result = if let Some(found) = bookmarks.get(&name) {
                log::trace!("bookmark found!: {found:#?}");
                let result = edit_bookmark(found)?;
                log::trace!("Edit bookmark successful!: {result:#?}");
                Some(result)
            } else {
                log::warn!("No bookmark found for name {name}!");
                None
            };
            if let Some(bm) = result {
                let removed = bookmarks.remove(&name);
                log::trace!("removed bookmark: {removed:#?}");
                bookmarks.insert(bm.name.clone(), bm.clone());
                log::trace!("inserted bookmark: {bm:#?}");
            }
        }
    };

    Ok(())
}

pub fn open_nvim(file_path: &Path) -> Result<(), Error> {
    log::trace!("starting default editor with temp file: {file_path:?}");
    let editor = std::env::var("EDITOR").unwrap_or("vi".to_string());
    let output = std::process::Command::new(editor)
        .arg(file_path)
        .spawn()
        .expect("default editor should spawn")
        .wait_with_output()
        .expect("default editor should finish successfully");

    if output.status.success() {
        Ok(())
    } else {
        Err(Error::FailedEditorUpdateBookmark)
    }
}

fn edit_bookmark(bookmark: &Bookmark) -> Result<Bookmark, Error> {
    let temp_edit_file = create_temp_file(
        serde_json::to_string_pretty(&bookmark).map_err(|_| Error::FailedSerializingBookmark)?,
    )?;
    open_nvim(&temp_edit_file)?;
    let bookmark = serde_json::from_str::<Bookmark>(
        &std::fs::read_to_string(&temp_edit_file)
            .expect("temp file content can be read into a string"),
    )
    .map_err(|e| {
        log::error!("{e}");
        Error::FailedDeserializingBookmark
    })?;
    std::fs::remove_file(temp_edit_file).map_err(Error::FailedCleaningupTempEditFile)?;
    Ok(bookmark)
}

fn create_temp_file(content: String) -> Result<PathBuf, Error> {
    let temp_dir = std::env::temp_dir();
    let file_path = temp_dir.join("bookmark_edit_file.json");
    log::trace!("creating temp edit file: {file_path:?}");
    let mut file = File::create(&file_path).map_err(Error::FailedCreatingTempEditFile)?;
    log::trace!("writing bookmark content to temp file: {file_path:?}");
    writeln!(file, "{}", content).map_err(Error::FailedWritingTempEditContents)?;

    Ok(file_path)
}
