use crate::algo::Algorithm;
use clap::Parser;

/// Struct defining possible arguments that can be passed into the command line.
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
pub struct Args {
    /// Set the end time for the simulations.
    #[clap(short = 't', long)]
    pub time: f32,

    /// Set number of repeats.
    #[clap(short, long, default_value_t = 1)]
    pub repeats: usize,

    /// Set the timestep in seconds.
    #[clap(short, long, default_value_t = 0.)]
    pub granularity: f32,

    /// Write to disk.
    #[clap(short, long)]
    pub write: bool,

    /// List of algorithms to run.
    #[clap(arg_enum)]
    pub algorithms: Vec<Algorithm>,
}
