//! Test helpers

use crate::err::CustomResult;
use std::{fs, path::PathBuf};
use tempfile::TempDir;

pub trait TestObject {
    fn setup() -> CustomResult<Self>
    where
        Self: Sized;
}

pub struct HomeTestObject {
    pub test_home_path: PathBuf,
    _test_home_dir: TempDir,
}

impl TestObject for HomeTestObject {
    fn setup() -> CustomResult<Self> {
        let test_home_dir = tempfile::tempdir()?;
        let test_home_path = PathBuf::from(&test_home_dir.path());

        Ok(HomeTestObject {
            test_home_path,
            _test_home_dir: test_home_dir,
        })
    }
}

pub struct SessionTestObject {
    pub test_tmuxsg_path: PathBuf,
    pub test_home_dir_path: PathBuf,
    _test_home_dir: TempDir,
}

impl TestObject for SessionTestObject {
    fn setup() -> CustomResult<Self> {
        let test_home_dir = tempfile::tempdir()?;
        let test_home_dir_path = PathBuf::from(&test_home_dir.path());
        let test_tmuxsg_path = test_home_dir_path.join(".tmuxsg");

        fs::create_dir(&test_tmuxsg_path)?;

        Ok(SessionTestObject {
            test_tmuxsg_path,
            test_home_dir_path,
            _test_home_dir: test_home_dir,
        })
    }
}

pub struct WindowTestObject {
    pub test_tmuxsg_path: PathBuf,
    pub test_session_path: PathBuf,
    _test_home_dir: TempDir,
}

impl TestObject for WindowTestObject {
    fn setup() -> CustomResult<Self> {
        let test_home_dir = tempfile::tempdir()?;
        let test_home_dir_path = PathBuf::from(&test_home_dir.path());
        let test_tmuxsg_path = test_home_dir_path.join(".tmuxsg");
        let test_session_path = test_tmuxsg_path.join("test_session");
        let script_path = test_session_path.join(&format!("test_session.sh"));

        fs::create_dir(&test_tmuxsg_path)?;
        fs::create_dir(&test_session_path)?;
        fs::File::create(script_path)?;

        Ok(WindowTestObject {
            test_tmuxsg_path,
            test_session_path,
            _test_home_dir: test_home_dir,
        })
    }
}
