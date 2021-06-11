mod new_session;
mod new_window;

use new_session::{create_session_script, session_script_content};
use new_window::window_script_content;
use std::{path::PathBuf, usize};
use structopt::StructOpt;

use new_window::create_window_script;

#[derive(Debug, StructOpt)]
pub enum Opts {
    NewSession {
        /// Specify working directory for the session.
        #[structopt(short, long, default_value = "~")]
        command: String,

        /// Don't attach new session to current terminal
        #[structopt(short, long)]
        detach: bool,

        /// Initial window name
        #[structopt(short, long)]
        name_window: Option<String>,

        /// Name the session
        #[structopt(short, long, default_value = "new_session")]
        session_name: String,

        /// Specify target session
        #[structopt(short, long)]
        target_session: Option<String>,

        /// Specify width
        #[structopt(short, long)]
        x: Option<usize>,

        /// Specify height
        #[structopt(short, long)]
        y: Option<usize>,
    },

    NewWindow {
        /// Insert new window at next free index from -t
        #[structopt(short, long)]
        a: bool,

        /// Destroy it if the specified window exists
        #[structopt(short, long)]
        kill: bool,

        /// Don't make the new window become the active one
        #[structopt(short, long, default_value = "~")]
        command: String,

        /// Don't make the new window become the active one
        #[structopt(short, long)]
        detach: bool,

        /// Specify a window name
        #[structopt(short, long)]
        name_window: Option<String>,

        /// Specify target window
        #[structopt(short, long)]
        target_window: Option<String>,
    },
}

impl Opts {
    pub fn invoke_subcommand(
        self,
        tmuxsg_home: PathBuf,
    ) -> Result<Self, Box<dyn std::error::Error>> {
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

                create_session_script(content, session_name, tmuxsg_home)?;

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
                    window_script_content(a, kill, command, detach, name_window, target_window);

                create_window_script(content, tmuxsg_home)?;

                Ok(self)
            }
        }
    }
}
