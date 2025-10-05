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
        let time = self.scenario.time_axis();

        if time.step.is_negative() {
            write!(
                f,
                "({}) elapsed={} (FORW={})",
                time.current,
                time.elapsed(),
                time.step,
            )
        } else {
            write!(
                f,
                "({}) elapsed={} (BACK={})",
                time.current,
                time.elapsed(),
                time.step,
            )
        }
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
