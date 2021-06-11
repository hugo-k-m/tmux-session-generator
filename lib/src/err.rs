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

#[cfg(test)]
mod tests {
    use crate::err::DirectoryError;

    #[test]
    fn directory_error_displays_correctly() {
        let actual_error_disp = format!("{}", DirectoryError("Test".to_owned()));
        let expected_error_disp = format!("Test directory error");

        assert_eq!(actual_error_disp, expected_error_disp);
    }
}
