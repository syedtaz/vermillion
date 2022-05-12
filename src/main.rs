mod algo;
mod cli;
mod examples;
mod system;

use clap::Parser;

fn main() {
    let args = cli::Args::parse();
    println!("{:?}", args.time);
}
