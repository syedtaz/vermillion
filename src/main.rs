mod cli;
use clap::Parser;

fn main() {
    let args = cli::Args::parse();
    println!("Count is {:?}", args.time);
}
