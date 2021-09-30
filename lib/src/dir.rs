//! Directory helper methods

use crate::err::CustomResult;
use std::{fs, path::PathBuf};

pub fn create_dir(parent: PathBuf, child: String) -> CustomResult<PathBuf> {
    let new_dir = parent.join(&child);

    fs::create_dir_all(&new_dir)?;

    Ok(new_dir)
}
