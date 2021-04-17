mod sessions;

use std::usize;

use structopt::StructOpt;

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

        /// Name of the session
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
        /// insert new window at next free index from -t
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
    pub fn generate_script(self) -> Result<Self, Box<dyn std::error::Error>> {
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
                let content = Self::session_script_contents(
                    command,
                    detach,
                    name_window,
                    session_name,
                    target_session,
                    x,
                    y,
                );

                Self::session_script(content)?;

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
                // Self::generate_window_script()

                Ok(self)
            }
        }
    }
}