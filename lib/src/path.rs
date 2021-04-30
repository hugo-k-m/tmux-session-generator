//! Functions used to create and/or fetch session paths

use std::{fs, path::PathBuf};

pub fn session_script(s_dir: PathBuf, s_name: &str) -> Result<PathBuf, Box<dyn std::error::Error>> {
    let script_path = s_dir.join(&format!("{}.sh", s_name));

    fs::File::create(&script_path)?;

    Ok(script_path)
}

pub fn session_dir(tsg_home: PathBuf, s_name: &str) -> Result<PathBuf, Box<dyn std::error::Error>> {
    let s_dir = tsg_home.join(s_name);

    fs::create_dir(&s_dir)?;

    Ok(s_dir)
}

#[cfg(test)]
mod tests {
    use crate::{home_dirs::tmuxsg_home_dir, test::CreationTest};

    use super::*;

    #[test]
    fn create_session_script_success() -> Result<(), Box<dyn std::error::Error>> {
        let tsg_test = CreationTest::setup()?;
        let home_d = tsg_test.path;
        let tsg_home = tmuxsg_home_dir(home_d)?;

        const S_NAME: &str = "new_session";
        let s_dir = session_dir(tsg_home, S_NAME)?;
        let script_path_expected = PathBuf::from(&format!("{}/{}.sh", &s_dir.display(), S_NAME));
        session_script(s_dir, S_NAME)?;

        assert_eq!(script_path_expected.is_file(), true);

        Ok(())
    }

    #[test]
    fn create_session_directory_success() -> Result<(), Box<dyn std::error::Error>> {
        let tsg_test = CreationTest::setup()?;
        let home_d = tsg_test.path;
        let tsg_home = tmuxsg_home_dir(home_d)?;

        const S_NAME: &str = "new_session";
        let s_dir_expected = PathBuf::from(&format!("{}/{}", &tsg_home.display(), S_NAME));
        session_dir(tsg_home, "new_session")?;

        assert_eq!(s_dir_expected.is_dir(), true);

        Ok(())
    }
}
