use sir_bench::base::Parameters;
use sir_bench::run_from_args;

// Runs a simple SIR model with 100k population
fn main() {
    run_from_args(|args| Parameters {
        r0: 1.5,
        infectious_period: 3.0,
        population: 1000,
        initial_infections: 5,
        seed: 12345,
        max_time: args.time,
        enable_stats: args.stats,
        disable_queries: args.disable_queries,
    });
}
