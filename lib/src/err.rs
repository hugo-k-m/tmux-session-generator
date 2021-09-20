//! Custom errors

use anyhow::Result as AnyhowResult;
use std::{error::Error as StdError, fmt::Display};

pub type CustomResult<T> = AnyhowResult<T>;

#[derive(Debug, Clone)]
pub struct DirectoryError(pub String);

impl Display for DirectoryError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Directory error: {}", &self.0)
    }
}

impl StdError for DirectoryError {}

#[macro_export]
macro_rules! produce_directory_error {
    ($err_details:expr) => {
        let dir_err = DirectoryError($err_details);

        return Err(dir_err).map_err(anyhow::Error::msg);
    };
}

#[derive(Debug, Clone)]
pub struct ScriptError(pub String);

impl Display for ScriptError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Script error: {}", &self.0)
    }
}

impl StdError for ScriptError {}

#[macro_export]
macro_rules! produce_script_error {
    ($err_details:expr) => {
        let script_err = ScriptError($err_details);

        return Err(script_err).map_err(anyhow::Error::msg);
    };
}

#[cfg(test)]
mod tests {
    use crate::err::{DirectoryError, ScriptError};

    #[test]
    fn directory_error_displays_correctly() {
        let actual_error_disp = format!("{}", DirectoryError("Test".to_owned()));
        let expected_error_disp = format!("Directory error: Test");

        assert_eq!(actual_error_disp, expected_error_disp);
    }

    #[test]
    fn script_error_displays_correctly() {
        let actual_error_disp = format!("{}", ScriptError("Test".to_owned()));
        let expected_error_disp = format!("Script error: Test");

        assert_eq!(actual_error_disp, expected_error_disp);
    }
}
