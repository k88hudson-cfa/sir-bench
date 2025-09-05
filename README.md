# SIR Bench

This crate compares implementations of an SIR model at for parameter
configurations.

## Methodology

We use `hyperfine` for benchmarking, which is a platform-agnostic benchmarking
tool. The baseline implementation is a simple stochastic SIR model using minimal
dependencies.

So far, I've only run these on my local machine.

## Running the tests

Install [`just`](https://just.systems/man/en/packages.html) to use the the commands
in `justfile`; you can also install hyperfine (`cargo install hyperfine`), build
the binaries, and run each test yourself. take a look at `justfile` for how to
run hyperfine directly.

```sh
# Installs hyperfine
just setup

# Run the sir benchmark, which has a population of 1000
just compare sir

# Run the large benchmark only up to t=50
just compare large -t 50

```

## Benchmarks

## sir

Small population with the following parameters:

```
r0: 1.5,
infectious_period: 3.0,
population: 1000,
initial_infections: 5,
seed: 42,
max_time: args.time,
enable_stats: args.stats,
```

## large

Same parameters as the small test, but with a population of 100k.

