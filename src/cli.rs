use clap::{ArgEnum, Parser};
use std::fmt;

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ArgEnum)]
pub enum AlgoList {
    Direct,
}

impl fmt::Debug for AlgoList {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self {
            AlgoList::Direct => {
                write! {f, "direct"}
            }
        }
    }
}

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
pub struct Args {
    /// Set the end time for the simulations.
    #[clap(short, long)]
    pub time: f32,

    /// Set number of repeats.
    #[clap(short, long, default_value_t = 0)]
    pub repeats: usize,

    /// Set the timestep in seconds.
    #[clap(short, long, default_value_t = 0.)]
    pub granularity: f32,

    /// Write to disk.
    #[clap(short, long)]
    pub write: bool,

    /// List of algorithms to run.
    #[clap(arg_enum)]
    pub algorithms: Vec<AlgoList>,
}
