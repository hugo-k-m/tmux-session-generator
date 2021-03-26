mod options;
mod sessions;

fn main() {
    let opts = options::Opts::get_opts();
    println!("{:?}", opts);
}
