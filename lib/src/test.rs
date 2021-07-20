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
        let test_home_path = PathBuf::from(&test_home_dir.path());

        Ok(HomeTestObjects {
            test_home_path,
            _test_home_dir: test_home_dir,
        })
    }
}

pub struct SessionTestObjects {
    pub test_tmuxsg_path: PathBuf,
    _test_home_dir: TempDir,
}

impl SessionTestObjects {
    pub fn setup() -> Result<Self, Box<dyn std::error::Error>> {
        let test_home_dir = tempfile::tempdir()?;
        let test_home_dir_path = PathBuf::from(&test_home_dir.path());
        let test_tmuxsg_path = test_home_dir_path.join(".tmuxsg");

        fs::create_dir(&test_tmuxsg_path)?;

        Ok(SessionTestObjects {
            test_tmuxsg_path,
            _test_home_dir: test_home_dir,
        })
    }
}

pub struct WindowTestObjects {
    pub test_tmuxsg_path: PathBuf,
    pub test_session_path: PathBuf,
    _test_home_dir: TempDir,
}

impl WindowTestObjects {
    pub fn setup() -> Result<Self, Box<dyn std::error::Error>> {
        let test_home_dir = tempfile::tempdir()?;
        let test_home_dir_path = PathBuf::from(&test_home_dir.path());
        let test_tmuxsg_path = test_home_dir_path.join(".tmuxsg");
        let test_session_path = test_tmuxsg_path.join("test_session");
        let script_path = test_session_path.join(&format!("test_session.sh"));

        fs::create_dir(&test_tmuxsg_path)?;
        fs::create_dir(&test_session_path)?;
        fs::File::create(script_path)?;

        Ok(WindowTestObjects {
            test_tmuxsg_path,
            test_session_path,
            _test_home_dir: test_home_dir,
        })
    }
}
