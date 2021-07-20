//! Shared utilities for the options module

use crate::{err::ScriptError, produce_script_error};
use std::{
    fs::{self, File},
    path::PathBuf,
};

/// Creates the script and opens it in write-only mode.
pub fn create_script(
    session_dir: PathBuf,
    script_name: &str,
) -> Result<File, Box<dyn std::error::Error>> {
    let script_path = session_dir.join(&format!("{}.sh", script_name));

    if script_path.is_file() {
        produce_script_error!(format!("{}", script_name));
    }

    let file = fs::File::create(&script_path)?;

    Ok(file)
}

#[macro_export]
macro_rules! tmux_option {
    ( $( $y:expr ) +, $content:ident ) => {
        $(
            if let Some(tmux_opt) = $y {
                $content.push_str(&format!("-{} {}", stringify!($y), tmux_opt));
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
    use crate::{options::create_script, test::SessionTestObjects};
    use std::{fs, path::PathBuf};

    /// Test script creation process
    #[test]
    fn create_script_success() -> Result<(), Box<dyn std::error::Error>> {
        const SESSION_NAME: &str = "new_session";

        let tsg_test = SessionTestObjects::setup()?;
        let tsg_dir = tsg_test.test_tmuxsg_path;

        let session_dir = PathBuf::from(&format!("{}/{}", tsg_dir.display(), SESSION_NAME));
        fs::create_dir(&session_dir)?;

        let script_path_expected =
            PathBuf::from(&format!("{}/{}.sh", session_dir.display(), SESSION_NAME));

        create_script(session_dir, SESSION_NAME)?;

        assert!(script_path_expected.is_file());

        Ok(())
    }
}
