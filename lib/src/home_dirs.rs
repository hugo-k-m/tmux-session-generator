//! Handle home directories

use std::{fs, path::PathBuf};

use directories::BaseDirs;

use crate::err::DirectoryError;

pub fn tmuxsg_home_dir(home_d: PathBuf) -> Result<PathBuf, Box<dyn std::error::Error>> {
    let tsg_home = home_d.join(".tmuxsg");

    if !tsg_home.is_dir() {
        fs::create_dir(&tsg_home)?;
    }

    Ok(tsg_home)
}

pub fn home_directory() -> Result<PathBuf, Box<dyn std::error::Error>> {
    let base_d = match BaseDirs::new() {
        Some(bd) => bd,
        None => {
            return Err(Box::new(DirectoryError));
        }
    };

    Ok(base_d.home_dir().to_owned())
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use crate::{
        home_dirs::{home_directory, tmuxsg_home_dir},
        test::CreationTest,
    };

    #[test]
    fn create_tmuxsg_home_directory_success() -> Result<(), Box<dyn std::error::Error>> {
        let tsg_test = CreationTest::setup()?;
        let home_d = tsg_test.path;
        let tsg_home_expected = PathBuf::from(&format!("{}/.tmuxsg", &home_d.display()));
        tmuxsg_home_dir(home_d)?;

        assert!(tsg_home_expected.is_dir());

        Ok(())
    }

    #[test]
    fn tsg_home_directory_found() -> Result<(), Box<dyn std::error::Error>> {
        let home_d = home_directory()?;
        let tsg_home_expected = PathBuf::from(&format!("{}/.tmuxsg", &home_d.display()));

        assert!(tsg_home_expected.is_dir());

        Ok(())
    }

    #[test]
    fn home_directory_found() -> Result<(), Box<dyn std::error::Error>> {
        let home_d = home_directory()?;

        assert!(home_d.is_dir());

        Ok(())
    }
}
