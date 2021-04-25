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
