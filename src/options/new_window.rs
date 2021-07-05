//! NewWindow subcommand helpers

use lib::{
    err::{DirectoryError, ScriptError},
    options::create_script,
    tmux_bool_option, tmux_option,
};
use std::{io::Write, path::PathBuf};

pub(in crate::options) fn create_window_script(
    content: (String, String, String),
    tmuxsg_home: PathBuf,
) -> Result<(), Box<dyn std::error::Error>> {
    let script_content = content.0;
    let session_name = content.1;
    let window_name = content.2;

    let isolated_session_name = if session_name.contains(":") {
        session_name.split(":").collect::<Vec<&str>>()[0].to_owned()
    } else {
        session_name
    };

    let session_dir = tmuxsg_home.join(&isolated_session_name);

    if !session_dir.is_dir() {
        let dir_err = DirectoryError(format!("{}", isolated_session_name));

        return Err(Box::new(dir_err));
    } else {
        let final_window_name = if window_name == "" {
            "new_window".to_owned()
        } else {
            window_name
        };

        let mut file = create_script(session_dir, &final_window_name)?;
        file.write_all(script_content.as_bytes())?;
    }

    Ok(())
}

pub(in crate::options) fn window_script_content(
    a: &bool,
    k: &bool,
    command: &String,
    d: &bool,
    n: &Option<String>,
    t: &Option<String>,
) -> Result<(String, String, String), Box<dyn std::error::Error>> {
    let error = "Window content related".to_owned();

    let mut content = "#!/bin/sh\n\n".to_owned();
    content.push_str(&format!("tmux new-window -c {}", command));

    tmux_option!(
        n t,
        content
    );

    tmux_bool_option!(
        a, content
        d, content
        k, content
    );

    let session_name = if let Some(target) = t {
        target
    } else {
        let content_err = ScriptError(error);

        return Err(Box::new(content_err));
    };

    let window_name = if let Some(target) = n {
        target
    } else {
        let content_err = ScriptError(error);

        return Err(Box::new(content_err));
    };

    Ok((content, session_name.to_owned(), window_name.to_owned()))
}
