use std::error::Error;

use lib::home_dirs::home_directory;
use structopt::StructOpt;

mod options;

fn main() -> Result<(), Box<dyn Error>> {
    let opts = options::Opts::from_args();
    let home_d = home_directory()?;
    println!("{:?}", opts.invoke_subcommand(home_d));

    Ok(())
}
