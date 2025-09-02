pub struct ModelStats {
    cum_incidence: usize,
}

impl ModelStats {
    pub fn new() -> Self {
        Self { cum_incidence: 0 }
    }
    pub fn record_infection(&mut self) {
        self.cum_incidence += 1;
    }
    pub fn get_cum_incidence(&self) -> usize {
        self.cum_incidence
    }
}
