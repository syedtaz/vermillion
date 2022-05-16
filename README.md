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

- Download and install Rust from (https://www.rust-lang.org/tools/install)[here].
- Clone the repository.
- cd into the repository
- `cargo build`

## Usage

##

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
