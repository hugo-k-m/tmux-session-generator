use structopt::StructOpt;

#[derive(Debug, StructOpt)]
pub struct Opts {
    /// Name of the tmux session
    #[structopt(short, long)]
    pub session_name: Option<String>,

    /// Directory to start the session in
    #[structopt(short, long)]
    pub path: Option<String>,

    /// Number of windows in the session
    #[structopt(short, long)]
    pub number_windows: Option<usize>,

    /// Name of each window
    #[structopt(short, long)]
    pub window_names: Option<Vec<String>>,

    /// Attach or detach the session after creation
    #[structopt(short, long)]
    pub attach: Option<bool>,
}

impl Default for Opts {
    fn default() -> Self {
        let mut w_names = Vec::new();
        w_names.push("".to_owned());
        w_names.push("".to_owned());
        w_names.push("".to_owned());

        Self {
            session_name: Some("new_session".to_owned()),
            path: Some("~".to_owned()),
            number_windows: Some(3),
            window_names: Some(w_names),
            attach: Some(false),
        }
    }
}

impl Opts {
    pub fn get_opts() -> Opts {
        let options = Self::from_args();

        if let Opts {
            session_name: Some(_a),
            path: Some(_b),
            number_windows: Some(_c),
            window_names: Some(_d),
            attach: Some(_e),
        } = options
        {
            Self::from_args()
        } else {
            let default_opts = Self::default();

            let session_name = match options.session_name {
                Some(i) => Some(i),
                None => default_opts.session_name,
            };
            let path = match options.path {
                Some(i) => Some(i),
                None => default_opts.path,
            };
            let number_windows = match options.number_windows {
                Some(i) => Some(i),
                None => default_opts.number_windows,
            };
            let window_names = match options.window_names {
                Some(i) => Some(i),
                None => default_opts.window_names,
            };
            let attach = match options.attach {
                Some(i) => Some(i),
                None => default_opts.attach,
            };

            Opts {
                session_name: session_name,
                path: path,
                number_windows: number_windows,
                window_names: window_names,
                attach: attach,
            }
        }
    }
}
