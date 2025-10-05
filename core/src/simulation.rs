use hifitime::errors::HifitimeError;

use crate::prelude::{Almanac, Duration, Epoch, Scenario, State, TimeScale};

pub struct Simulation<S: Scenario> {
    /// [Almanac] configuration
    almanac: Almanac,

    /// Simulation [Scenario]
    pub scenario: S,
}

impl<S: Scenario> std::fmt::Display for Simulation<S> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "({}) elapsed={} (dt={})",
            self.scenario.epoch(),
            self.scenario.elapsed(),
            self.scenario.step()
        )
    }
}

impl<S: Scenario> Simulation<S> {
    /// Initiates a new [Simulation].
    ///
    /// ## Input
    /// - scenario: simulation [Scenario]
    /// - almanac: [Almanac] definition for this session
    pub fn new(scenario: S, almanac: Almanac) -> Self {
        Self { scenario, almanac }
    }
}
