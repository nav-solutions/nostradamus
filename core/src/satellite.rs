use crate::{ClockState, Epoch, Frame, Orbit, State, EARTH_J2000};

use nyx_space::Spacecraft;

use gnss_rs::prelude::SV;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Satellite {
    /// True when current state is predicted
    predicted: bool,

    /// [Spacecraft] model from nyx.
    /// The orbital state contains the [Epoch] of observation/preduction.
    spacecraft: Spacecraft,
    
    /// [ClockState]
    pub clock: ClockState,
}

impl SatelliteState {
    // /// Copies and returns updated [SatelliteState] with new
    // /// coordinates
    // pub fn with_latlongalt_km(mut self, latlongalt_km: (f64, f64, f64)) -> Self {
    //     self.orbit = Orbit::from_latlongalt(latlongalt_km, self.epoch, self.frame);
    //     self
    // }
}

impl State for SatelliteState {
    type T = Self;

    fn default(epoch: Epoch) -> Self {
        Self {
            clock: ClockState::default(epoch),
            orbit: Orbit::from_position(0.0, 0.0, 0.0, epoch, EARTH_J2000),
        }
    }

    fn random(epoch: Epoch) -> Self {
        Self {
            clock: ClockState::random(epoch),
            orbit: Orbit::from_position(0.0, 0.0, 0.0, epoch, EARTH_J2000),
        }
    }

    fn predicted(&self) -> bool {
        self.predicted
    }

    

}
