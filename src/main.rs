mod algo;
mod cli;
mod dataframes;
mod examples;
mod system;

use algo::dispatch;
use clap::Parser;
use std::env;

fn main() {
    env::set_var("RUST_BACKTRACE", "1");
    let args = cli::Args::parse();

    // MRNA Degradation
    // let initial = vec![100.];
    // let network = examples::formation::Protein {};

    // Repressilator
    let initial = vec![10., 10., 10., 10., 10., 10., 0., 0., 0.];
    let network = examples::repressilator::Repressilator {};
    dispatch(args, network, initial).unwrap();
}
