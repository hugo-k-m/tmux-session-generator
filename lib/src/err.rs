//! Custom errors

use std::{error::Error, fmt::Display};

#[derive(Debug, Clone)]
pub struct DirectoryError(pub String);

impl Display for DirectoryError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} directory error", &self.0)
    }
}

impl Error for DirectoryError {}

#[macro_export]
macro_rules! produce_directory_error {
    ($err_details:expr) => {
        let dir_err = DirectoryError($err_details);

        return Err(Box::new(dir_err));
    };
}

#[derive(Debug, Clone)]
pub struct ScriptError(pub String);

impl Display for ScriptError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} script error", &self.0)
    }
}

impl Error for ScriptError {}

#[macro_export]
macro_rules! produce_script_error {
    ($err_details:expr) => {
        let script_err = ScriptError($err_details);

        return Err(Box::new(script_err));
    };
}

#[cfg(test)]
mod tests {
    use crate::err::{DirectoryError, ScriptError};

    #[test]
    fn directory_error_displays_correctly() {
        let actual_error_disp = format!("{}", DirectoryError("Test".to_owned()));
        let expected_error_disp = format!("Test directory error");

        assert_eq!(actual_error_disp, expected_error_disp);
    }

    #[test]
    fn script_error_displays_correctly() {
        let actual_error_disp = format!("{}", ScriptError("Test".to_owned()));
        let expected_error_disp = format!("Test script error");

        assert_eq!(actual_error_disp, expected_error_disp);
    }
}
