mod options;

use options::get_opts;

fn main() {
    let opts = get_opts();
    println!("{:?}", opts);
}
