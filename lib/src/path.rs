//! Functions used to check and/or fetch paths on disk

use crate::{create_dir, create_file};

use std::{fs, path::PathBuf};

use crate::err::DirectoryError;
use directories::BaseDirs;

pub fn script_path(s_dir: PathBuf, s_name: &str) -> Result<PathBuf, Box<dyn std::error::Error>> {
    let script_path = s_dir.join(&format!("{}.sh", s_name));

    create_file!(!&script_path.is_file(), &script_path);

    Ok(script_path)
}

pub fn session_dir(tsg_home: PathBuf, s_name: &str) -> Result<PathBuf, Box<dyn std::error::Error>> {
    let s_dir = tsg_home.join(s_name);

    create_dir!(!&s_dir.is_dir(), &s_dir);

    Ok(s_dir)
}

pub fn tmuxsg_home_dir(home_d: PathBuf) -> Result<PathBuf, Box<dyn std::error::Error>> {
    let tsg_home = home_d.join(".tmuxsg");

    create_dir!(!&tsg_home.is_dir(), &tsg_home);

    Ok(tsg_home)
}

pub fn home_dir() -> Result<PathBuf, Box<dyn std::error::Error>> {
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
    use super::*;

    #[test]
    fn check_script_path() -> Result<(), Box<dyn std::error::Error>> {
        let home_d = home_dir()?;
        let tsg_home = tmuxsg_home_dir(home_d)?;

        const S_NAME: &str = "new-session";
        let s_dir = session_dir(tsg_home, S_NAME)?;
        let script_path_expected = PathBuf::from(&format!("{}/{}.sh", &s_dir.display(), S_NAME));
        let script_path = script_path(s_dir, S_NAME)?;

        assert_eq!(script_path, script_path_expected);

        Ok(())
    }

    #[test]
    fn check_session_dir() -> Result<(), Box<dyn std::error::Error>> {
        let home_d = home_dir()?;
        let tsg_home = tmuxsg_home_dir(home_d)?;

        const S_NAME: &str = "new-session";
        let s_dir_expected = PathBuf::from(&format!("{}/{}", &tsg_home.display(), S_NAME));
        let s_dir = session_dir(tsg_home, S_NAME)?;

        assert_eq!(s_dir, s_dir_expected);

        Ok(())
    }

    #[test]
    fn check_tmuxsg_home_dir() -> Result<(), Box<dyn std::error::Error>> {
        let home_d = home_dir()?;
        let tsg_home_expected = PathBuf::from(&format!("{}/.tmuxsg", &home_d.display()));
        let tsg_home = tmuxsg_home_dir(home_d)?;

        assert_eq!(tsg_home, tsg_home_expected);

        Ok(())
    }

    #[test]
    fn found_home_dir() -> Result<(), Box<dyn std::error::Error>> {
        home_dir()?;

        Ok(())
    }
}
