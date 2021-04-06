use structopt::StructOpt;

mod options;
mod sessions;

fn main() {
    let opts = options::Opts::from_args();
    println!("{:?}", opts);
}
