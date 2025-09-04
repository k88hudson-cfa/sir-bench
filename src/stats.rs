pub struct ModelStats {
    cum_incidence: usize,
    prevalence: usize,
}

impl ModelStats {
    pub fn new(initial_infections: usize) -> Self {
        Self {
            cum_incidence: 0,
            prevalence: initial_infections,
        }
    }
    pub fn set_prevalence(&mut self, value: usize) {
        self.prevalence = value;
    }
    pub fn record_recovery(&mut self) {
        self.prevalence -= 1;
    }
    pub fn record_infection(&mut self) {
        self.cum_incidence += 1;
        self.prevalence += 1;
    }
    pub fn get_cum_incidence(&self) -> usize {
        self.cum_incidence
    }
    pub fn get_prevalence(&self) -> usize {
        self.prevalence
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_model_stats() {
        let mut stats = ModelStats::new(5);
        assert_eq!(stats.get_cum_incidence(), 0);
        assert_eq!(stats.get_prevalence(), 5);

        stats.record_infection();
        assert_eq!(stats.get_cum_incidence(), 1);
        assert_eq!(stats.get_prevalence(), 6);

        stats.record_recovery();
        assert_eq!(stats.get_cum_incidence(), 1);
        assert_eq!(stats.get_prevalence(), 5);
    }
}
