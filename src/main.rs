use structopt::StructOpt;

mod options;

fn main() {
    let opts = options::Opts::from_args();
    println!("{:?}", opts.generate_script());
}
