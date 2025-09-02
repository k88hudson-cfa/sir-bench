use crate::{
    base::{Parameters, SIRModel},
    stats::ModelStats,
};
use indexmap::IndexSet;
use rand::{Rng, SeedableRng, rngs::SmallRng};
use rand_distr::Exp;

#[derive(Clone, Copy)]
pub enum InfectionStatus {
    Susceptible,
    Infectious,
    Recovered,
}

pub struct Context {
    parameters: Parameters,
    time: f64,
    rng: SmallRng,
    infection_status_lookup: Vec<InfectionStatus>,
    susceptible_people: IndexSet<PersonId>,
    infectious_people: IndexSet<PersonId>,
    recovered_people: IndexSet<PersonId>,
    population: usize,
    stats: ModelStats,
}

impl Context {
    pub fn new(parameters: Parameters) -> Context {
        let stats = ModelStats::new();
        Context {
            infection_status_lookup: Vec::new(),
            susceptible_people: IndexSet::new(),
            infectious_people: IndexSet::new(),
            recovered_people: IndexSet::new(),
            population: 0,
            rng: SmallRng::seed_from_u64(parameters.seed),
            parameters,
            time: 0.0,
            stats,
        }
    }

    fn add_person(&mut self, infection_status: InfectionStatus) -> PersonId {
        self.infection_status_lookup.push(infection_status);
        let person_id = PersonId {
            id: self.population,
        };
        self.population += 1;
        match infection_status {
            InfectionStatus::Susceptible => {
                self.susceptible_people.insert(person_id);
            }
            InfectionStatus::Infectious => {
                self.infectious_people.insert(person_id);
            }
            InfectionStatus::Recovered => {
                self.recovered_people.insert(person_id);
            }
        }
        person_id
    }

    fn get_infection_status(&self, person_id: PersonId) -> InfectionStatus {
        *self.infection_status_lookup.get(person_id.id).unwrap()
    }

    fn set_infection_status(&mut self, person_id: PersonId, infection_status: InfectionStatus) {
        match infection_status {
            InfectionStatus::Susceptible => {
                self.susceptible_people.insert(person_id);
            }
            InfectionStatus::Infectious => {
                self.susceptible_people.swap_remove(&person_id);
                self.infectious_people.insert(person_id);
            }
            InfectionStatus::Recovered => {
                self.infectious_people.swap_remove(&person_id);
                self.recovered_people.insert(person_id);
            }
        }
        *self.infection_status_lookup.get_mut(person_id.id).unwrap() = infection_status;
    }

    fn infect_person(&mut self, person_id: PersonId, _t: Option<f64>) {
        self.set_infection_status(person_id, InfectionStatus::Infectious);
        self.stats.record_infection();
    }

    fn sample_random_person(&mut self) -> PersonId {
        let index = self.rng.random_range(0..self.population);
        PersonId { id: index }
    }

    pub fn get_stats(&self) -> &ModelStats {
        &self.stats
    }

    pub fn run(&mut self) {
        // Set up population
        for _ in 0..self.parameters.population {
            self.add_person(InfectionStatus::Susceptible);
        }

        // Seed infections
        for _ in 0..self.parameters.initial_infections {
            let n_susceptible = self.susceptible_people.len();
            let index = self.rng.random_range(0..n_susceptible);
            let person_to_infect = *self.susceptible_people.get_index(index).unwrap();
            self.infect_person(person_to_infect, None);
        }

        // Start infection loop
        let infection_rate = self.parameters.r0 / self.parameters.infectious_period;
        let mut n_infectious = self.infectious_people.len();

        while n_infectious > 0 && self.time < self.parameters.max_time {
            let infection_event_rate = infection_rate * (n_infectious as f64);
            let recovery_event_rate = (n_infectious as f64) / self.parameters.infectious_period;

            let infection_event_time = self.rng.sample(Exp::new(infection_event_rate).unwrap());
            let recovery_event_time = self.rng.sample(Exp::new(recovery_event_rate).unwrap());

            if infection_event_time < recovery_event_time {
                let person_to_infect = self.sample_random_person();
                if let InfectionStatus::Susceptible = self.get_infection_status(person_to_infect) {
                    self.time += infection_event_time;
                    self.infect_person(person_to_infect, Some(self.time));
                }
            } else {
                let index = self.rng.random_range(0..n_infectious);
                let person_to_recover = *self.infectious_people.get_index(index).unwrap();
                self.set_infection_status(person_to_recover, InfectionStatus::Recovered);
                self.time += recovery_event_time;
            }

            n_infectious = self.infectious_people.len();
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct PersonId {
    id: usize,
}

impl SIRModel for Context {
    fn id(&self) -> &'static str {
        "baseline"
    }
    fn current_time(&self) -> f64 {
        self.time
    }
    fn run(&mut self) {
        self.run();
    }
    fn get_stats(&self) -> &ModelStats {
        self.get_stats()
    }
    fn get_params(&self) -> &Parameters {
        &self.parameters
    }
}

#[cfg(test)]
mod test {
    use approx::assert_relative_eq;

    use super::*;

    #[test]
    fn run_model() {
        let mut context = Context::new(Parameters {
            r0: 1.5,
            infectious_period: 3.0,
            population: 100_000,
            initial_infections: 5,
            seed: 8675308,
            max_time: 200.0,
            enable_stats: true,
        });
        context.run();

        // Final size relation is ~58%
        let incidence = context.get_stats().get_cum_incidence() as f64;
        let expected = context.parameters.population as f64 * 0.58;
        assert_relative_eq!(incidence, expected, max_relative = 0.02);
    }
}
