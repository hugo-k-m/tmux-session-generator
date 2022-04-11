//! Test helpers

use crate::err::CustomResult;
use std::{fs, io::Write, path::PathBuf};
use tempfile::TempDir;

pub trait TestObject {
    fn setup(group_option: Option<bool>) -> CustomResult<Self>
    where
        Self: Sized;
}

pub struct TestHomeDir {
    pub test_home_path: PathBuf,
    _test_home_dir: TempDir,
}

impl TestObject for TestHomeDir {
    fn setup(_group_option: Option<bool>) -> CustomResult<Self> {
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
    fn setup(_group_option: Option<bool>) -> CustomResult<Self> {
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
    fn setup(_group_option: Option<bool>) -> CustomResult<Self> {
        let test_home_dir = tempfile::tempdir()?;
        let test_home_dir_path = PathBuf::from(&test_home_dir.path());
        let test_tmuxsg_path = test_home_dir_path.join(".tmuxsg");
        let test_session_path = test_tmuxsg_path.join("test_session");
        let script_path = test_session_path.join(&format!("test_session_0.sh"));

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
    fn setup(group_option: Option<bool>) -> CustomResult<Self> {
        let session_group_option = if let Some(_) = group_option {
            "is_session_group"
        } else {
            "is_not_session_group"
        };

        let test_home_dir = tempfile::tempdir()?;
        let test_home_dir_path = PathBuf::from(&test_home_dir.path());
        let test_tmuxsg_path = test_home_dir_path.join(".tmuxsg");
        let test_session_path = test_tmuxsg_path.join("test_session");
        let group_script_path = test_session_path.join(&format!("__session_group_option.sh"));
        let script_path = test_session_path.join(&format!("test_session_0.sh"));

        fs::create_dir(&test_tmuxsg_path)?;
        fs::create_dir(&test_session_path)?;

        let mut group_script = fs::File::create(group_script_path)?;

        group_script.write_all(session_group_option.as_bytes())?;

        fs::File::create(script_path)?;

        Ok(TestSessionDirGroupScript {
            test_tmuxsg_path,
            test_session_path,
            _test_home_dir: test_home_dir,
        })
    }
}

pub struct TestSessionDirGroupMultipleScripts {
    pub test_tmuxsg_path: PathBuf,
    pub test_session_path: PathBuf,
    _test_home_dir: TempDir,
}

fn create_test_scripts(
    number_of_scripts: usize,
    session_name: String,
    test_session_path: &PathBuf,
) -> CustomResult<()> {
    let mut script_path: PathBuf;

    for i in 0..number_of_scripts {
        script_path = test_session_path.join(&format!("{}_{}.sh", session_name, i));
        fs::File::create(script_path)?;
    }

    Ok(())
}

impl TestObject for TestSessionDirGroupMultipleScripts {
    fn setup(group_option: Option<bool>) -> CustomResult<Self> {
        let session_group_option = if let Some(_) = group_option {
            "is_session_group"
        } else {
            "is_not_session_group"
        };

        let test_home_dir = tempfile::tempdir()?;
        let test_home_dir_path = PathBuf::from(&test_home_dir.path());
        let test_tmuxsg_path = test_home_dir_path.join(".tmuxsg");
        let test_session_path = test_tmuxsg_path.join("test_session");
        let group_script_path = test_session_path.join(&format!("__session_group_option.sh"));

        fs::create_dir(&test_tmuxsg_path)?;
        fs::create_dir(&test_session_path)?;

        let mut group_script = fs::File::create(group_script_path)?;
        group_script.write_all(session_group_option.as_bytes())?;

        create_test_scripts(3, "first_test_session".to_owned(), &test_session_path)?;
        create_test_scripts(10, "second_test_session".to_owned(), &test_session_path)?;
        create_test_scripts(5, "third_test_session".to_owned(), &test_session_path)?;

        Ok(TestSessionDirGroupMultipleScripts {
            test_tmuxsg_path,
            test_session_path,
            _test_home_dir: test_home_dir,
        })
    }
}

#[cfg(test)]
mod tests {
    use std::fs;

    use crate::err::CustomResult;

    use super::{TestObject, TestSessionDirGroupMultipleScripts};

    #[test]
    fn group_multiple_scripts_test() -> CustomResult<()> {
        let tsg_test = TestSessionDirGroupMultipleScripts::setup(None)?;
        let session_dir = tsg_test.test_session_path;
        let dir_size = fs::read_dir(&session_dir)?.count().to_owned();

        assert_eq!(19, dir_size);
        assert!(session_dir.join("first_test_session_1.sh").is_file());
        assert!(session_dir.join("second_test_session_9.sh").is_file());
        assert!(session_dir.join("third_test_session_3.sh").is_file());

        Ok(())
    }
}
