use std::env;

use okari::builder::Builder;

fn main() {
    Builder::run(env::args().nth(1).expect("USAGE: okari <src>"))
}
