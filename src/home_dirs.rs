//! Handle home directories

use directories::BaseDirs;
use lib::{err::DirectoryError, produce_directory_error};
use std::{fs, path::PathBuf};

pub fn tmuxsg_home_dir(home_d: PathBuf) -> Result<PathBuf, Box<dyn std::error::Error>> {
    let error = "Tmux home".to_owned();
    let tsg_home = home_d.join(".tmuxsg");

    if !tsg_home.is_dir() {
        fs::create_dir(&tsg_home)?;
    } else {
        produce_directory_error!(error);
    }

    Ok(tsg_home)
}

pub fn home_directory(base_dirs: Option<BaseDirs>) -> Result<PathBuf, Box<dyn std::error::Error>> {
    let base_d = match base_dirs {
        Some(bd) => bd,
        None => {
            produce_directory_error!("Home".to_owned());
        }
    };

    Ok(base_d.home_dir().to_owned())
}

#[cfg(test)]
mod tests {
    use crate::home_dirs::{home_directory, tmuxsg_home_dir};
    use directories::BaseDirs;
    use lib::test::{HomeTestObject, TestObject};
    use std::path::PathBuf;

    #[test]
    fn create_tmuxsg_home_directory_success() -> Result<(), Box<dyn std::error::Error>> {
        let tsg_test = HomeTestObject::setup()?;
        let home_d = tsg_test.test_home_path;
        let tsg_home_expected = PathBuf::from(&format!("{}/.tmuxsg", home_d.display()));
        tmuxsg_home_dir(home_d)?;

        assert!(tsg_home_expected.is_dir());

        Ok(())
    }

    // TODO: rewrite as integration test
    #[test]
    fn tsg_home_directory_found() -> Result<(), Box<dyn std::error::Error>> {
        let home_d = home_directory(BaseDirs::new())?;
        let tsg_home_expected = PathBuf::from(&format!("{}/.tmuxsg", home_d.display()));

        assert!(tsg_home_expected.is_dir());

        Ok(())
    }

    // TODO: rewrite as integration test
    #[test]
    fn home_directory_found() -> Result<(), Box<dyn std::error::Error>> {
        let home_d = home_directory(BaseDirs::new())?;

        assert!(home_d.is_dir());

        Ok(())
    }

    #[test]
    fn home_directory_error() -> Result<(), Box<dyn std::error::Error>> {
        let actual_error_disp = format!("{}", home_directory(None).unwrap_err());
        let expected_error_disp = format!("Home directory error");

        assert_eq!(actual_error_disp, expected_error_disp);

        Ok(())
    }
}
