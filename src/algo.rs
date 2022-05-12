mod direct;
mod fr;
mod nrm;
mod sdirect;

use clap::ArgEnum;
use std::fmt;

/// Defines the list of algorithms that can be passed into the command line interface.
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ArgEnum)]
pub enum Algorithm {
    Direct,
}

/// Implements debug trait for Algolist.
impl fmt::Debug for Algorithm {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self {
            Algorithm::Direct => {
                write! {f, "direct"}
            }
        }
    }
}

/// Requires every algorithm to implement a simulate function.
trait Simulate {
    fn simulate(&self, t_end: f32) -> Result<(), usize>;
}

impl Simulate for Algorithm {
    fn simulate(&self, t_end: f32) -> Result<(), usize> {
        match &self {
            Algorithm::Direct => direct::simulate(t_end),
        }
    }
}
