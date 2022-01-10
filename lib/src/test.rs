//! Test helpers

use crate::err::CustomResult;
use std::{fs, path::PathBuf};
use tempfile::TempDir;

pub trait TestObject {
    fn setup() -> CustomResult<Self>
    where
        Self: Sized;
}

pub struct TestHomeDir {
    pub test_home_path: PathBuf,
    _test_home_dir: TempDir,
}

impl TestObject for TestHomeDir {
    fn setup() -> CustomResult<Self> {
        let test_home_dir = tempfile::tempdir()?;
        let test_home_path = PathBuf::from(&test_home_dir.path());

        Ok(TestHomeDir {
            test_home_path,
            _test_home_dir: test_home_dir,
        })
    }
}

pub struct TestTmuxHomeDir {
    pub test_tmuxsg_path: PathBuf,
    pub test_home_dir_path: PathBuf,
    _test_home_dir: TempDir,
}

impl TestObject for TestTmuxHomeDir {
    fn setup() -> CustomResult<Self> {
        let test_home_dir = tempfile::tempdir()?;
        let test_home_dir_path = PathBuf::from(&test_home_dir.path());
        let test_tmuxsg_path = test_home_dir_path.join(".tmuxsg");

        fs::create_dir(&test_tmuxsg_path)?;

        Ok(TestTmuxHomeDir {
            test_tmuxsg_path,
            test_home_dir_path,
            _test_home_dir: test_home_dir,
        })
    }
}

pub struct TestSessionDir {
    pub test_tmuxsg_path: PathBuf,
    pub test_session_path: PathBuf,
    _test_home_dir: TempDir,
}

impl TestObject for TestSessionDir {
    fn setup() -> CustomResult<Self> {
        let test_home_dir = tempfile::tempdir()?;
        let test_home_dir_path = PathBuf::from(&test_home_dir.path());
        let test_tmuxsg_path = test_home_dir_path.join(".tmuxsg");
        let test_session_path = test_tmuxsg_path.join("test_session");
        let script_path = test_session_path.join(&format!("test_session.sh"));

        fs::create_dir(&test_tmuxsg_path)?;
        fs::create_dir(&test_session_path)?;
        fs::File::create(script_path)?;

        Ok(TestSessionDir {
            test_tmuxsg_path,
            test_session_path,
            _test_home_dir: test_home_dir,
        })
    }
}

pub struct TestSessionDirGroupScript {
    pub test_tmuxsg_path: PathBuf,
    pub test_session_path: PathBuf,
    _test_home_dir: TempDir,
}

impl TestObject for TestSessionDirGroupScript {
    fn setup() -> CustomResult<Self> {
        let test_home_dir = tempfile::tempdir()?;
        let test_home_dir_path = PathBuf::from(&test_home_dir.path());
        let test_tmuxsg_path = test_home_dir_path.join(".tmuxsg");
        let test_session_path = test_tmuxsg_path.join("test_session");
        let group_script_path = test_session_path.join(&format!("__session_group_option.sh"));
        let script_path = test_session_path.join(&format!("test_session.sh"));

        fs::create_dir(&test_tmuxsg_path)?;
        fs::create_dir(&test_session_path)?;
        fs::File::create(group_script_path)?;
        fs::File::create(script_path)?;

        Ok(TestSessionDirGroupScript {
            test_tmuxsg_path,
            test_session_path,
            _test_home_dir: test_home_dir,
        })
    }
}
