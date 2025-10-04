use crate::prelude::{Clock, Duration, Epoch, Orbit, State, EARTH_J2000};

use nyx_space::{md::prelude::GuidanceMode, Spacecraft};

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Satellite {
    /// True when current state is predicted
    predicted: bool,

    /// [Spacecraft] model from nyx.
    /// The orbital state contains the [Epoch] of observation/preduction.
    spacecraft: Spacecraft,

    /// Satellite on-board [Clock]
    clock: Clock,
}

impl Satellite {
    // /// Copies and returns updated [SatelliteState] with new
    // /// coordinates
    // pub fn with_latlongalt_km(mut self, latlongalt_km: (f64, f64, f64)) -> Self {
    //     self.orbit = Orbit::from_latlongalt(latlongalt_km, self.epoch, self.frame);
    //     self
    // }
}

impl State for Satellite {
    fn default(epoch: Epoch) -> Self {
        let orbit = Orbit::from_position(0.0, 0.0, 0.0, epoch, EARTH_J2000); // TODO
                                                                             // model:
                                                                             // Dry mass: 1_500 kg
                                                                             // SRP   Cr: 1.3
                                                                             // SRP area: 25 (m^2)
                                                                             // Drag  Cd: 2.2
                                                                             // DragArea: 25  (m^2)
        Self {
            predicted: false,
            clock: Clock::default(epoch),
            spacecraft: Spacecraft::builder()
                .orbit(orbit)
                .build()
                .with_dry_mass(1_500.0)
                .with_srp(25.0, 1.3)
                .with_drag(25.0, 2.0)
                .with_guidance_mode(GuidanceMode::Coast)
                .with_prop_mass(0.0),
        }
    }

    fn random(epoch: Epoch) -> Self {
        let orbit = Orbit::from_position(0.0, 0.0, 0.0, epoch, EARTH_J2000); // TODO
                                                                             // model:
                                                                             // Dry mass: 1_500 kg
                                                                             // SRP   Cr: 1.3
                                                                             // SRP area: 25 (m^2)
                                                                             // Drag  Cd: 2.2
                                                                             // DragArea: 25  (m^2)
        Self {
            predicted: false,
            clock: Clock::random(epoch),
            spacecraft: Spacecraft::builder()
                .orbit(orbit)
                .build()
                .with_dry_mass(1_500.0)
                .with_srp(25.0, 1.3)
                .with_drag(25.0, 2.0)
                .with_guidance_mode(GuidanceMode::Coast)
                .with_prop_mass(0.0),
        }
    }

    fn predicted(&self) -> bool {
        self.predicted
    }

    fn predict(mut self, step: Duration) -> Self {
        self.predicted = true;
        self
    }

    fn observe(&mut self, state: Self) {}
}
