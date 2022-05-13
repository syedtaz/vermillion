mod direct;
mod fr;
mod nrm;
mod sdirect;

use crate::system::System;
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
pub trait Simulate {
    fn simulate(
        &self,
        t_end: f32,
        network: &impl System,
        initial: Vec<f32>,
    ) -> Result<Vec<Vec<f32>>, usize>;
}

impl Simulate for Algorithm {
    fn simulate(
        &self,
        t_end: f32,
        network: &impl System,
        initial: Vec<f32>,
    ) -> Result<Vec<Vec<f32>>, usize> {
        match &self {
            Algorithm::Direct => direct::simulate(t_end, network, initial),
        }
    }
}
