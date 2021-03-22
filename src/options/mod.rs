use structopt::{clap, StructOpt};

#[derive(Debug, StructOpt)]
pub struct Opts {
    /// Name of the tmux session
    #[structopt(short, long)]
    session_name: String,

    /// Directory to start the session in
    #[structopt(short, long)]
    path: String,

    /// Number of windows in the session
    #[structopt(short, long)]
    number_windows: u32,

    /// Name of each window
    #[structopt(short, long)]
    window_names: Option<Vec<String>>,

    /// Attach or detach the session after creation
    #[structopt(short, long)]
    attach: bool,
}

pub fn get_opts() -> Result<Opts, clap::Error> {
    Ok(Opts::from_args())
}
