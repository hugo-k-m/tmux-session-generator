use std::str::FromStr;

use structopt::StructOpt;

#[derive(Debug)]
pub struct WindowNames(Vec<String>);

impl FromStr for WindowNames {
    type Err = Box<dyn std::error::Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let new_s = s.chars().filter(|x| !x.is_whitespace()).collect::<String>();
        let split = new_s.split(",");
        let vec = split.map(|x| x.to_string()).collect();

        Ok(WindowNames(vec))
    }
}

#[derive(Debug, StructOpt)]
pub struct Opts {
    /// Name of the tmux session
    #[structopt(short, long, default_value = "new_session")]
    pub session_name: String,

    /// Directory to start the session in
    #[structopt(short, long, default_value = "~")]
    pub path: String,

    /// Number of windows in the session
    #[structopt(short, long, default_value = "3")]
    pub number_windows: usize,

    /// Name of each window
    #[structopt(short, long, default_value = "w_1,w_2,w_3")]
    pub window_names: WindowNames,

    /// Attach or detach the session after creation
    #[structopt(short, long)]
    pub attach: bool,
}

impl Opts {
    pub fn get_opts() -> Opts {
        Self::from_args()
    }
}
