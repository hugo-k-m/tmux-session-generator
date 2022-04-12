mod new_session;
mod new_window;
mod parser_associated_functions;
mod parser_helpers;

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
