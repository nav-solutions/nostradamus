use crate::prelude::{Clock, Epoch, Orbit, State, EARTH_J2000};

/// [GroundStation] model for static scenarios
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct GroundStation {
    /// State as [Orbit]
    state: Orbit,
    
    /// Clock
    clock: Clock,
}

/// [Rover] model for moving scenarios
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Rover {
    /// State as [Orbit]
    state: Orbit,

    /// Clock
    clock: Clock,
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
