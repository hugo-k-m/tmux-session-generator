//! Helpers for tests

use std::path::PathBuf;

use tempfile::TempDir;

use crate::err::DirectoryError;

pub struct CreationTest {
    pub path: PathBuf,
    _tempdir: TempDir,
}

impl CreationTest {
    pub fn setup() -> Result<Self, Box<dyn std::error::Error>> {
        let tempdir = tempfile::tempdir()?;
        let home_d = PathBuf::from(&tempdir.path());

        Ok(CreationTest {
            path: home_d,
            _tempdir: tempdir,
        })
    }
}

/// Returns result used in directory error test
pub fn _directory_error() -> Result<(), DirectoryError> {
    Err(DirectoryError)
}

/// Produces directory error
pub fn _produce_directory_error() {
    if let Err(e) = _directory_error() {
        eprintln!("{}", e)
    }
}
