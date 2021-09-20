//! Directory helper methods

use crate::{
    err::{CustomResult, DirectoryError},
    produce_directory_error,
};
use std::{fs, path::PathBuf};

pub fn create_dir(parent: PathBuf, child: String) -> CustomResult<PathBuf> {
    let new_dir = parent.join(&child);

    if !new_dir.is_dir() {
        fs::create_dir(&new_dir)?;
    } else if child != ".tmuxsg" {
        produce_directory_error!(child + " session already exists!");
    }

    Ok(new_dir)
}
