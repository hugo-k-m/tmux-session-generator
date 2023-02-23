use super::new_session::create_session_script_content;
use super::new_window::{create_window_script, create_window_script_content};
use super::parser_helpers::handle_new_session_options;
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
                let content = create_session_script_content(
                    command,
                    detach,
                    name_window,
                    session_name,
                    target_session,
                    x,
                    y,
                );

                handle_new_session_options(content, session_name, target_session, tmuxsg_home)?;

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
                let content = create_window_script_content(
                    a,
                    kill,
                    command,
                    detach,
                    name_window,
                    target_window,
                )?;

                create_window_script(content, tmuxsg_home)?;

                Ok(self)
            }
        }
    }
}
