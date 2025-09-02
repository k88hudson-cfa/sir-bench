use crate::{
    base::{Parameters, SIRModel},
    stats::ModelStats,
};
use ixa::{PersonId, prelude::*};
use serde::Serialize;
use statrs::distribution::Exp;

#[derive(Clone, Copy, PartialEq, Eq, Debug, Serialize)]
pub enum InfectionStatusValue {
    Susceptible,
    Infectious,
    Recovered,
}

define_person_property_with_default!(
    InfectionStatus,
    InfectionStatusValue,
    InfectionStatusValue::Susceptible
);

define_global_property!(Params, Parameters);

pub struct Model {
    ctx: Context,
}

// TODO split up
define_rng!(ModelRng);

define_data_plugin!(ModelStatsPlugin, ModelStats, ModelStats::new());

#[derive(Serialize)]
pub struct Incidence {
    t: f64,
    status: InfectionStatusValue,
}

define_report!(Incidence);

trait InfectionLoop {
    fn get_params(&self) -> &Parameters;
    fn get_stats(&self) -> &ModelStats;
    fn infectious_people(&self) -> usize;
    fn infect_person(&mut self, p: PersonId, t: Option<f64>);
    fn recover_person(&mut self, p: PersonId, t: f64);
    fn next_event(&mut self);
    fn setup(&mut self);
}

impl InfectionLoop for Context {
    fn get_params(&self) -> &Parameters {
        self.get_global_property_value(Params).unwrap()
    }
    fn get_stats(&self) -> &ModelStats {
        self.get_data(ModelStatsPlugin)
    }
    fn infectious_people(&self) -> usize {
        self.query_people((InfectionStatus, InfectionStatusValue::Infectious))
            .len()
    }
    fn infect_person(&mut self, p: PersonId, t: Option<f64>) {
        if self.get_person_property(p, InfectionStatus) != InfectionStatusValue::Susceptible {
            return;
        }
        let enable_stats = self.get_params().enable_stats;
        self.set_person_property(p, InfectionStatus, InfectionStatusValue::Infectious);

        let stats_data = self.get_data_mut(ModelStatsPlugin);
        stats_data.record_infection();

        if let Some(t) = t {
            if enable_stats {
                self.send_report(Incidence {
                    t,
                    status: InfectionStatusValue::Infectious,
                });
            }
        }
    }
    fn recover_person(&mut self, p: PersonId, t: f64) {
        let enable_stats = self.get_params().enable_stats;
        self.set_person_property(p, InfectionStatus, InfectionStatusValue::Recovered);

        if enable_stats {
            self.send_report(Incidence {
                t,
                status: InfectionStatusValue::Recovered,
            });
        }
    }
    fn next_event(&mut self) {
        let params = self.get_params();
        let infection_rate = params.r0 / params.infectious_period;
        let n = self.infectious_people() as f64;

        // If there are no more infected people, exit the loop.
        if n == 0.0 {
            return;
        }

        let infection_event_rate = infection_rate * n;
        let recovery_event_rate = n / params.infectious_period;

        let infection_event_time =
            self.sample_distr(ModelRng, &Exp::new(infection_event_rate).unwrap());
        let recovery_event_time =
            self.sample_distr(ModelRng, &Exp::new(recovery_event_rate).unwrap());

        let p = self.sample_person(ModelRng, ()).unwrap();
        if infection_event_time < recovery_event_time {
            if self.get_person_property(p, InfectionStatus) == InfectionStatusValue::Susceptible {
                self.add_plan(
                    self.get_current_time() + infection_event_time,
                    move |context| {
                        context.infect_person(p, Some(context.get_current_time()));
                        if context.infectious_people() > 0 {
                            context.next_event();
                        }
                    },
                );
                return;
            }
        } else {
            self.add_plan(self.get_current_time() + recovery_event_time, |context| {
                if let Some(p) = context.sample_person(
                    ModelRng,
                    (InfectionStatus, InfectionStatusValue::Infectious),
                ) {
                    context.recover_person(p, context.get_current_time());
                }
                if context.infectious_people() > 0 {
                    context.next_event();
                }
            });
            return;
        }

        // If we didn't schedule any plans, retry.
        self.next_event();
    }
    fn setup(&mut self) {
        let &Parameters {
            population,
            initial_infections,
            seed,
            max_time,
            enable_stats,
            ..
        } = self.get_params();

        self.init_random(seed);
        self.index_property(InfectionStatus);

        self.report_options().overwrite(true);

        if enable_stats {
            self.add_report::<Incidence>("incidence-ixa").unwrap();
        }

        // Set up population
        for _ in 0..population {
            self.add_person(()).unwrap();
        }

        // Seed infections
        for p in self.sample_people(
            ModelRng,
            (InfectionStatus, InfectionStatusValue::Susceptible),
            initial_infections,
        ) {
            self.infect_person(p, None);
        }

        self.add_plan(max_time, |context| {
            context.shutdown();
        });

        assert_eq!(self.infectious_people(), 5);
    }
}

impl Model {
    pub fn new(params: Parameters) -> Self {
        let mut ctx = Context::new();
        ctx.set_global_property_value(Params, params).unwrap();
        Self { ctx }
    }
    pub fn get_stats(&self) -> &ModelStats {
        self.ctx.get_stats()
    }
    pub fn run(&mut self) {
        self.ctx.setup();
        // Set up the first event in the loop
        self.ctx.next_event();
        self.ctx.execute();
    }
}

impl SIRModel for Model {
    fn id(&self) -> &'static str {
        "ixa"
    }
    fn current_time(&self) -> f64 {
        self.ctx.get_current_time()
    }
    fn run(&mut self) {
        self.run();
    }
    fn get_stats(&self) -> &ModelStats {
        self.get_stats()
    }
    fn get_params(&self) -> &Parameters {
        self.ctx.get_params()
    }
}

#[cfg(test)]
mod test {
    use approx::assert_relative_eq;

    use super::*;

    #[test]
    fn infected_counts() {
        let mut model = Model::new(Parameters::default());
        model.ctx.setup();
        assert_eq!(model.ctx.infectious_people(), 5);
        let p = model
            .ctx
            .sample_person(
                ModelRng,
                (InfectionStatus, InfectionStatusValue::Susceptible),
            )
            .unwrap();
        model.ctx.infect_person(p, Some(0.0));
        assert_eq!(model.ctx.infectious_people(), 6);
        assert_eq!(model.ctx.get_stats().get_cum_incidence(), 1);
        model.ctx.recover_person(p, 0.0);
        assert_eq!(model.ctx.infectious_people(), 5);
        assert_eq!(model.ctx.get_stats().get_cum_incidence(), 1);
    }

    #[test]
    fn run_model() {
        let params = Parameters::default();
        let population = params.population;
        let mut model = Model::new(params);
        model.run();

        // Final size relation is ~58%
        let incidence = model.get_stats().get_cum_incidence() as f64;
        let expected = population as f64 * 0.58;
        assert_relative_eq!(incidence, expected, max_relative = 0.02);
    }
}
