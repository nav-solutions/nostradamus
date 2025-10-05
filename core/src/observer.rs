use crate::prelude::{Clock, Epoch, Orbit, State, EARTH_J2000};

/// Ground based [Observer], can be either moving (also refered to as "rover" in this case)
/// or static (also referred to as "ground-station" in this case).
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Observer {
    /// [Clock] state
    clock: Clock,

    /// [Orbit]al state.
    orbit: Orbit,
}

impl State for Observer {
    fn default(epoch: Epoch) -> Self {
        let (x_km, y_km, z_km) = (0.0, 0.0, 0.0); // TODO

        Self {
            clock: Clock::random(epoch),
            orbit: Orbit::from_position(x_km, y_km, z_km, epoch, EARTH_J2000),
        }
    }

    fn random(epoch: Epoch) -> Self {
        let (x_km, y_km, z_km) = (0.0, 0.0, 0.0); // TODO

        Self {
            clock: Clock::random(epoch),
            orbit: Orbit::from_position(x_km, y_km, z_km, epoch, EARTH_J2000),
        }
    }

    fn epoch(&self) -> Epoch {
        self.orbit.epoch
    }

    fn observed(mut self, state: Self) -> Self {
        self = state;
        self
    }
}

// impl UserState {
//     // /// Expresses [UserState] at latitude, longitude degrees, and altitude kilometers.
//     // pub fn latlongalt_deg_km(&self) -> (f64, f64, f64) {
//     //     self.orbit.latlongalt()
//     // }
//
//     // /// Copies and returns updated [UserState] with new latitude, longitude (degrees) and altitude
//     // /// (kilometers) state.
//     // pub fn with_latlongalt_deg_km(mut self, latlongalt_deg_km: (f64, f64, f64)) -> Self {
//     //     self.orbit = Orbit::from_latlongalt(latlongalt_deg_km, self.orbit.epoch, self.orbit.frame);
//     //     self
//     // }
// }
//
// impl State for UserState {
//     type T = Self;
//
//     fn default(epoch: Epoch) -> Self {
//         Self {
//             clock: ClockState::default(epoch),
//             orbit: Orbit::from_position(0.0, 0.0, 0.0, epoch, EARTH_J2000),
//         }
//     }
//
//     fn random(epoch: Epoch) -> Self {
//         Self {
//             clock: ClockState::default(epoch),
//             orbit: Orbit::from_position(0.0, 0.0, 0.0, epoch, EARTH_J2000),
//         }
//     }
//
//     fn temporal_update(&mut self, next: Self) {}
// }
