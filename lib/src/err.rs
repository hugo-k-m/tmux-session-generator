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
    use crate::test::_produce_directory_error;

    #[test]
    fn produce_directory_error() {
        _produce_directory_error();
    }
}
