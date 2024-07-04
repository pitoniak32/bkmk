use std::io;

use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Could not edit bookmark with name [{0}]")]
    FailedEditingBookmark(String),

    #[error("Could not create temp edit dir: {0}")]
    FailedCreatingTempEditDir(io::Error),

    #[error("Could not create temp edit file: {0}")]
    FailedCreatingTempEditFile(io::Error),

    #[error("Could not cleanup temp edit file: {0}")]
    FailedCleaningupTempEditFile(io::Error),

    #[error("Failed to write temp edit contents: {0}")]
    FailedWritingTempEditContents(io::Error),

    #[error("Failed to convert Bookmark struct into string")]
    FailedSerializingBookmark,

    #[error("Failed to convert edited json to Bookmark object")]
    FailedDeserializingBookmark,

    #[error("Failed to use editor to update bookmark")]
    FailedEditorUpdateBookmark,
}
