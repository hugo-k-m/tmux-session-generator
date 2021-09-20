mod home_dirs;
mod options;

use crate::home_dirs::{home_directory, tmuxsg_home_dir};
use directories::BaseDirs;
use lib::err::CustomResult;
use structopt::StructOpt;

fn main() -> CustomResult<()> {
    let opts = options::Opts::from_args();
    let home_d = home_directory(BaseDirs::new())?;
    let tmuxsg_home = tmuxsg_home_dir(home_d)?;

    println!("{:?}", opts.invoke_subcommand(tmuxsg_home));

    Ok(())
}
