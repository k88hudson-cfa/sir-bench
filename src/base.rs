use crate::stats::ModelStats;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Parameters {
    pub r0: f64,
    pub infectious_period: f64,
    pub population: usize,
    pub initial_infections: usize,
    pub seed: u64,
    pub max_time: f64,
    pub enable_stats: bool,
}

impl Default for Parameters {
    fn default() -> Self {
        Parameters {
            r0: 1.5,
            infectious_period: 3.0,
            population: 1000,
            initial_infections: 5,
            seed: 42,
            max_time: 100.0,
            enable_stats: false,
        }
    }
}

pub trait SIRModel {
    fn id(&self) -> &'static str;
    fn current_time(&self) -> f64;
    fn run(&mut self);
    fn get_stats(&self) -> &ModelStats;
    fn get_params(&self) -> &Parameters;
}
