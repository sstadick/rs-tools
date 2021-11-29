use std::path::PathBuf;

use clap::Parser;

#[derive(Debug, Parser)]
struct Opts {
    cm_file: PathBuf,
}
fn main() {
    let opts = Opts::parse();
    eprintln!("{:?}", opts);
}
