//! Custom errors

use std::{error::Error, fmt::Display};

#[derive(Debug, Clone)]
pub struct DirectoryError<'a>(pub &'a str);

impl<'a> Display for DirectoryError<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} directory error", &self.0)
    }
}

impl<'a> Error for DirectoryError<'a> {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn directory_error_displays_correctly() {
        let actual_error_disp = format!("{}", DirectoryError("Test"));
        let expected_error_disp = format!("Test directory error");

        assert_eq!(actual_error_disp, expected_error_disp);
    }
}
