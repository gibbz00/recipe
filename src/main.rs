use clap::Parser;
use recipe::cli::{Args, Execute};

fn main() {
    if let Err(err) = Args::parse().execute() {
        eprintln!("{err}")
    }
}
