//! Shared utilities for the options module (necessary?)

use crate::err::ScriptError;

use std::{
    fs::{self, File},
    path::PathBuf,
};

// TODO: handle error with custom (script?) error
/// Creates the script and opens it in write-only mode.
pub fn create_script(
    session_dir: PathBuf,
    script_name: &str,
) -> Result<File, Box<dyn std::error::Error>> {
    let script_path = session_dir.join(&format!("{}.sh", script_name));

    if script_path.is_file() {
        let dir_err = ScriptError(format!("{}", script_name));

        return Err(Box::new(dir_err));
    }

    let file = fs::File::create(&script_path)?;

    Ok(file)
}

#[macro_export]
macro_rules! tmux_option {
    ( $( $x:ident, $y:expr ) + ) => {
        $(
            let $x = if let Some(tmux_opt) = $y {
                format!("-{} {}", stringify!($y), tmux_opt)
            } else {
                "".to_string()
            };
        ) +
    };
}

#[macro_export]
macro_rules! tmux_bool_option {
    ( $( $x:expr, $y:ident ) + ) => {
        $(
            if $x.to_owned() {
                $y.push_str(&format!(" -{}", stringify!($x)));
            }
        ) +
    };
}
