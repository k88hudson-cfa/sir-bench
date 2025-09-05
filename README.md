# SIR Bench

This crate compares implementations of a basic stochastic continuous-time SIR transmission model
with the same parameters.

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

## Model implementations

* `baseline`: A statically typed, simple implementation that stores the population
  in a `Vec<Person>`, and uses simple random sampling to select contacts.
* `ixa`: An implementation that uses ixa, using `sample_person` / querying; the model adds an index on `InfectionStatus`.
* `ixa-no-queries`: Same as `ixa` but avoids indexing or querying the population. The intention here is to isolate the effect of indexing/querying.

## Benchmarks

## sir

Small population with the following parameters:

```
r0: 1.5,
infectious_period: 3.0,
population: 1000,
initial_infections: 5,
seed: 1235,
max_time: args.time,
enable_stats: args.stats,
```

## large

Same parameters as the small test, but with a population of 100k.

## Results

The following results are from a CI run on  commit `7e704ae`. You can reproduce these results by running the [benchmarks](https://github.com/k88hudson-cfa/sir-bench/actions/workflows/benchmarks.yml) workflow with `large` as the first input, or running `just compare large` locally.

![Dispatch workflow UI with 'large' input](image.png)

Overall, it appears that the baseline implementation of a basic SIR model for a population of 100k with
a final attack rate of ~58% is around 7x faster than the ixa implementation, and
that using indexed queries for contact selection increases
the runtime by around 40x:

```
 Benchmark 1: ./target/release/large --model baseline

  Time (mean ± σ):      25.9 ms ±   1.7 ms    [User: 21.5 ms, System: 4.4 ms]
  Warning: The first benchmarking run for this command was significantly slower than the rest (27.9 ms). This could be caused by (filesystem) caches that were not filled until after the first run. You are already using the '--warmup' option which helps to fill these caches before the actual benchmark. You can either try to increase the warmup count further or re-run this benchmark on a quiet system in case it was a random outlier. Alternatively, consider using the '--prepare' option to clear the caches before each timing run.
  Range (min … max):    24.9 ms …  27.9 ms    3 runs

Benchmark 2: ./target/release/large --model ixa

  Time (mean ± σ):      7.378 s ±  0.022 s    [User: 7.360 s, System: 0.017 s]
  Warning: The first benchmarking run for this command was significantly slower than the rest (7.403 s). This could be caused by (filesystem) caches that were not filled until after the first run. You are already using the '--warmup' option which helps to fill these caches before the actual benchmark. You can either try to increase the warmup count further or re-run this benchmark on a quiet system in case it was a random outlier. Alternatively, consider using the '--prepare' option to clear the caches before each timing run.
  Range (min … max):    7.364 s …  7.403 s    3 runs

Benchmark 3: ./target/release/large --model ixa-no-queries
  Time (mean ± σ):     182.6 ms ±   1.0 ms    [User: 178.6 ms, System: 3.9 ms]
  Range (min … max):   181.6 ms … 183.6 ms    3 runs

Summary
  ./target/release/large --model baseline  ran
    7.05 ± 0.46 times faster than ./target/release/large --model ixa-no-queries
  284.79 ± 18.63 times faster than ./target/release/large --model ixa
```

These results are much harder to see for a smaller population (locally, I can't
reproduce this):

```
 Summary
  ./target/release/sir --model baseline  ran
    4.16 ± 0.09 times faster than ./target/release/sir --model ixa-no-queries
    6.08 ± 0.11 times faster than ./target/release/sir --model ixa
```
