//! Test helpers

use std::{fs, path::PathBuf};
use tempfile::TempDir;

pub struct HomeTestObjects {
    pub test_home_path: PathBuf,
    _test_home_dir: TempDir,
}

impl HomeTestObjects {
    pub fn setup() -> Result<Self, Box<dyn std::error::Error>> {
        let test_home_dir = tempfile::tempdir()?;
        let test_home_dir_path = PathBuf::from(&test_home_dir.path());

        Ok(HomeTestObjects {
            test_home_path: test_home_dir_path,
            _test_home_dir: test_home_dir,
        })
    }
}
