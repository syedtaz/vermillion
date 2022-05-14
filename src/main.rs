mod algo;
mod cli;
mod dataframes;
mod examples;
mod system;

use algo::Simulate;
use clap::Parser;
use dataframes::write_csv;
use system::System;

fn main() {
    let args = cli::Args::parse();
    let initial = vec![100.];

    let network = examples::formation::Protein {};

    for alg in args.algorithms {
        for idx in [0..args.repeats] {
            let results = alg
                .simulate(args.time, &network, initial.clone(), args.granularity)
                .unwrap();
            if args.write {
                let fname = format!(
                    "/Users/tazmilur/Projects/vermillion/data/{}_{:?}_{:?}",
                    network.name(),
                    alg,
                    idx
                );
                write_csv(results, &fname, network.size());
            }
        }
    }
}
