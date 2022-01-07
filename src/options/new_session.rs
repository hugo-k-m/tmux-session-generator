//! NewSession subcommand helpers.

use anyhow::Context;
use lib::{dir::create_dir, err::CustomResult, options::create_script, tmux_option};
use std::{io::Write, path::PathBuf};

pub(in crate::options) fn create_session_script(
    content: String,
    s_name: &str,
    tmuxsg_home: PathBuf,
    group_option: bool,
) -> CustomResult<()> {
    let s_dir = create_dir(tmuxsg_home, s_name.to_owned())?;

    set_session_group_option(&s_dir, group_option)?;

    let mut file =
        create_script(s_dir, s_name).with_context(|| format!("could not create session script"))?;
    file.write_all(content.as_bytes())?;

    Ok(())
}

pub(in crate::options) fn session_script_content(
    command: &String,
    detach: &bool,
    n: &Option<String>,
    session_name: &String,
    t: &Option<String>,
    x: &Option<usize>,
    y: &Option<usize>,
) -> String {
    const SESSION_VAR: &str = "session";
    const PATH_VAR: &str = "session_path";

    let mut content = "#!/bin/sh\n\n".to_owned();

    content.push_str(&format!("{}=\"{}\"\n", SESSION_VAR, session_name));
    content.push_str(&format!("{}={}\n", PATH_VAR, command));

    content.push_str(&format!(
        "tmux new-session -d -s ${} -c ${}",
        SESSION_VAR, PATH_VAR
    ));

    tmux_option!(
        n t x y,
        content
    );

    if detach.to_owned() {
        return content;
    } else {
        content.push_str("\n\n# Attach\n");
        content.push_str(&format!("tmux attach -t ${}", SESSION_VAR));
    };

    content
}

fn set_session_group_option(session_dir: &PathBuf, group_option: bool) -> CustomResult<()> {
    let session_group_option = if group_option {
        "is_session_group"
    } else {
        "is_not_session_group"
    };

    let mut file = create_script(session_dir.to_owned(), "__session_group_option")?;
    file.write_all(session_group_option.as_bytes())?;

    Ok(())
}

#[cfg(test)]
mod tests;
