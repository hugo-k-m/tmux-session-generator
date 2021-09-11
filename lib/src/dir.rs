//! Directory helper methods

use std::{fs, path::PathBuf};

pub fn create_dir(parent: PathBuf, child: String) -> Result<PathBuf, Box<dyn std::error::Error>> {
    let new_dir = parent.join(child);

    if !new_dir.is_dir() {
        fs::create_dir(&new_dir)?;
    }

    Ok(new_dir)
}
