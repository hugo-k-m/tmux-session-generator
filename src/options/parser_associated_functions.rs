use super::new_session::{create_session_script, session_script_content};
use super::new_window::{create_window_script, window_script_content};
use super::Opts;
use lib::err::CustomResult;
use std::path::PathBuf;

impl Opts {
    pub fn invoke_subcommand(self, tmuxsg_home: PathBuf) -> CustomResult<Self> {
        match &self {
            Opts::NewSession {
                command,
                detach,
                name_window,
                session_name,
                target_session,
                x,
                y,
            } => {
                let content = session_script_content(
                    command,
                    detach,
                    name_window,
                    session_name,
                    target_session,
                    x,
                    y,
                );

                let group_option = false;

                create_session_script(content, session_name, tmuxsg_home, group_option)?;

                Ok(self)
            }

            Opts::NewWindow {
                a,
                kill,
                command,
                detach,
                name_window,
                target_window,
            } => {
                let content =
                    window_script_content(a, kill, command, detach, name_window, target_window)?;

                create_window_script(content, tmuxsg_home)?;

                Ok(self)
            }
        }
    }
}
