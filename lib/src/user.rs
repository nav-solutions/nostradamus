use crate::{ClockState, Epoch, Orbit, State, EARTH_J2000};

use core::f64::consts::PI;

const PI_2: f64 = PI * 2.0;

/// [UserClock] model
#[derive(Debug, Copy, Clone, PartialEq)]
struct UserClock {
    /// Clock current phase
    phase: f64,

    /// Average clock frequency
    frequency: f64,
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct UserState {
    clock: ClockState,
    orbit: Orbit,
}

impl UserState {
    /// Expresses [UserState] at latitude, longitude degrees, and altitude kilometers.
    pub fn latlongalt_deg_km(&self) -> (f64, f64, f64) {
        self.orbit.latlongalt()
    }

    /// Copies and returns updated [UserState] with new latitude, longitude (degrees) and altitude
    /// (kilometers) state.
    pub fn with_latlongalt_deg_km(mut self, latlongalt_deg_km: (f64, f64, f64)) -> Self {
        self.orbit = Orbit::from_latlongalt(latlongalt_deg_km, self.orbit.epoch, self.orbit.frame);
        self
    }
}

impl State for UserState {
    fn default(epoch: Epoch) -> Self {
        Self {
            clock: ClockState {
                phase: Default::default(),
                frequency: 1E6,
            },
            orbit: Orbit::from_position(0.0, 0.0, 0.0, epoch, EARTH_J2000),
        }
    }

    fn random(epoch: Epoch) -> Self {
        Self {
            phase: Default::default(),
            frequency: 1E6,
            white_noise_psd: -120.0,
        }
    }

    fn epoch(&self) -> Epoch {
        self.orbit.epoch
    }

    fn set_epoch(&mut self, _: Epoch) {}

    fn temporal_update(&mut self, epoch: Epoch, state: &Self) {
        let dt = epoch - self.epoch();
        self.clock.phase += PI_2 * self.clock.frequency * dt;

        if self.clock.phase > PI_2 {
            self.clock.phase -= PI_2;
        }
    }
}
