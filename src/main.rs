use structopt::StructOpt;

#[derive(Debug, StructOpt)]
struct Opts {
    /// Name of the tmux session
    #[structopt(short = "sn", long = "session-name")]
    session_name: String,

    /// Directory to start the session in
    #[structopt(short = "p", long = "session-path")]
    path: String,

    /// Number of windows in the session
    #[structopt(short = "numw", long = "number-windows")]
    number_windows: u32,

    /// Name of each window
    #[structopt(short = "wname", long = "window-names")]
    window_names: Option<Vec<String>>,

    /// Attach or detach the session after creation
    #[structopt(short, long)]
    attach: bool,
}

fn main() {
    let opts = Opts::from_args();
    println!("{:?}", opts);
}
