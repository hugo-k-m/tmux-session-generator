//! Shared utilities for the options module.

use crate::err::CustomResult;
use std::{
    fs::{self, File},
    path::PathBuf,
};

/// Creates the script and opens it in write-only mode.
pub fn create_script(session_dir: PathBuf, script_name: String) -> CustomResult<File> {
    let script_path = session_dir.join(&format!("{}.sh", script_name));
    let file = fs::File::create(&script_path)?;

    Ok(file)
}

#[macro_export]
macro_rules! tmux_option {
    ( $( $y:expr ) +, $content:ident ) => {
        $(
            if let Some(tmux_opt) = $y {
                $content.push_str(&format!(" -{} {}", stringify!($y), tmux_opt));
            };
        ) +
    };
}

#[macro_export]
macro_rules! tmux_bool_option {
    ( $( $x:expr, $y:ident ) + ) => {
        $(
            if $x.to_owned() {
                $y.push_str(&format!(" -{}", stringify!($x)));
            }
        ) +
    };
}

#[cfg(test)]
mod tests {
    use crate::{
        err::CustomResult,
        mocks::{TestObject, TestTmuxHomeDir},
        options::create_script,
    };
    use std::{fs, path::PathBuf};

    /// Test script creation process
    #[test]
    fn create_script_success() -> CustomResult<()> {
        let session_name = "new_session".to_owned();

        let tsg_test = TestTmuxHomeDir::setup(None)?;
        let tsg_dir = tsg_test.test_tmuxsg_path;

        let session_dir = PathBuf::from(&format!("{}/{}", tsg_dir.display(), session_name));
        fs::create_dir(&session_dir)?;

        let script_path_expected =
            PathBuf::from(&format!("{}/{}.sh", session_dir.display(), session_name));

        create_script(session_dir, session_name)?;

        assert!(script_path_expected.is_file());

        Ok(())
    }
}
