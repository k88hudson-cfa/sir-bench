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

```
just compare large
cargo build --release
    Finished `release` profile [optimized] target(s) in 0.14s
hyperfine --warmup 1 --runs 3 './target/release/large --model baseline ' './target/release/large --model ixa '
Benchmark 1: ./target/release/large --model baseline
  Time (mean ± σ):      21.2 ms ±   2.1 ms    [User: 18.8 ms, System: 1.9 ms]
  Range (min … max):    18.8 ms …  22.5 ms    3 runs

Benchmark 2: ./target/release/large --model ixa
  Time (mean ± σ):      6.013 s ±  0.180 s    [User: 5.786 s, System: 0.041 s]
  Range (min … max):    5.806 s …  6.125 s    3 runs

  Warning: Statistical outliers were detected. Consider re-running this benchmark on a quiet system without any interferences from other programs. It might help to use the '--warmup' or '--prepare' options.

Summary
  ./target/release/large --model baseline  ran
  284.18 ± 29.08 times faster than ./target/release/large --model ixa
```




