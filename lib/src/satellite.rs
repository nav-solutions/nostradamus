use crate::{ClockState, Epoch, Frame, Orbit, Simulation, State, EARTH_J2000};

use gnss_rs::prelude::SV;

#[derive(Debug, Default, Copy, Clone, PartialEq)]
pub struct SatelliteState {
    /// [Orbit]al state
    orbit: Orbit,

    /// [ClockState]
    clock: ClockState,
}

impl SatelliteState {
    /// Copies and returns updated [SatelliteState] with new
    /// coordinates
    pub fn with_latlongalt_km(mut self, latlongalt_km: (f64, f64, f64)) -> Self {
        self.orbit = Orbit::from_latlongalt(latlongalt_km, self.epoch, self.frame);
        self
    }
}

impl State for SatelliteState {
    fn default(epoch: Epoch) -> Self {
        Self {
            clock: ClockState::default(),
            orbit: Orbit::from_position(0.0, 0.0, 0.0, epoch, EARTH_J2000),
        }
    }

    fn random(epoch: Epoch) -> Self {
        Self {
            clock: ClockState::random(),
            orbit: Orbit::from_position(0.0, 0.0, 0.0, epoch, EARTH_J2000),
        }
    }

    fn temporal_update(&mut self, state: &Self) {
        self.clock = state.clock;
        self.orbit = state.orbit;
    }

    fn temporally_updated(mut self, state: Self) -> Self {
        self.clock = state.clock;
        self.orbit = state.orbit;
        self
    }
}
