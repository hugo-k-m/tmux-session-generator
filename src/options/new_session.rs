//! Helper functions for the new-session subcommand

use lib::{
    self,
    path::{home_dir, script_path, session_dir, tmuxsg_home_dir},
    tmux_option,
};
use std::fs;
use std::io::Write;

pub(in crate::options) fn create_session_script(
    content: String,
    s_name: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let home_d = home_dir()?;
    let tmuxsg_home = tmuxsg_home_dir(home_d)?;
    let s_dir = session_dir(tmuxsg_home, s_name)?;
    let script_path = script_path(s_dir, s_name)?;

    let mut file = fs::File::create(script_path)?;
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
    tmux_option!(name_w, n
        target_s, t
        width, x
        height, y);

    const SESSION_VAR: &str = "session";
    const PATH_VAR: &str = "session_path";

    let mut content = "#!/bin/sh\n\n".to_owned();

    content.push_str(&format!("{}=\"{}\"\n", SESSION_VAR, session_name));
    content.push_str(&format!("{}={}\n", PATH_VAR, command));

    content.push_str(&format!(
        "tmux new-session -d -s ${} -c ${} {} {} {} {}\n",
        SESSION_VAR, PATH_VAR, name_w, target_s, width, height
    ));

    if detach.to_owned() {
        return content;
    } else {
        content.push_str("\n# Attach\n");
        content.push_str(&format!("tmux attach -t ${}", SESSION_VAR));
    };

    content
}
