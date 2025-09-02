pub mod base;
pub mod sir;
pub mod stats;

use base::{Parameters, SIRModel};
use clap::{Parser, ValueEnum};
use sir::{baseline, ixa};

#[derive(Copy, Clone, Debug, ValueEnum)]
pub enum ModelKind {
    Baseline,
    Ixa,
}

impl ModelKind {
    pub fn all() -> Vec<Self> {
        vec![Self::Baseline, Self::Ixa]
    }
    pub fn into_model(self, params: Parameters) -> Box<dyn SIRModel> {
        match self {
            ModelKind::Baseline => Box::new(baseline::Context::new(params)),
            ModelKind::Ixa => Box::new(ixa::Model::new(params)),
        }
    }
}

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    #[arg(short, long, value_enum)]
    pub model: Option<ModelKind>,

    #[arg(short, long, default_value_t = 200.0)]
    pub time: f64,

    #[arg(long)]
    pub stats: bool,
}

pub fn run_model(model: &mut Box<dyn SIRModel>) {
    println!(
        "Running model '{}' with params {:?}",
        model.id(),
        model.get_params()
    );
    model.run();
    println!(
        "Completed at time {:.2}, Infection incidence: {}",
        model.current_time(),
        model.get_stats().get_cum_incidence()
    );
}

pub fn run_from_args<F: FnOnce(&Args) -> Parameters>(build_params: F) {
    let args = Args::parse();

    let params = build_params(&args);

    // By default run all the models
    let mut model_kinds: Vec<ModelKind> = Vec::new();
    if let Some(kind) = args.model {
        model_kinds.push(kind)
    } else {
        model_kinds.extend(ModelKind::all());
    }

    for k in model_kinds {
        let mut model = k.into_model(params.clone());
        run_model(&mut model);
    }
}
