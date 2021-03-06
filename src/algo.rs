mod direct;
mod direct_jump;
mod fr;
mod nrm;
mod sdirect;

use crate::cli;
use crate::dataframes::*;
use crate::system::System;
use clap::ArgEnum;
use ndarray;
use std::env;
use std::fmt;
use std::path::Path;

/// Defines the list of algorithms that can be passed into the command line interface.
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ArgEnum)]
pub enum Algorithm {
    Direct,
    DirectJump,
}

/// Defines the return type of each simulation algorithm. The algorithms can returns nested vectors if the granularity is not specified;
/// otherwise, it returns an ndarray.
pub enum Output {
    Vec2D(Vec<Vec<f32>>),
    Array2D(ndarray::Array<f32, ndarray::Dim<[usize; 2]>>),
}

/// Implements debug trait for Algolist.
impl fmt::Debug for Algorithm {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self {
            Algorithm::Direct => {
                write! {f, "Direct"}
            }
            Algorithm::DirectJump => {
                write! {f, "DirectJump"}
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
        granularity: Option<f32>,
    ) -> Result<Output, usize>;
}

impl Simulate for Algorithm {
    fn simulate(
        &self,
        t_end: f32,
        network: &impl System,
        initial: Vec<f32>,
        granularity: Option<f32>,
    ) -> Result<Output, usize> {
        match &self {
            Algorithm::Direct => direct::simulate(t_end, network, initial),
            Algorithm::DirectJump => {
                direct_jump::simulate(t_end, network, initial, granularity.unwrap())
            }
        }
    }
}

pub fn dispatch(args: cli::Args, system: impl System, initial: Vec<f32>) -> Result<(), ()> {
    match args.average {
        true => {
            //let length = (args.time / args.granularity.unwrap()).ceil() as usize;
            for alg in args.algorithms {
                let mut jumps: Vec<f32> = Vec::new();
                let mut queue_time = 0.;
                let jump = args.granularity.unwrap();

                while queue_time <= args.time {
                    jumps.push(queue_time);
                    queue_time += jump;
                }
                let length = jumps.len();

                let mut farray = ndarray::Array::<f32, _>::zeros((length, initial.len() + 1));

                for _ in 0..args.repeats {
                    let results = alg
                        .simulate(args.time, &system, initial.clone(), args.granularity)
                        .unwrap();
                    if let Output::Array2D(x) = results {
                        farray += &x;
                    }
                }

                farray = farray / args.repeats as f32;
                let mut temp = farray.column_mut(0);
                for (i, item) in jumps.iter().enumerate() {
                    temp[i] = *item;
                }

                let mut curdir = env::current_dir().unwrap();
                curdir.pop();
                curdir.pop();
                let temp = format!("data/{}_{:?}_avg", system.name(), alg);
                let path = Path::new(&temp);
                let fname = curdir.join(path);
                write_csv_array(farray, fname.to_str().unwrap(), initial.len()).unwrap();
            }
        }
        false => {
            for alg in args.algorithms {
                for idx in 0..args.repeats {
                    let results = alg
                        .simulate(args.time, &system, initial.clone(), args.granularity)
                        .unwrap();
                    if args.write {
                        let mut curdir = env::current_dir().unwrap();
                        curdir.pop();
                        curdir.pop();
                        let temp = format!("data/{}_{:?}_{:?}", system.name(), alg, idx);
                        let path = Path::new(&temp);
                        let fname = curdir.join(path);
                        if let Output::Vec2D(x) = results {
                            write_csv(x, fname.to_str().unwrap(), initial.len());
                        }
                    }
                }
            }
        }
    }

    Ok(())
}
