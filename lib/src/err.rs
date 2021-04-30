//! Custom errors

use std::{error::Error, fmt::Display};

#[derive(Debug, Clone)]
pub struct DirectoryError;

impl Display for DirectoryError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "directory not found")
    }
}

impl Error for DirectoryError {}

#[cfg(test)]
mod tests {
    use crate::test::_produce_directory_error;

    #[test]
    fn produce_directory_error() {
        _produce_directory_error();
    }
}
