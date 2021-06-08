//! Test helpers

use std::path::PathBuf;
use tempfile::TempDir;

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
