<div align="center">
  <a href="https://github.com/syedtaz/vermillion">
    <img src="./logo.png" style = "width: 10vw; min-width: 100px;" alt="vermillion logo"/>
  </a>
  <br />
  <strong>Vermillion</strong>
</div>

<div align="center">

---

</div>

## Installation

- Download and install Rust from [here](https://www.rust-lang.org/tools/install).
- Install git if you don't have it.
- Clone the repository by running `git clone https://github.com/syedtaz/vermillion.git`
- `cd` into the repository
- Run `cargo build`

The first time you run `build` it will take some time since the package manager will download and build
packages for numpy-style arrays, data wrangling, random numbers, etc. However, subsequent builds should
not take long.

## Examples

#### MRNA Degradation

The package comes with two systems preloaded. For example, if you want to simulate a simple MRNA degradation
model with a degradation rate of `1.0`.

- Edit the `main.rs` file and uncomment the `mrna degradation` block.
- Open a terminal and `cd` to the main directory.
- Run `cargo build` to compile.
- `cd` to `/target/debug/`
- If you want to run a single simulation for `10.0` seconds and save every timestep
  - Run `./vermillion -t 10. -w direct`
- Or if you want an average of `5` different repeats of the simulations for `10.` seconds with a granularity of `0.5` seconds:
  - Run `./vermillion -a -g 0.5 -r 5 -t 10. -w direct-jump`
- The results should be saved in a `/data/` folder.
- Go back to the main directory and run `python3 plotter.py`. You will need to have `pandas` and `matplotlib` installed.
- A `plot.png` file will be created in your data directory.

#### Repressilator

If you want to simulate the repressilator system:

- Edit the `main.rs` file and uncomment the `repressilator` block.
- Open a terminal and `cd` to the main directory.
- Run `cargo build` to compile.
- `cd` to `/target/debug/`
- If you want to run a single simulation for `1000.0` seconds and save every timestep
  - Run `./vermillion -t 10. -w direct`
  - Note that the resulting log file can be very large, so this method is not recommended.
- If you want an average of `5` different repeats of the simulations for `1000.` seconds with a granularity of `60` seconds:
  - Run `./vermillion -a -g 60. -r 5 -t 1000. -w direct-jump`
- The results should be saved in a `/data/` folder.
- Go back to the main directory and run `python3 plotter.py`. You will need to have `pandas` and `matplotlib` installed.
- A `plot.png` file will be created in your data directory.

## Usage

Vermillion can simulate a system in two ways: it can run a complete simulation of every time jump, or you
can specify a granularity `g` and it will only save the results at every `g` timestep. In the `main.rs`
file, you will have to specify a system and its initial conditions and then from the command line
pass in arguments that will define how long you want to simulate, the granularity, whether to save the results
or not and so on.

```
vermillion 0.1.0
Tazmilur Saad
Fastest stochastic simulator in the west.

USAGE:
    vermillion [OPTIONS] --time <TIME> [ALGORITHMS]...

ARGS:
    <ALGORITHMS>...    List of algorithms to run [possible values: direct, direct-jump]

OPTIONS:
    -g, --granularity <GRANULARITY>    Set the timestep in seconds
    -h, --help                         Print help information
    -r, --repeats <REPEATS>            Set number of repeats [default: 1]
    -t, --time <TIME>                  Set the end time for the simulations
    -V, --version                      Print version information
    -w, --write                        Write to disk
```
