use std::{env, io};

use okari::builder::Builder;

fn main() -> io::Result<()> {
    Builder::run(env::args().nth(1).expect("USAGE: okari <src>"))
}
