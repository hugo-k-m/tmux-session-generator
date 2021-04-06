use structopt::StructOpt;

#[derive(Debug, StructOpt)]
pub enum Opts {
    NewSession {
        /// Specify working directory for the session.
        #[structopt(short = "c", long, default_value = "~")]
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
        #[structopt(short)]
        x: Option<usize>,

        /// Specify height
        #[structopt(short)]
        y: Option<usize>,
    },
}
